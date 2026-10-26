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
    cps,
    //
    cns,
    //
    fln, sk, sks, hf, ft, sp, hp,
    //
    es, ec,
    //
    hn, hxp,
    hln, hs, hinc, hdec, calc, hsg, ht,
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
    type Inst = Instruction;
    let mut insts = [Inst::INVALID; 256];

    const fn stub(name: &'static str) -> Instruction {
        Instruction::new_skipped(name)
    }

    insts[0x00] = Inst::new("ED", ed);
    insts[0x01] = Inst::new("LS", ls);
    insts[0x02] = Inst::new("LSBS", lsbs);
    insts[0x03] = Inst::new("SRET", sret);
    insts[0x04] = Inst::new("JP", jp);
    insts[0x05] = Inst::new("JS", js);
    insts[0x06] = Inst::new("RT", rt);
    insts[0x07] = Inst::new("ONJP", onjp);
    insts[0x08] = Inst::new("ONJS", onjs);
    insts[0x09] = Inst::new_skipped("CHILD");
    insts[0x0A] = Inst::new_skipped("URL");
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
    insts[0x20] = stub("WS"); // set text window position
    insts[0x21] = stub("WP"); // set text window background
    insts[0x22] = stub("WL");
    insts[0x23] = stub("WW");
    insts[0x24] = stub("CN");
    insts[0x25] = Inst::new("CNS", cns);
    insts[0x26] = stub("PF");
    insts[0x27] = stub("PB"); // set text window fontsize
    insts[0x28] = stub("PJ"); // set text window fontstyle?
    insts[0x29] = stub("WO"); // show text window
    insts[0x2A] = stub("WC"); // hide text window
    insts[0x2B] = stub("PM");
    insts[0x2C] = stub("PMP");
    insts[0x2D] = stub("WSH");
    insts[0x2E] = stub("WSS");
    insts[0x30] = Inst::new("FLN", fln);
    insts[0x31] = Inst::new("SK", sk);
    insts[0x32] = Inst::new("SKS", sks);
    insts[0x33] = Inst::new("HF", hf);
    insts[0x34] = Inst::new("FT", ft);
    insts[0x35] = Inst::new("SP", sp);
    insts[0x36] = Inst::new("HP", hp);
    insts[0x37] = stub("STS");
    insts[0x38] = Inst::new("ES", es);
    insts[0x39] = Inst::new("EC", ec);
    insts[0x3A] = stub("STC");
    insts[0x3B] = Inst::new("HN", hn);
    insts[0x3C] = Inst::new("HXP", hxp);
    insts[0x40] = Inst::new("HLN", hln);
    insts[0x41] = Inst::new("HS", hs);
    insts[0x42] = Inst::new("HINC", hinc);
    insts[0x43] = Inst::new("HDEC", hdec);
    insts[0x44] = Inst::new("CALC", calc);
    insts[0x45] = Inst::new("HSG", hsg);
    insts[0x46] = Inst::new("HT", ht);
    insts[0x47] = Inst::new("IF", r#if);
    insts[0x48] = Inst::new("EXA", exa);
    insts[0x49] = Inst::new("EXS", exs);
    insts[0x4A] = Inst::new("EXC", exc);
    insts[0x4B] = stub("SCP");
    insts[0x4C] = stub("SSP");
    insts[0x50] = Inst::new("VSET", vset);
    insts[0x51] = Inst::new("GN", gn);
    insts[0x52] = Inst::new("GF", gf);
    insts[0x53] = Inst::new("GC", gc);
    insts[0x54] = stub("GI");
    insts[0x55] = stub("GO");
    insts[0x56] = Inst::new("GL", gl);
    insts[0x57] = Inst::new("GP", gp);
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
    insts[0x70] = Inst::new("ML", ml);
    insts[0x71] = Inst::new("MP", mp);
    insts[0x72] = Inst::new("MF", mf);
    insts[0x73] = Inst::new("MS", ms);
    insts[0x74] = Inst::new("SER", ser);
    insts[0x75] = Inst::new("SEP", sep);
    insts[0x76] = Inst::new("SED", sed);
    insts[0x77] = Inst::new("PCMON", pcmon);
    insts[0x78] = Inst::new("PCML", pcml);
    insts[0x79] = Inst::new("PCMS", pcms);
    insts[0x7A] = Inst::new("PCMEND", pcmend);
    insts[0x7B] = Inst::new("SES", ses);
    insts[0x7C] = stub("BGMGETPOS");
    insts[0x7D] = stub("SEGETPOS");
    insts[0x7E] = stub("PCMGETPOS");
    insts[0x7F] = stub("PCMCN");
    insts[0x80] = stub("IM");
    insts[0x81] = stub("IC");
    insts[0x82] = stub("IMS");
    insts[0x83] = stub("IXY"); // move mouse cursor
    insts[0x84] = Inst::new("IH", ih);
    insts[0x85] = stub("IG");
    insts[0x86] = Inst::new_skipped("IGINIT");
    insts[0x87] = Inst::new_skipped("IGRELEASE");
    insts[0x88] = Inst::new("IHK", ihk);
    insts[0x89] = Inst::new("IHKDEF", ihkdef);
    insts[0x8A] = Inst::new("IHGL", ihgl);
    insts[0x8B] = Inst::new("IHGC", ihgc);
    insts[0x8C] = stub("IHGP");
    insts[0x8D] = stub("CLK");
    insts[0x8E] = stub("IGN");

    insts[0x8F] = stub("<8F>");

    insts[0x90] = stub("DAE");
    insts[0x91] = Inst::new("DAP", dap);
    insts[0x92] = Inst::new("DAS", das);
    insts[0x9F] = Inst::new("SETINSIDEVOL", setinsidevol);
    insts[0xA0] = stub("KIDCLR");
    insts[0xA1] = stub("KIDMOJI");
    insts[0xA2] = stub("KIDPAGE");
    insts[0xA3] = stub("KIDSET");
    insts[0xA4] = stub("KIDEND");
    insts[0xA5] = Inst::new("KIDFN", kidfn);
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
    insts[0xF0] = Inst::new("CNF", cnf);
    insts[0xF1] = Inst::new("ATIMES", atimes);
    insts[0xF2] = Inst::new("AWAIT", r#await);
    insts[0xF3] = stub("AVIP");
    insts[0xF4] = stub("PPF"); // enable/disable context menu
    insts[0xF5] = stub("SVF"); // enable/disable saving
    insts[0xF6] = stub("PPE");
    insts[0xF7] = Inst::new("SETGAMEINFO", setgameinfo);
    insts[0xF8] = stub("SETFONTSTYLE");
    insts[0xF9] = stub("SETFONTCOLOR");
    insts[0xFA] = Inst::new("TIMERSET", timerset);
    insts[0xFB] = Inst::new("TIMEREND", timerend);
    insts[0xFC] = Inst::new("TIMERGET", timerget);
    insts[0xFD] = stub("GRPOUT");
    insts[0xFE] = stub("BREAK");
    insts[0xFF] = stub("EXT");

    let mut op = 0x00;
    while op <= 0xFF {
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

    pub(crate) const fn new_skipped(name: &'static str) -> Self {
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

        // dbg!(&vm.parser.scenario.name);
        // dbg!(self.name);
        // let offset = ip + vm.parser.scenario.metadata.code_offset;
        // dbg!(ip);
        // dbg!(offset);

        // end of scenario
        if info.len == 0 {
            vm.state.exit();
            return Ok(());
        }

        let result = (self.exec_fn)(vm, info);

        if vm.parser.scenario.is_debug_version() {
            if let Err(err) = result {
                dbg!(err);
                vm.parser.state.ip = next_ip
            }
        } else {
            result?;
        }

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
    fn parse(parser: &mut crate::ScenarioParser) -> Result<Self> {
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
