use anyhow::bail;

use crate::{
    Vm,
    graphic::BlitParam,
    instruction::{InstructionInfo, Value},
};

const BLIT_COPY: u8 = 0;
const BLIT_BLEND: u8 = 1;
const TRANSITION_FADE: u8 = 19;
const BLIT_COPY_ALPHA: u8 = 20;
const BLIT_BLEND_ALPHA: u8 = 21;

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
        BLIT_COPY | BLIT_COPY_ALPHA => {
            if cmd == BLIT_COPY_ALPHA {
                // TODO: maybe we need a premultiplied image for this?
                let _value = parser.read_param::<Value>()?.eval(ctx);
            }

            let dst_id = u8::try_from(value)?;
            vm.gfx.blit_copy_image(
                src_id,
                Some(dst_id),
                BlitParam::new_offset(sx, sy, sw, sh, dx, dy),
            )?;
        }
        BLIT_BLEND => {
            let dst_id = u8::try_from(value)?;
            vm.gfx.blit_blend_image(
                src_id,
                Some(dst_id),
                BlitParam::new_offset(sx, sy, sw, sh, dx, dy),
                None,
            )?;
        }
        // NOTE: in crescendo executable if its not full-alpha and the
        // blit is not full-region either its crashed the game
        // i hope that this is an actual bug
        // so we dont need to follow its weird behavior.
        // Example:
        // inst, cmd, src, sx,  sy, w,   h,   dst, dx,  dy, alpha
        // GP    21,  2,   320, 0,  320, 480, 3,   320, 0,  127  ; crash
        // GP    21,  2,   0,   0,  640, 480, 3,   0,   0,  127  ; fine
        BLIT_BLEND_ALPHA => {
            let dst_id = u8::try_from(value)?;
            let alpha = parser.read_param::<Value>()?.eval(ctx);
            vm.gfx.blit_blend_image(
                src_id,
                Some(dst_id),
                BlitParam::new_offset(sx, sy, sw, sh, dx, dy),
                Some(alpha.clamp(0, 255) as u8),
            )?;
        }
        TRANSITION_FADE => {
            let duration_ms = u32::try_from(value)?;

            if duration_ms != 0 {
                vm.gfx.start_fade_in_image(
                    src_id,
                    BlitParam::new_offset(sx, sy, sw, sh, dx, dy),
                    duration_ms,
                )?;
                vm.runtime.state.wait_transition();
            } else {
                vm.gfx.blit_blend_image(
                    src_id,
                    None,
                    BlitParam::new_offset(sx, sy, sw, sh, dx, dy),
                    None,
                )?;
            }
        }
        // TODO: another cmd
        _ if cmd < 30 => bail!("unimplemented cmd: {cmd}"),
        _ => bail!("invalid or unsupported cmd: {cmd}"),
    }

    Ok(())
}
