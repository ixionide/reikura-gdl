use std::time::Duration;

use anyhow::bail;

use crate::{
    Vm,
    instruction::{InstructionInfo, Rect, Value},
};

reikura_util::const_iota! {
    u8 = iota,
    BLIT_COPY,
    BLIT_BLEND,
}

#[allow(unused)]
pub fn gp(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    let (ctx, parser) = (&vm.ctx, &mut vm.parser);
    let cmd: u8 = parser.read_param()?;

    let src_id = parser.read_param::<Value>()?.eval(ctx);
    let src_rect = parser.read_param::<Rect<Value>>()?.eval(ctx);

    let dst_id = parser.read_param::<Value>()?.eval(ctx);
    let dst_x = parser.read_param::<Value>()?.eval(ctx);
    let dst_y = parser.read_param::<Value>()?.eval(ctx);

    let src_id = u8::try_from(src_id)?;
    match cmd {
        BLIT_COPY => {
            let dst_id = Some(u8::try_from(dst_id)?);

            vm.gfx.blit_copy_image(
                src_id, dst_id, src_rect.x, src_rect.y, src_rect.w, src_rect.h, dst_x, dst_y,
            )?;
        }
        BLIT_BLEND => {
            let dst_id = Some(u8::try_from(dst_id)?);

            vm.gfx.blit_blend_image(
                src_id, dst_id, src_rect.x, src_rect.y, src_rect.w, src_rect.h, dst_x, dst_y,
            )?;
        }
        19 => {
            let _duration = Duration::from_millis(dst_id as u64);

            // TODO
            vm.gfx.blit_blend_image(
                src_id, None, src_rect.x, src_rect.y, src_rect.w, src_rect.h, dst_x, dst_y,
            )?;
        }
        // TODO: another cmd
        _ if cmd < 30 => bail!("unimplemented cmd: {cmd}"),
        _ => bail!("invalid or unsupported cmd: {cmd}"),
    }

    Ok(())
}
