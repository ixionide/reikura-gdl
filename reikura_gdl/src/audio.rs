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

pub const SFX_SLOT: usize = 32;

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
    pub volume: Option<f32>,
}

impl Audio {
    pub fn load(name: String, data: Vec<u8>) -> Result<Self> {
        Ok(Self {
            name: name.into(),
            data: data.into(),
            volume: None,
        })
    }

    pub fn is_midi(&self) -> bool {
        self.data.starts_with(MIDI_SIGNATURE)
    }
}

pub struct AudioManager {
    _manager: kira::AudioManager,
    pub track: AllTrack,
    pub bgm: Option<Audio>,
    pub sfx: [Option<Audio>; SFX_SLOT],
    pub voice: Option<Audio>,
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
    pub fn play_bgm(&mut self, looping: bool, fade_duration: Option<Duration>) -> Result<()> {
        let Some(audio) = self.bgm.take() else {
            bail!("no bgm loaded");
        };

        // TODO: handle midi bgm
        let mut data = StreamingSoundData::from_cursor(Cursor::new(audio.data))?;

        if let Some(volume) = audio.volume {
            data = data.volume(volume);
        }

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

    pub fn play_sfx(&mut self, slot: usize, fade_duration: Option<Duration>) -> Result<()> {
        let Some(audio) = self.sfx[slot].take() else {
            bail!("no sfx loaded at slot: {slot}");
        };

        let mut data = StreamingSoundData::from_cursor(Cursor::new(audio.data))?;

        if let Some(fade_tween) = fade_duration.map(tween_duration) {
            data = data.fade_in_tween(fade_tween);
        }

        self.track.sfx.play_audio_at_slot(slot, data)?;

        Ok(())
    }

    pub fn stop_sfx(&mut self, slot: usize, fade_duration: Option<Duration>) {
        self.track
            .sfx
            .stop_audio_at_slot(slot, fade_duration.map(tween_duration));
    }

    pub fn play_voice(&mut self, fade_duration: Option<Duration>) -> Result<()> {
        let Some(audio) = self.voice.take() else {
            bail!("no voice loaded");
        };

        let mut data = StreamingSoundData::from_cursor(Cursor::new(audio.data))?;

        if let Some(volume) = audio.volume {
            data = data.volume(volume);
        }

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
        let bgm = Track::new(kira_manager, &volume.bgm)?;
        let sfx = Track::new(kira_manager, &volume.sfx)?;
        let voice = Track::new(kira_manager, &volume.voice)?;

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
    handle_slots: [Option<SoundHandle>; SLOT],
}

impl<const SLOT: usize> Track<SLOT> {
    pub fn new(kira_manager: &mut kira::AudioManager, volume: &Volume) -> Result<Self> {
        let handle = kira_manager.add_sub_track(TrackBuilder::new().volume(volume.modulator()))?;
        Ok(Self {
            track_handle: handle,
            handle_slots: [const { None }; SLOT],
        })
    }

    pub fn play_audio(&mut self, sound_data: SoundData) -> Result<()> {
        self.play_audio_at_slot(0, sound_data)
    }

    pub fn play_audio_at_slot(&mut self, slot: usize, sound_data: SoundData) -> Result<()> {
        let index = slot % SLOT;

        if let Some(old_handle) = &mut self.handle_slots[index] {
            old_handle.stop(DEFAULT_TWEEN);
        }

        let new_handle = self.track_handle.play(sound_data)?;
        self.handle_slots[index] = Some(new_handle);

        Ok(())
    }

    pub fn stop_audio(&mut self, fade_duration: Option<Tween>) {
        self.stop_audio_at_slot(0, fade_duration);
    }

    pub fn stop_audio_at_slot(&mut self, slot: usize, fade_duration: Option<Tween>) {
        let index = slot % SLOT;

        if let Some(mut handle) = self.handle_slots[index].take() {
            handle.stop(fade_duration.unwrap_or(DEFAULT_TWEEN));
        }
    }

    pub fn is_audio_finished(&self) -> bool {
        self.is_audio_at_slot_finished(0)
    }

    pub fn is_audio_at_slot_finished(&self, slot: usize) -> bool {
        let index = slot % SLOT;

        self.handle_slots[index]
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
    pub sfx: Volume,
    pub voice: Volume,
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
            sfx: Volume::new(kira_manager, sfx)?,
            voice: Volume::new(kira_manager, voice)?,
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
