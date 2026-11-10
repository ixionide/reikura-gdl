use anyhow::bail;

use crate::{
    Vm,
    graphic::BlitParam,
    instruction::{InstructionInfo, Value},
};

const BLIT_COPY: u8 = 0;
const BLIT_BLEND: u8 = 1;
const TRANSITION_FADE: u8 = 19;

#[allow(unused)]
pub fn gp(vm: &mut Vm, _info: InstructionInfo) -> anyhow::Result<()> {
    let (ctx, parser) = (&vm.ctx, &mut vm.parser);

    let cmd: u8 = parser.read_param()?;
    let src_id_value = parser.read_param::<Value>()?.eval(ctx);
    let sx = parser.read_param::<Value>()?.eval(ctx);
    let sy = parser.read_param::<Value>()?.eval(ctx);
    let sw = parser.read_param::<Value>()?.eval(ctx);
    let sh = parser.read_param::<Value>()?.eval(ctx);
    let value = parser.read_param::<Value>()?.eval(ctx);
    let dx = parser.read_param::<Value>()?.eval(ctx);
    let dy = parser.read_param::<Value>()?.eval(ctx);

    let src_id = u8::try_from(src_id_value)?;
    match cmd {
        BLIT_COPY => {
            let dst_id = Some(u8::try_from(value)?);

            vm.gfx.blit_copy_image(
                src_id,
                dst_id,
                BlitParam::new_offset(sx, sy, sw, sh, dx, dy),
            )?;
        }
        BLIT_BLEND => {
            let dst_id = Some(u8::try_from(value)?);

            vm.gfx.blit_blend_image(
                src_id,
                dst_id,
                BlitParam::new_offset(sx, sy, sw, sh, dx, dy),
            )?;
        }
        TRANSITION_FADE => {
            // let _duration = Duration::from_millis(u64::try_from(value)?);
            let duration = u32::try_from(value)?;

            // TODO
            vm.gfx
                .blit_blend_image(src_id, None, BlitParam::new_offset(sx, sy, sw, sh, dx, dy))?
        }
        // TODO: another cmd
        _ if cmd < 30 => bail!("unimplemented cmd: {cmd}"),
        _ => bail!("invalid or unsupported cmd: {cmd}"),
    }

    Ok(())
}
