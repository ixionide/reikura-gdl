use anyhow::{Result, bail};

use crate::instruction::*;

type InstFn = fn(&mut crate::Vm) -> Result<()>;

pub fn invalid_instruction(_vm: &mut crate::Vm) -> Result<()> {
    bail!("invalid instruction")
}

pub fn unsupported_instruction(_vm: &mut crate::Vm) -> anyhow::Result<()> {
    Ok(())
}

pub const INSTRUCTIONS: [InstFn; 256] = const {
    let mut insts = [invalid_instruction as InstFn; 256];

    insts[0x00] = Ed::execute as InstFn;
    insts[0x01] = Ls::execute as InstFn;
    insts[0x02] = Lsbs::execute as InstFn;
    insts[0x03] = Sret::execute as InstFn;
    insts[0x04] = Jp::execute as InstFn;
    insts[0x05] = Js::execute as InstFn;
    insts[0x06] = Rt::execute as InstFn;
    insts[0x07] = Onjp::execute as InstFn;
    insts[0x08] = Onjs::execute as InstFn;
    insts[0x09] = unsupported_instruction; // Child unsupported
    insts[0x0A] = unsupported_instruction; // Url unsupported
    // insts[0x10] = Cw::execute as InstFn;
    // insts[0x11] = Cp::execute as InstFn;
    // insts[0x12] = Cir::execute as InstFn;
    // insts[0x13] = Cps::execute as InstFn;
    // insts[0x14] = Cip::execute as InstFn;
    // insts[0x15] = Cset::execute as InstFn;
    // insts[0x16] = Cwo::execute as InstFn;
    // insts[0x17] = Cwc::execute as InstFn;
    // insts[0x18] = Cc::execute as InstFn;
    // insts[0x19] = Cclr::execute as InstFn;
    // insts[0x1A] = Creset::execute as InstFn;
    // insts[0x1B] = Crnd::execute as InstFn;
    // insts[0x1C] = Ctext::execute as InstFn;
    // insts[0x20] = Ws::execute as InstFn;
    // insts[0x21] = Wp::execute as InstFn;
    // insts[0x22] = Wl::execute as InstFn;
    // insts[0x23] = Ww::execute as InstFn;
    // insts[0x24] = Cn::execute as InstFn;
    // insts[0x25] = Cns::execute as InstFn;

    insts
};
