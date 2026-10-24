use std::time::{Duration, Instant};

use anyhow::Context;
use reikura_gfx::GraphicEngine;
use reikura_util::{flag::Flag, register::Register};

use crate::{
    AssetManager, AssetName, AudioManager, InputManager, Manifest, Parser, SaveContext, UserSetup,
    instruction::INSTRUCTIONS,
};

pub const FRAME_INTERVAL: Duration = Duration::from_millis(16);

pub struct VmContext {
    pub flags: Flag,
    pub registers: Register,
    pub wait_duration: Option<Duration>,
    pub timer: Option<Timer>,
    // TODO: this make much more sense stored in the text renderer struct
    pub char_names: [Option<String>; 256],
}

impl VmContext {
    pub fn new() -> Self {
        Self {
            flags: Flag::new(2048),
            registers: Register::new(2048),
            wait_duration: None,
            timer: None,
            char_names: [const { None }; 256],
        }
    }
}

pub struct Vm {
    pub last_update: Option<Instant>,
    pub manifest: Manifest,
    pub assets: AssetManager,
    pub parser: Parser,
    pub audio: AudioManager,
    pub gfx: GraphicEngine,
    pub ctx: VmContext,
    pub save: Option<SaveContext>,
    pub user_setup: UserSetup,
    pub input: InputManager,
    pub state: State,
}

impl Vm {
    pub fn new(manifest: Manifest, gfx: GraphicEngine) -> anyhow::Result<Self> {
        let mut assets = AssetManager::new(&manifest)?;
        let input = InputManager::new(manifest.view_size);
        let config_path = manifest.game_path().join("user_setup");
        let start_scene = assets
            .load_scene(AssetName::START)
            .context("failed to load start script")?;

        Ok(Self {
            last_update: None,
            manifest,
            assets,
            parser: Parser::new(start_scene),
            audio: AudioManager::new(1.0, 1.0, 1.0)?,
            gfx,
            ctx: VmContext::new(),
            save: None,
            user_setup: UserSetup::open(config_path)?,
            input,
            state: State::Running,
        })
    }

    pub fn update(&mut self) -> anyhow::Result<()> {
        let now = Instant::now();
        let _dt = self.last_update.map(|last| now.duration_since(last));
        self.last_update = Some(now);

        match self.state {
            State::Exit => todo!(),
            State::Running => self.tick()?,
            State::Wait { start, duration } => {
                let elapsed = start.elapsed();

                if elapsed > duration {
                    self.state.run();
                    return Ok(());
                }

                let time_remaining = duration.saturating_sub(elapsed);
                let sleep_duration = FRAME_INTERVAL.min(time_remaining);
                std::thread::sleep(sleep_duration);
            }
            State::WaitClick => todo!(),
            State::WaitText => todo!(),
            State::WaitTransition => todo!(),
            State::WaitVoice => {
                if self.audio.track.voice.is_audio_finished() {
                    self.state.run();
                }
            }
            State::WaitVideo => todo!(),
        }

        Ok(())
    }

    pub fn tick(&mut self) -> anyhow::Result<()> {
        let ip = self.parser.state.ip;
        let op = self.parser.read_opcode()?;
        let inst = INSTRUCTIONS[op as usize];

        inst.execute(self, ip)?;

        Ok(())
    }
}

pub struct Timer {
    start: Instant,
    duration: i32,
}

impl Timer {
    pub fn new(duration: i32) -> Self {
        Self {
            start: Instant::now(),
            duration,
        }
    }

    pub fn get(&self) -> i32 {
        let elapsed = self.start.elapsed().as_millis() as i32;
        // NOTE: not sure if saturating is accurate here
        elapsed.saturating_sub(self.duration)
    }
}

pub enum State {
    Exit,
    Running,
    Wait { start: Instant, duration: Duration },
    WaitClick,
    WaitText,
    WaitTransition,
    WaitVoice,
    WaitVideo,
}

impl State {
    pub fn exit(&mut self) {
        *self = Self::Exit;
    }

    pub fn run(&mut self) {
        *self = Self::Running;
    }

    pub fn wait(&mut self, duration: Duration) {
        *self = Self::Wait {
            start: Instant::now(),
            duration,
        }
    }

    pub fn wait_click(&mut self) {
        *self = Self::WaitClick;
    }

    pub fn wait_text(&mut self) {
        *self = Self::WaitText;
    }

    pub fn wait_voice(&mut self) {
        *self = Self::WaitVoice;
    }

    pub fn wait_video(&mut self) {
        *self = Self::WaitVideo;
    }
}
