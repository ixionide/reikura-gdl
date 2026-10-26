use std::{io::Cursor, rc::Rc, sync::Arc, time::Duration};

use anyhow::{Result, bail};
use kira::{
    Decibels, Easing, Mapping, StartTime, Tween, Value,
    modulator::tweener::{TweenerBuilder, TweenerHandle},
    sound::{
        FromFileError,
        streaming::{StreamingSoundData, StreamingSoundHandle},
    },
    track::{TrackBuilder, TrackHandle},
};

const SFX_SLOT: usize = 32;

const MIDI_SIGNATURE: &[u8] = b"MThd";

const DEFAULT_TWEEN: Tween = Tween {
    start_time: StartTime::Immediate,
    duration: Duration::from_millis(10),
    easing: Easing::Linear,
};

fn tween_duration(duration: Duration) -> Tween {
    Tween {
        duration,
        ..DEFAULT_TWEEN
    }
}

#[derive(Clone)]
pub struct Audio {
    pub name: Rc<str>,
    pub data: Arc<[u8]>,
}

impl Audio {
    pub fn load(name: String, data: Vec<u8>) -> Result<Self> {
        Ok(Self {
            name: name.into(),
            data: data.into(),
        })
    }

    pub fn is_midi(&self) -> bool {
        self.data.starts_with(MIDI_SIGNATURE)
    }
}

pub struct AudioManager {
    _manager: kira::AudioManager,
    pub track: AllTrack,
    bgm: Option<Audio>,
    sfx: [Option<Audio>; SFX_SLOT],
    voice: Option<Audio>,
}

impl AudioManager {
    pub fn new(bgm_vol: f64, sfx_vol: f64, voice_vol: f64) -> Result<Self> {
        let mut kira_manager = kira::AudioManager::new(Default::default())?;
        let volume = AllVolume::new(&mut kira_manager, bgm_vol, sfx_vol, voice_vol)?;
        let track = AllTrack::new(&mut kira_manager, volume)?;

        Ok(Self {
            _manager: kira_manager,
            track,
            bgm: None,
            sfx: [const { None }; SFX_SLOT],
            voice: None,
        })
    }
}

impl AudioManager {
    pub fn set_bgm(&mut self, audio: Audio) {
        self.stop_bgm(None);
        self.bgm = Some(audio);
    }

    pub fn play_bgm(&mut self, looping: bool, fade_duration: Option<Duration>) -> Result<()> {
        let Some(audio) = self.bgm.take() else {
            bail!("no bgm loaded");
        };

        // TODO: handle midi bgm
        let mut data = StreamingSoundData::from_cursor(Cursor::new(audio.data))?;

        if looping {
            data = data.loop_region(..);
        }

        if let Some(fade_tween) = fade_duration.map(tween_duration) {
            data = data.fade_in_tween(fade_tween);
        }

        self.track.bgm.play_audio(data)?;

        Ok(())
    }

    pub fn stop_bgm(&mut self, fade_duration: Option<Duration>) {
        self.track.bgm.stop_audio(fade_duration.map(tween_duration));
    }

    pub fn set_sfx(&mut self, slot: usize, audio: Audio) {
        let slot = slot % SFX_SLOT;

        self.stop_sfx(slot, None);
        self.sfx[slot] = Some(audio);
    }

    pub fn play_sfx(&mut self, slot: usize, fade_duration: Option<Duration>) -> Result<()> {
        let slot = slot % SFX_SLOT;

        let Some(audio) = self.sfx[slot].take() else {
            bail!("no sfx loaded at slot: {slot}");
        };

        let mut data = StreamingSoundData::from_cursor(Cursor::new(audio.data))?;

        if let Some(fade_tween) = fade_duration.map(tween_duration) {
            data = data.fade_in_tween(fade_tween);
        }

        self.track.sfx.play_audio_slot(slot, data)?;

        Ok(())
    }

    pub fn stop_sfx(&mut self, slot: usize, fade_duration: Option<Duration>) {
        let slot = slot % SFX_SLOT;

        self.track
            .sfx
            .stop_audio_slot(slot, fade_duration.map(tween_duration));
    }

    pub fn set_voice(&mut self, audio: Audio) {
        self.stop_voice(None);
        self.voice = Some(audio);
    }

    pub fn play_voice(&mut self, fade_duration: Option<Duration>) -> Result<()> {
        let Some(audio) = self.voice.take() else {
            bail!("no voice loaded");
        };

        let mut data = StreamingSoundData::from_cursor(Cursor::new(audio.data))?;

        if let Some(fade_tween) = fade_duration.map(tween_duration) {
            data = data.fade_in_tween(fade_tween);
        }

        self.track.voice.play_audio(data)?;

        Ok(())
    }

