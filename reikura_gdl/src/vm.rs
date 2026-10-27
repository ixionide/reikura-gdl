use std::time::{Duration, Instant};

use anyhow::Context;
use reikura_util::{flag::Flag, register::Register};

use crate::{
    AssetManager, AssetName, AudioManager, InputManager, Manifest, Parser, SaveContext, UserSetup,
    graphic::Graphics, instruction::INSTRUCTIONS,
};

pub const _FRAME_INTERVAL: Duration = Duration::from_millis(17);

pub struct VmContext {
    pub flags: Flag,
    pub registers: Register,
    pub timer: Option<Timer>,
    // TODO: this make much more sense stored in the text renderer struct
    pub char_names: [Option<String>; 256],
}

impl VmContext {
    pub fn new() -> Self {
        Self {
            flags: Flag::new(2048),
            registers: Register::new(2048),
            timer: None,
            char_names: [const { None }; 256],
        }
    }
}

pub struct Vm {
    last_update: Instant,
    pub manifest: Manifest,
    pub assets: AssetManager,
    pub parser: Parser,
    pub audio: AudioManager,
    pub gfx: Graphics,
    pub ctx: VmContext,
    pub save: SaveContext,
    pub user_setup: UserSetup,
    pub input: InputManager,
    pub state: State,
    pub wait_until: Option<Instant>,
}

impl Vm {
    pub fn new(manifest: Manifest) -> anyhow::Result<Self> {
        let now = Instant::now();
        let (view_w, view_h) = manifest.view_size;
        let save_path = manifest.game_path().join("saves");
        let user_setup_path = manifest.game_path().join("user_setup");

        let mut assets = AssetManager::new(&manifest)?;
        let start_scene = assets
            .load_scene(AssetName::START)
            .context("failed to load start scenario")?;

        Ok(Self {
            last_update: now,
            manifest,
            assets,
            parser: Parser::new(start_scene),
            audio: AudioManager::new(1.0, 1.0, 1.0)?,
            gfx: Graphics::new(view_w, view_h),
            ctx: VmContext::new(),
            save: SaveContext::new(save_path),
            user_setup: UserSetup::open(user_setup_path)?,
            input: InputManager::new(),
            state: State::Run,
            wait_until: None,
        })
    }

    fn should_tick(&mut self, now: Instant) -> bool {
        let tick = match self.state {
            State::Exit => false,
            State::Run => true,
            State::Wait => self.wait_until.is_none_or(|until| now > until),
            State::WaitClick => todo!(),
            State::WaitText => todo!(),
            State::WaitTransition => todo!(),
            State::WaitVoice => self.audio.track.voice.is_audio_finished(),
            State::WaitVideo => todo!(),
        };

        if tick {
            self.state.run()
        }

        tick
    }

    fn tick(&mut self) -> anyhow::Result<()> {
        let ip = self.parser.state.ip;
        let op = self.parser.read_opcode()?;
        let inst = INSTRUCTIONS[op as usize];

        inst.execute(self, ip)?;

        Ok(())
    }

    pub fn update(&mut self) -> anyhow::Result<()> {
        let now = Instant::now();
        let _dt = now - self.last_update;
        self.last_update = now;

        if self.should_tick(now) {
            self.tick()?;
        }

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
    Run,
    Wait,
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
        *self = Self::Run;
    }

    pub fn wait(&mut self) {
        *self = Self::Wait;
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
