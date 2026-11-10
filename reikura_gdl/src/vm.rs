use std::{collections::HashMap, time::Instant};

use anyhow::Context;
use reikura_util::{
    flag::{Flag, FlagGroup},
    register::Register,
};

use crate::{
    AssetManager, AssetName, AudioManager, InputManager, Manifest, Parser, Runtime, SaveContext,
    State, UserSetup, graphic::Graphics, instruction::INSTRUCTION_SET,
};

pub const FRAME_INTERVAL_MS: u32 = 17;

pub struct VmContext {
    pub flags: Flag,
    pub flag_groups: HashMap<u8, Vec<FlagGroup>>,
    pub registers: Register,

    // TODO: move this out of this struct
    pub char_names: [Option<String>; 256],
}

impl VmContext {
    pub fn new() -> Self {
        Self {
            flags: Flag::new(2048),
            flag_groups: HashMap::with_capacity(100),
            registers: Register::new(2048),
            char_names: [const { None }; 256],
        }
    }
}

pub struct Vm {
    pub manifest: Manifest,
    pub assets: AssetManager,
    pub parser: Parser,
    pub audio: AudioManager,
    pub gfx: Graphics,
    pub ctx: VmContext,
    pub save: SaveContext,
    pub user_setup: UserSetup,
    pub input: InputManager,
    pub runtime: Runtime,
}

impl Vm {
    pub fn new(manifest: Manifest) -> anyhow::Result<Self> {
        let (view_w, view_h) = manifest.view_size;
        let save_path = manifest.game_path().join("saves");
        let user_setup_path = manifest.game_path().join("user_setup");

        let mut assets = AssetManager::new(&manifest)?;
        let start_scene = assets
            .load_scene(AssetName::START)
            .context("failed to load start scenario")?;

        Ok(Self {
            manifest,
            assets,
            parser: Parser::new(start_scene),
            audio: AudioManager::new(1.0, 1.0, 1.0)?,
            gfx: Graphics::new(view_w, view_h),
            ctx: VmContext::new(),
            save: SaveContext::new(save_path),
            user_setup: UserSetup::open(user_setup_path)?,
            input: InputManager::new(),
            runtime: Runtime::new(),
        })
    }

    fn should_tick(&mut self) -> bool {
        let tick = match self.runtime.state {
            State::Exit => false,
            State::Run => true,
            State::Wait => self.runtime.is_wait_finished(),
            State::WaitClick => todo!(),
            State::WaitText => todo!(),
            State::WaitTransition => todo!(),
            State::WaitVoice => self.audio.track.voice.is_finished(),
            State::WaitVideo => todo!(),
        };

        if tick {
            self.runtime.state.run()
        }

        tick
    }

    fn tick(&mut self) -> anyhow::Result<()> {
        let ip = self.parser.state.ip;
        let op = self.parser.read_opcode()?;
        let inst = INSTRUCTION_SET[op as usize];

        inst.execute(self, ip)?;

        Ok(())
    }

    pub fn update(&mut self) -> anyhow::Result<()> {
        let now = Instant::now();
        let frame_time =
            (now - self.runtime.current_time).as_millis() as u32 + self.runtime.accumulator;
        self.runtime.current_time = now;

        if frame_time >= FRAME_INTERVAL_MS {
            let advance = frame_time / FRAME_INTERVAL_MS;
            self.gfx.update(advance);
            self.runtime.accumulator = frame_time % FRAME_INTERVAL_MS;
        } else {
            self.runtime.accumulator = frame_time;
        }

        if self.should_tick() {
            self.tick()?;
        }

        Ok(())
    }
}
