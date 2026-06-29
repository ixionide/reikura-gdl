use std::time::Duration;

use anyhow::bail;
use reikura_gfx::backend::BlitParam;

use crate::instruction::{Evaluate, Instruction, ReadParam, Rect, Value};

const CMD_BLIT_COPY: u8 = 0;
const CMD_BLIT_BLEND: u8 = 1;
const CMD_BLIT_FADE: u8 = 19;

pub struct Gp;

impl Instruction for Gp {
    fn execute(vm: &mut crate::Vm, _info: super::InstructionInfo) -> anyhow::Result<()> {
        let cmd: u8 = vm.scene.param()?;

        let src_id = vm.scene.param::<Value>()?.evaluate(&vm.ctx).try_into()?;
        let rect: [_; _] = vm.scene.param::<Rect<Value>>()?.evaluate(&vm.ctx).into();
        let [src_x, src_y, width, height] = rect.map(|it| it as u32);
        let dst_id = vm.scene.param::<Value>()?.evaluate(&vm.ctx);
        let dst_x = vm.scene.param::<Value>()?.evaluate(&vm.ctx) as u32;
        let dst_y = vm.scene.param::<Value>()?.evaluate(&vm.ctx) as u32;

        let mut param = BlitParam {
            src_id,
            dst_id: 0,
            src_x,
            src_y,
            width,
            height,
            dst_x,
            dst_y,
        };

        match cmd {
            CMD_BLIT_COPY => {
                param.dst_id = dst_id as u8;
                vm.gfx.blit_copy_image(param)?;
            }
            CMD_BLIT_BLEND => {
                param.dst_id = dst_id as u8;
                vm.gfx.blit_blend_image(param)?;
            }
            CMD_BLIT_FADE => {
                let _fade_duration = Duration::from_millis(dst_id as u64);
                vm.gfx.blit_blend_image(param)?;
            }
            // TODO
            _ => bail!("invalid or unsupported GP cmd: {cmd}"),
        }

        Ok(())
    }
}