    pub fn stop_voice(&mut self, fade_duration: Option<Duration>) {
        self.track
            .voice
            .stop_audio(fade_duration.map(tween_duration));
    }
}

pub struct AllTrack {
    pub bgm: Track<1>,
    pub sfx: Track<SFX_SLOT>,
    pub voice: Track<1>,
    pub volume: AllVolume,
}

impl AllTrack {
    pub fn new(kira_manager: &mut kira::AudioManager, volume: AllVolume) -> Result<Self> {
        let bgm = Track::new(kira_manager, &volume.bgm, Some(&volume.bgm_inside))?;
        let sfx = Track::new(kira_manager, &volume.sfx, None)?;
        let voice = Track::new(kira_manager, &volume.voice, Some(&volume.voice_inside))?;

        Ok(Self {
            bgm,
            sfx,
            voice,
            volume,
        })
    }
}

type SoundData = StreamingSoundData<FromFileError>;
type SoundHandle = StreamingSoundHandle<FromFileError>;

pub struct Track<const SLOT: usize> {
    track_handle: TrackHandle,
    _inside_handle: Option<TrackHandle>,
    handle_slots: [Option<SoundHandle>; SLOT],
}

impl Track<1> {
    pub fn play_audio(&mut self, sound_data: SoundData) -> Result<()> {
        self.play_audio_slot(0, sound_data)
    }

    pub fn stop_audio(&mut self, fade_duration: Option<Tween>) {
        self.stop_audio_slot(0, fade_duration);
    }

    pub fn is_audio_finished(&self) -> bool {
        self.is_audio_slot_finished(0)
    }
}

impl<const SLOT: usize> Track<SLOT> {
    pub fn new(
        kira_manager: &mut kira::AudioManager,
        volume: &Volume,
        inside_volume: Option<&Volume>,
    ) -> Result<Self> {
        let mut handle =
            kira_manager.add_sub_track(TrackBuilder::new().volume(volume.modulator()))?;
        let inside_handle = inside_volume
            .map(|vol| handle.add_sub_track(TrackBuilder::new().volume(vol.modulator())))
            .transpose()?;

        Ok(Self {
            track_handle: handle,
            _inside_handle: inside_handle,
            handle_slots: [const { None }; SLOT],
        })
    }

    pub fn play_audio_slot(&mut self, slot: usize, sound_data: SoundData) -> Result<()> {
        let new_handle = self.track_handle.play(sound_data)?;
        self.handle_slots[slot] = Some(new_handle);

        Ok(())
    }

    pub fn stop_audio_slot(&mut self, slot: usize, fade_duration: Option<Tween>) {
        if let Some(mut handle) = self.handle_slots[slot].take() {
            handle.stop(fade_duration.unwrap_or(DEFAULT_TWEEN));
        }
    }

    pub fn is_audio_slot_finished(&self, slot: usize) -> bool {
        self.handle_slots[slot]
            .as_ref()
            .map(SoundHandle::state)
            .is_none_or(|state| !state.is_advancing())
    }

    pub fn pause_track(&mut self) {
        self.track_handle.pause(DEFAULT_TWEEN);
    }

    pub fn resume_track(&mut self) {
        self.track_handle.resume(DEFAULT_TWEEN);
    }
}

pub struct AllVolume {
    pub bgm: Volume,
    pub bgm_inside: Volume,
    pub sfx: Volume,
    pub voice: Volume,
    pub voice_inside: Volume,
}

impl AllVolume {
    pub fn new(
        kira_manager: &mut kira::AudioManager,
        bgm: f64,
        sfx: f64,
        voice: f64,
    ) -> Result<Self> {
        Ok(Self {
            bgm: Volume::new(kira_manager, bgm)?,
            bgm_inside: Volume::new(kira_manager, 1.0)?,
            sfx: Volume::new(kira_manager, sfx)?,
            voice: Volume::new(kira_manager, voice)?,
            voice_inside: Volume::new(kira_manager, 1.0)?,
        })
    }
}

pub struct Volume(TweenerHandle);

impl Volume {
    pub const MAPPING: Mapping<Decibels> = Mapping {
        input_range: (0.0, 1.0),
        output_range: (Decibels::SILENCE, Decibels::IDENTITY),
        easing: Easing::Linear,
    };

    pub fn new(kira_manager: &mut kira::AudioManager, initial: f64) -> Result<Self> {
        let initial_value = initial.clamp(0.0, 1.0);
        let handle = kira_manager.add_modulator(TweenerBuilder { initial_value })?;
        Ok(Self(handle))
    }

    pub fn modulator(&self) -> Value<Decibels> {
        Value::FromModulator {
            id: self.0.id(),
            mapping: Volume::MAPPING,
        }
    }

    pub fn set(&mut self, volume: f64) {
        self.0.set(volume.clamp(0.0, 1.0), DEFAULT_TWEEN);
    }
}
