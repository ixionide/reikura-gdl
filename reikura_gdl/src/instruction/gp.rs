use std::time::Duration;

use anyhow::bail;
use reikura_gfx::backend::BlitParam;

use crate::instruction::{Instruction, Rect, Value};

reikura_util::const_iota! {
    u8 = iota,
    BLIT_COPY,
    BLIT_BLEND,
}

pub struct Gp;

impl Instruction for Gp {
    fn execute(vm: &mut crate::Vm, _info: super::InstructionInfo) -> anyhow::Result<()> {
        let cmd: u8 = vm.parser.read_param()?;

        let src_id = vm
            .parser
            .read_param::<Value>()?
            .evaluate(&vm.ctx)
            .try_into()?;
        let rect: [_; _] = vm
            .parser
            .read_param::<Rect<Value>>()?
            .evaluate(&vm.ctx)
            .into();
        let [src_x, src_y, width, height] = rect.map(|it| it as u32);
        let dst_id = vm.parser.read_param::<Value>()?.evaluate(&vm.ctx);
        let dst_x = vm.parser.read_param::<Value>()?.evaluate(&vm.ctx) as u32;
        let dst_y = vm.parser.read_param::<Value>()?.evaluate(&vm.ctx) as u32;

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
            BLIT_COPY => {
                param.dst_id = dst_id as u8;
                vm.gfx.blit_copy_image(param)?;
            }
            BLIT_BLEND => {
                param.dst_id = dst_id as u8;
                vm.gfx.blit_blend_image(param)?;
            }
            // TODO: another gp cmd
            _ if cmd < 30 => {
                let _duration = Duration::from_millis(dst_id as u64);
                vm.gfx.blit_blend_image(param)?;
            }
            _ => bail!("invalid or unsupported GP cmd: {cmd}"),
        }

        Ok(())
    }
}
