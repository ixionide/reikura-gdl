use anyhow::{Result, bail};

use crate::instruction::*;

type InstFn = fn(&mut crate::Vm, InstructionInfo) -> anyhow::Result<()>;

pub fn invalid_instruction(_vm: &mut crate::Vm, _info: InstructionInfo) -> Result<()> {
    bail!("invalid instruction")
}

pub const INSTRUCTIONS: [InstFn; 256] = const {
    let mut insts = [invalid_instruction as InstFn; 256];

    insts[0x00] = Ed::execute;
    insts[0x01] = Ls::execute;
    insts[0x02] = Lsbs::execute;
    insts[0x03] = Sret::execute;
    insts[0x04] = Jp::execute;
    insts[0x05] = Js::execute;
    insts[0x06] = Rt::execute;
    insts[0x07] = Onjp::execute;
    insts[0x08] = Onjs::execute;
    insts[0x09] = Child::skip;
    insts[0x0A] = Url::skip;
    // insts[0x10] = Cw::execute;
    // insts[0x11] = Cp::execute;
    // insts[0x12] = Cir::execute;
    // insts[0x13] = Cps::execute;
    // insts[0x14] = Cip::execute;
    // insts[0x15] = Cset::execute;
    // insts[0x16] = Cwo::execute;
    // insts[0x17] = Cwc::execute;
    // insts[0x18] = Cc::execute;
    // insts[0x19] = Cclr::execute;
    // insts[0x1A] = Creset::execute;
    // insts[0x1B] = Crnd::execute;
    // insts[0x1C] = Ctext::execute;
    // insts[0x20] = Ws::execute;
    // insts[0x21] = Wp::execute;
    // insts[0x22] = Wl::execute;
    // insts[0x23] = Ww::execute;
    // insts[0x24] = Cn::execute;
    // insts[0x25] = Cns::execute;
    // insts[0x26] = Pf::execute;
    // insts[0x27] = Pb::execute;
    // insts[0x28] = Pj::execute;
    // insts[0x29] = Wo::execute;
    // insts[0x2A] = Wc::execute;
    // insts[0x2B] = Pm::execute;
    // insts[0x2C] = Pmp::execute;
    // insts[0x2D] = Wsh::execute;
    // insts[0x2E] = Wss::execute;
    insts[0x30] = Fln::execute;
    insts[0x31] = Sk::execute;
    insts[0x32] = Sks::execute;
    insts[0x33] = Hf::execute;
    insts[0x34] = Ft::execute;
    // insts[0x35] = Sp::execute;
    // insts[0x36] = Hp::execute;
    // insts[0x37] = Sts::execute;
    // insts[0x38] = Es::execute;
    // insts[0x39] = Ec::execute;
    // insts[0x3A] = Stc::execute;
    insts[0x3B] = Hn::execute;
    // insts[0x3C] = Hxp::execute;
    insts[0x40] = Hln::execute;
    insts[0x41] = Hs::execute;
    insts[0x42] = Hinc::execute;
    insts[0x43] = Hdec::execute;
    insts[0x44] = Calc::execute;
    insts[0x45] = Hsg::execute;
    insts[0x46] = Ht::execute;
    insts[0x47] = If::execute;
    insts[0x48] = Exa::execute;
    // insts[0x49] = Exs::execute;
    // insts[0x4A] = Exc::execute;
    // insts[0x4B] = Scp::execute;
    // insts[0x4C] = Ssp::execute;
    // insts[0x50] = Vset::execute;
    // insts[0x51] = Gn::execute;

    insts
};
