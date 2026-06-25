use anyhow::bail;
use reikura_gfx::backend::BlitParam;

use crate::instruction::{Evaluate, Instruction, ReadParam, Value};

const CMD_BLIT_COPY: u8 = 0;
const CMD_BLIT_BLEND: u8 = 1;
const CMD_BLIT_FADE: u8 = 19;

pub struct Gp;

impl Instruction for Gp {
    fn execute(vm: &mut crate::Vm, _info: super::InstructionInfo) -> anyhow::Result<()> {
        let cmd: u8 = vm.scene.param()?;

        let param = BlitParam {
            src_id: vm.scene.param::<Value>()?.evaluate(&vm.ctx).try_into()?,
            src_x: vm.scene.param::<Value>()?.evaluate(&vm.ctx) as u32,
            src_y: vm.scene.param::<Value>()?.evaluate(&vm.ctx) as u32,
            width: vm.scene.param::<Value>()?.evaluate(&vm.ctx) as u32,
            height: vm.scene.param::<Value>()?.evaluate(&vm.ctx) as u32,
            dst_id: vm.scene.param::<Value>()?.evaluate(&vm.ctx).try_into()?,
            dst_x: vm.scene.param::<Value>()?.evaluate(&vm.ctx) as u32,
            dst_y: vm.scene.param::<Value>()?.evaluate(&vm.ctx) as u32,
        };

        match cmd {
            CMD_BLIT_COPY => vm.gfx.blit_copy_image(param)?,
            CMD_BLIT_BLEND => vm.gfx.blit_blend_image(param)?,
            CMD_BLIT_FADE => vm.gfx.blit_blend_image(param)?, //TODO
            _ => bail!("invalid or unsupported Gp cmd: {cmd}"),
        }

        Ok(())
    }
}
