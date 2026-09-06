mod _param;
pub use _param::*;

macro_rules! inst {
    ($($id:ident),*) => ($(
        mod $id;
        #[allow(unused_imports)] use $id::$id;
    )*)
}

inst! {
    ed, ls, lsbs, sret, rt, jp, js, onjp, onjs,
    //
    cns,
    //
    fln, sk, sks, hf, ft, sp, hp,
    //
    hn,
    //
    hln,
    //
    hs, hinc, hdec, calc, hsg, ht,
    r#if,
    exa, exs, exc,
    //
    vset, gn, gf, gc, /* gi, go, */ gl, gp,
    //
    ml, mp, mf, ms, ser, sep, sed, pcmon, pcml, pcms, pcmend, ses,
    //
    ih, /* ig, */ ihk, ihkdef, ihgl, ihgc,
    //
    /* dae, */ dap, das, setinsidevol,
    //
    kidfn,
    //
    cnf,
    atimes, r#await,
    //
    setgameinfo, // setfontstyle, setfontcolor
    timerset, timerend, timerget
}

pub const INSTRUCTIONS: [Instruction; 256] = const {
    let mut insts = [Instruction::INVALID; 256];

    const fn stub(name: &'static str) -> Instruction {
        Instruction::skip(name)
    }

    insts[0x00] = Instruction::new("ED", ed);
    insts[0x01] = Instruction::new("LS", ls);
    insts[0x02] = Instruction::new("LSBS", lsbs);
    insts[0x03] = Instruction::new("SRET", sret);
    insts[0x04] = Instruction::new("JP", jp);
    insts[0x05] = Instruction::new("JS", js);
    insts[0x06] = Instruction::new("RT", rt);
    insts[0x07] = Instruction::new("ONJP", onjp);
    insts[0x08] = Instruction::new("ONJS", onjs);
    insts[0x09] = Instruction::skip("CHILD");
    insts[0x0A] = Instruction::skip("URL");
    insts[0x10] = stub("CW");
    insts[0x11] = stub("CP");
    insts[0x12] = stub("CIR");
    insts[0x13] = stub("CPS");
    insts[0x14] = stub("CIP");
    insts[0x15] = stub("CSET");
    insts[0x16] = stub("CWO");
    insts[0x17] = stub("CWC");
    insts[0x18] = stub("CC");
    insts[0x19] = stub("CCLR");
    insts[0x1A] = stub("CRESET");
    insts[0x1B] = stub("CRND");
    insts[0x1C] = stub("CTEXT");
    insts[0x20] = stub("WS");
    insts[0x21] = stub("WP");
    insts[0x22] = stub("WL");
    insts[0x23] = stub("WW");
    insts[0x24] = stub("CN");
    insts[0x25] = Instruction::new("CNS", cns);
    insts[0x26] = stub("PF");
    insts[0x27] = stub("PB");
    insts[0x28] = stub("PJ");
    insts[0x29] = stub("WO");
    insts[0x2A] = stub("WC");
    insts[0x2B] = stub("PM");
    insts[0x2C] = stub("PMP");
    insts[0x2D] = stub("WSH");
    insts[0x2E] = stub("WSS");
    insts[0x30] = Instruction::new("FLN", fln);
    insts[0x31] = Instruction::new("SK", sk);
    insts[0x32] = Instruction::new("SKS", sks);
    insts[0x33] = Instruction::new("HF", hf);
    insts[0x34] = Instruction::new("FT", ft);
    insts[0x35] = Instruction::new("SP", sp);
    insts[0x36] = Instruction::new("HP", hp);
    insts[0x37] = stub("STS");
    insts[0x38] = stub("ES");
    insts[0x39] = stub("EC");
    insts[0x3A] = stub("STC");
    insts[0x3B] = Instruction::new("HN", hn);
    insts[0x3C] = stub("HXP");
    insts[0x40] = Instruction::new("HLN", hln);
    insts[0x41] = Instruction::new("HS", hs);
    insts[0x42] = Instruction::new("HINC", hinc);
    insts[0x43] = Instruction::new("HDEC", hdec);
    insts[0x44] = Instruction::new("CALC", calc);
    insts[0x45] = Instruction::new("HSG", hsg);
    insts[0x46] = Instruction::new("HT", ht);
    insts[0x47] = Instruction::new("IF", r#if);
    insts[0x48] = Instruction::new("EXA", exa);
    insts[0x49] = Instruction::new("EXS", exs);
    insts[0x4A] = Instruction::new("EXC", exc);
    insts[0x4B] = stub("SCP");
    insts[0x4C] = stub("SSP");
    insts[0x50] = Instruction::new("VSET", vset);
    insts[0x51] = Instruction::new("GN", gn);
    insts[0x52] = Instruction::new("GF", gf);
    insts[0x53] = Instruction::new("GC", gc);
    insts[0x54] = stub("GI");
    insts[0x55] = stub("GO");
    insts[0x56] = Instruction::new("GL", gl);
    insts[0x57] = Instruction::new("GP", gp);
    insts[0x58] = stub("GB");
    insts[0x59] = stub("GPB");
    insts[0x5A] = stub("GPJ");
    insts[0x5B] = stub("PR");
    insts[0x5C] = stub("GASTART");
    insts[0x5D] = stub("GASTOP");
    insts[0x5E] = stub("GPI");
    insts[0x5F] = stub("GPO");
    insts[0x60] = stub("GGE");
    insts[0x61] = stub("GPE");
    insts[0x62] = stub("GSCRL");
    insts[0x63] = stub("GV");
    insts[0x64] = stub("GAL");
    insts[0x65] = stub("GAOPEN");
    insts[0x66] = stub("GASET");
    insts[0x67] = stub("GAPOS");
    insts[0x68] = stub("GACLOSE");
    insts[0x69] = stub("GADELETE");
    insts[0x6F] = stub("SGL");
    insts[0x70] = Instruction::new("ML", ml);
    insts[0x71] = Instruction::new("MP", mp);
    insts[0x72] = Instruction::new("MF", mf);
    insts[0x73] = Instruction::new("MS", ms);
    insts[0x74] = Instruction::new("SER", ser);
    insts[0x75] = Instruction::new("SEP", sep);
    insts[0x76] = Instruction::new("SED", sed);
    insts[0x77] = Instruction::new("PCMON", pcmon);
    insts[0x78] = Instruction::new("PCML", pcml);
    insts[0x79] = Instruction::new("PCMS", pcms);
    insts[0x7A] = Instruction::new("PCMEND", pcmend);
    insts[0x7B] = Instruction::new("SES", ses);
    insts[0x7C] = stub("BGMGETPOS");
    insts[0x7D] = stub("SEGETPOS");
    insts[0x7E] = stub("PCMGETPOS");
    insts[0x7F] = stub("PCMCN");
    insts[0x80] = stub("IM");
    insts[0x81] = stub("IC");
    insts[0x82] = stub("IMS");
    insts[0x83] = stub("IXY");
    insts[0x84] = Instruction::new("IH", ih);
    insts[0x85] = stub("IG");
    insts[0x86] = Instruction::skip("IGINIT");
    insts[0x87] = Instruction::skip("IGRELEASE");
    insts[0x88] = Instruction::new("IHK", ihk);
    insts[0x89] = Instruction::new("IHKDEF", ihkdef);
    insts[0x8A] = Instruction::new("IHGL", ihgl);
    insts[0x8B] = Instruction::new("IHGC", ihgc);
    insts[0x8C] = stub("IHGP");
    insts[0x8D] = stub("CLK");
    insts[0x8E] = stub("IGN");

    insts[0x8F] = stub("<8F>");

    insts[0x90] = stub("DAE");
    insts[0x91] = Instruction::new("DAP", dap);
    insts[0x92] = Instruction::new("DAS", das);
    insts[0x9F] = Instruction::new("SETINSIDEVOL", setinsidevol);
    insts[0xA0] = stub("KIDCLR");
    insts[0xA1] = stub("KIDMOJI");
    insts[0xA2] = stub("KIDPAGE");
    insts[0xA3] = stub("KIDSET");
    insts[0xA4] = stub("KIDEND");
    insts[0xA5] = Instruction::new("KIDFN", kidfn);
    insts[0xA6] = stub("KIDHABA");
    insts[0xA7] = stub("KIDSCAN");

    insts[0xAD] = stub("<AD>");

    insts[0xAE] = stub("SETKIDWNDPUTPOS");
    insts[0xAF] = stub("SETMESWNDPUTPOS");
    insts[0xB0] = stub("INNAME");
    insts[0xB1] = stub("NAMECOPY");
    insts[0xB2] = stub("CHANGEWALL");
    insts[0xB3] = stub("MSGBOX");
    insts[0xB4] = stub("SETSMPRATE");
    insts[0xBD] = stub("CLKEXMCSET");
    insts[0xBE] = stub("IRCLK");
    insts[0xBF] = stub("IROPN");
    insts[0xD0] = stub("PPTL");
    insts[0xD1] = stub("PPABL");
    insts[0xD2] = stub("PPTYPE");
    insts[0xD3] = stub("PPORT");
    insts[0xD4] = stub("PPCRT");
    insts[0xD5] = stub("SABL");
    insts[0xD6] = stub("MPM");
    insts[0xD7] = stub("VOC");
    insts[0xD8] = stub("PM2");
    insts[0xD9] = stub("MPM2");

    insts[0xDA] = stub("<DA>");

    insts[0xE0] = stub("TAGSET");
    insts[0xE1] = stub("FRAMESET");
    insts[0xE2] = stub("RBSET");
    insts[0xE3] = stub("CBSET");
    insts[0xE4] = stub("SLDRSET");
    insts[0xE5] = stub("OPSL");
    insts[0xE6] = stub("OPPROP");
    insts[0xE7] = stub("DISABLE");
    insts[0xE8] = stub("ENABLE");
    insts[0xE9] = stub("TITLE");
    insts[0xEF] = stub("EXT2");
    insts[0xF0] = Instruction::new("CNF", cnf);
    insts[0xF1] = Instruction::new("ATIMES", atimes);
    insts[0xF2] = Instruction::new("AWAIT", r#await);
    insts[0xF3] = stub("AVIP");
    insts[0xF4] = stub("PPF");
    insts[0xF5] = stub("SVF");
    insts[0xF6] = stub("PPE");
    insts[0xF7] = Instruction::new("SETGAMEINFO", setgameinfo);
    insts[0xF8] = stub("SETFONTSTYLE");
    insts[0xF9] = stub("SETFONTCOLOR");
    insts[0xFA] = Instruction::new("TIMERSET", timerset);
    insts[0xFB] = Instruction::new("TIMEREND", timerend);
    insts[0xFC] = Instruction::new("TIMERGET", timerget);
    insts[0xFD] = stub("GRPOUT");
    insts[0xFE] = stub("BREAK");
    insts[0xFF] = stub("EXT");

    let mut op = 0;
    while op != 256 {
        insts[op].opcode = op as u8;
        op += 1;
    }

    insts
};

use anyhow::{Result, bail};
use reikura_util::io::ReadExt;

use crate::Vm;

#[derive(Clone, Copy)]
pub struct Instruction {
    pub name: &'static str,
    pub opcode: u8,
    exec_fn: fn(&mut Vm, InstructionInfo) -> Result<()>,
}

impl Instruction {
    pub const INVALID: Self = {
        fn invalid_fn(_vm: &mut Vm, _info: InstructionInfo) -> Result<()> {
            bail!("invalid instruction called")
        }

        Self::new("", invalid_fn)
    };

    pub(crate) const fn new(
        name: &'static str,
        exec_fn: fn(&mut Vm, InstructionInfo) -> Result<()>,
    ) -> Self {
        Self {
            name,
            opcode: 0x00,
            exec_fn,
        }
    }

    pub(crate) const fn skip(name: &'static str) -> Self {
        use std::io::Seek;

        fn skip_fn(vm: &mut Vm, info: InstructionInfo) -> Result<()> {
            vm.parser.seek_relative(info.param_len as i64)?;
            Ok(())
        }

        Self::new(name, skip_fn)
    }

    #[inline]
    pub fn execute(&self, vm: &mut Vm, ip: usize) -> Result<()> {
        let info: InstructionInfo = vm.parser.read_param()?;
        let next_ip = ip + info.len;

        // terminator
        if info.len == 0 {
            vm.state.exit();
            return Ok(());
        }

        (self.exec_fn)(vm, info)?;

        // TODO: log something is wrong
        if vm.parser.state.ip != next_ip {
            vm.parser.state.ip = next_ip;
        };

        Ok(())
    }
}

#[derive(Clone, Copy)]
pub struct InstructionInfo {
    pub len: usize,
    pub param_len: usize,
}

impl Parameters for InstructionInfo {
    #[inline]
    fn parse(parser: &mut crate::Parser) -> Result<Self> {
        let (len, param_len) = match parser.get_le::<u8>()? as usize {
            0 | 1 => (0, 0),
            hi if hi & 0x80 != 0 => {
                let len = {
                    let hi = (hi & 0x7F) << 8;
                    let lo = parser.get_le::<u8>()? as usize;
                    hi | lo
                };

                (len, len - 3)
            }
            len => (len, len - 2),
        };

        Ok(Self { len, param_len })
    }
}
