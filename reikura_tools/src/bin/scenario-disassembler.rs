use std::{
    fmt::{Display, Write as _},
    fs::File,
    io::{BufWriter, Write as _},
    path::Path,
};

use reikura_gdl::{
    AssetName, Parser, Scenario,
    instruction::{INSTRUCTIONS, Instruction, InstructionInfo, Value},
    secretfilter::{Deobfuscator, filters::get_known_filter},
};
use reikura_util::encoding::sjis_to_utf8;

fn get_obfuscated(data: &mut [u8]) -> Option<&mut [u8]> {
    use reikura_gdl::secretfilter::SIGNATURE;

    let mid = data.len().checked_sub(SIGNATURE.len())?;
    let (data, end) = data.split_at_mut(mid);

    if end == SIGNATURE {
        return Some(data);
    }

    None
}

fn main() {
    let mut deopfuscator = None;
    let mut args = std::env::args().skip(1);

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-k" => {
                if let Some(title_id) = args.next() {
                    deopfuscator =
                        get_known_filter(&title_id).map(|filter| Deobfuscator::new(filter))
                }
            }
            "-e" => {
                if let Some(exe_path) = args.next() {
                    let _ = exe_path;
                };
            }
            _ => {
                let path = Path::new(&arg);

                if path
                    .extension()
                    .is_none_or(|ext| !ext.eq_ignore_ascii_case("isf"))
                {
                    continue;
                }

                let Ok(mut data) = std::fs::read(path) else {
                    eprintln!("failed to read {arg}");
                    continue;
                };

                if let Some(obfuscated) = get_obfuscated(&mut data) {
                    let Some(deobfuscator) = &deopfuscator else {
                        eprintln!("scenario {arg} is obfuscated but the key is unknown");
                        continue;
                    };
                    deobfuscator.deobfuscate(obfuscated);
                }

                let outpath = path.with_extension("txt");
                let scenario = Scenario::load(String::new(), data).unwrap();
                disassemble(&outpath, scenario).unwrap();
            }
        }
    }
}

struct Formatter {
    mnemonic: String,
    params: String,
    err: Option<std::fmt::Error>,
}

impl Formatter {
    fn new(inst: &Instruction) -> Self {
        let mut mnemonic = inst.name.to_owned();

        if mnemonic == "INVALID" {
            mnemonic = format!("INVALID({:02X})", inst.opcode)
        }

        // let result = write!(out, "{mnemonic:15}");

        Self {
            mnemonic,
            params: String::with_capacity(256),
            err: None,
        }
    }

    fn add_param(&mut self, param: impl Display) -> &mut Self {
        if self.err.is_some() {
            return self;
        }

        let result = {
            if self.params.is_empty() {
                write!(self.params, "{param}")
            } else {
                write!(self.params, ", {param}")
            }
        };

        self.err = result.err();

        self
    }

    fn write(self, out: &mut BufWriter<File>) -> std::io::Result<()> {
        let Self {
            mnemonic,
            params,
            err,
        } = self;

        if let Some(err) = err {
            return Err(std::io::Error::other(err));
        };

        if params.is_empty() {
            writeln!(out, "\t{mnemonic}")
        } else {
            writeln!(out, "\t{mnemonic:15} {params}")
        }
    }
}

fn disassemble(outpath: &Path, scenario: Scenario) -> anyhow::Result<()> {
    let mut parser = Parser::new(scenario);
    let mut sub_index = 0;
    let mut out = BufWriter::new(File::create_new(outpath)?);

    while Some(parser.state.ip) == parser.state.scenario.sub_offset(sub_index) {
        writeln!(out, "#LABEL_{sub_index:04}:")?;
        sub_index += 1;
    }

    while let Ok(opcode) = parser.read_opcode() {
        let inst = INSTRUCTIONS[opcode as usize];
        let inst_info: InstructionInfo = parser.read_param()?;

        // terminator
        if inst_info.len == 0 {
            return Ok(());
        }

        let mut fmt = Formatter::new(&inst);

        match inst.name {
            "ED" | "SRET" | "RT" => (),
            "LS" | "LSBS" => {
                let asset = parser.read_param()?;
                fmt.add_param(display_assetname(asset)?);
            }
            "JP" | "JS" => {
                let label: u16 = parser.read_param()?;
                fmt.add_param(display_label(label)?);
            }
            "ONJP" | "ONJS" => {
                let switch = parser.read_param()?;
                let branch_count: u8 = parser.read_param()?;
                fmt.add_param(display_value(switch)?);

                for _ in 0..branch_count {
                    let label = parser.read_param()?;
                    fmt.add_param(display_label(label)?);
                }
            }
            "CW" => {
                let par1: u8 = parser.read_param()?;
                fmt.add_param(par1);

                for _ in 0..4 {
                    let value = parser.read_param()?;
                    fmt.add_param(display_value(value)?);
                }

                let cmd: u8 = parser.read_param()?;
                fmt.add_param(cmd);
            }
            "CP" | "CIR" => {
                for _ in 0..2 {
                    let par: u8 = parser.read_param()?;
                    fmt.add_param(par);
                }

                let asset = parser.read_param()?;
                fmt.add_param(display_assetname(asset)?);
            }
            "CPS" => {
                let par: u8 = parser.read_param()?;
                fmt.add_param(par);

                for _ in 0..6 {
                    let r: u8 = parser.read_param()?;
                    let g: u8 = parser.read_param()?;
                    let b: u8 = parser.read_param()?;
                    fmt.add_param(r).add_param(g).add_param(b);
                }
            }
            // "CIP"
            "CSET" => {
                for _ in 0..2 {
                    let par: u8 = parser.read_param()?;
                    fmt.add_param(par);
                }

                for _ in 0..4 {
                    let value = parser.read_param()?;
                    fmt.add_param(display_value(value)?);
                }
                // TODO: choicestring
            }
            "CWO" => {
                let par1: u8 = parser.read_param()?;
                let value = parser.read_param()?;
                let par2: u8 = parser.read_param()?;
                fmt.add_param(par1)
                    .add_param(display_value(value)?)
                    .add_param(par2);
            }
            "CWC" => {
                let par1: u8 = parser.read_param()?;
                fmt.add_param(par1);
            }
            "CC" => {
                for _ in 0..4 {
                    let par1: u8 = parser.read_param()?;
                    fmt.add_param(par1);
                }
            }
            // "CCLR"
            // "RESET"
            // "CRND"
            // "CTEXT"
            "WS" => {
                let par: u8 = parser.read_param()?;
                fmt.add_param(par);

                for _ in 0..4 {
                    let value = parser.read_param()?;
                    fmt.add_param(display_value(value)?);
                }

                let par: u8 = parser.read_param()?;
                fmt.add_param(par);
            }
            // "WP"
            "WL" => {
                let par: u8 = parser.read_param()?;
                let asset = parser.read_param()?;
                fmt.add_param(par).add_param(display_assetname(asset)?);
            }
            "WW" => {
                let par: u8 = parser.read_param()?;
                fmt.add_param(par);

                for _ in 0..2 {
                    let value = parser.read_param()?;
                    fmt.add_param(display_value(value)?);
                }

                let par: u8 = parser.read_param()?;
                fmt.add_param(par);
            }
            // "CN"
            "CNS" => {
                for _ in 0..2 {
                    let par: u8 = parser.read_param()?;
                    fmt.add_param(par);
                }

                let mut buf = vec![0; inst_info.param_len - 2];
                for b in buf.iter_mut() {
                    *b = parser.read_param()?;
                }
                fmt.add_param(sjis_to_utf8(&buf)?);
            }
            "PF" | "PB" | "PJ" => {
                let par: u8 = parser.read_param()?;
                fmt.add_param(par);
                let value = parser.read_param()?;
                fmt.add_param(display_value(value)?);
            }
            "WO" | "WC" => {
                let par: u8 = parser.read_param()?;
                fmt.add_param(par);
            }
            // "PM"
            // "PMP"
            "WSH" | "WSS" => {
                let value = parser.read_param()?;
                fmt.add_param(display_value(value)?);
            }
            "FLN" => {
                let par: u16 = parser.read_param()?;
                fmt.add_param(par);
            }
            "SK" => {
                let par1: u16 = parser.read_param()?;
                let par2: u8 = parser.read_param()?;
                fmt.add_param(par1).add_param(par2);
            }
            "SKS" => {
                let par1: u16 = parser.read_param()?;
                let par2: u16 = parser.read_param()?;
                let par3: u8 = parser.read_param()?;
                fmt.add_param(par1).add_param(par2).add_param(par3);
            }
            "HF" => {
                let par: u16 = parser.read_param()?;
                let label: u16 = parser.read_param()?;
                fmt.add_param(par).add_param(display_label(label)?);
            }
            "FT" => {
                let par1: u16 = parser.read_param()?;
                let par2: u16 = parser.read_param()?;
                let par3: u16 = parser.read_param()?;
                fmt.add_param(par1).add_param(par2).add_param(par3);
            }
            // "SP"
            // "STS"
            "ES" | "EC" => {
                let par1: u16 = parser.read_param()?;
                let par2: u16 = parser.read_param()?;
                fmt.add_param(par1).add_param(par2);
            }
            _ => {
                let params = &parser.state.scenario.code[parser.state.ip..][..inst_info.param_len];
                parser.state.ip += inst_info.param_len;
                fmt.add_param(display_bytes(params)?);
            }
        }

        fmt.write(&mut out)?;

        while Some(parser.state.ip) == parser.state.scenario.sub_offset(sub_index) {
            writeln!(out, "#LABEL_{sub_index:04}:")?;
            sub_index += 1;
        }
    }

    Ok(())
}

fn display_bytes(bytes: &[u8]) -> Result<String, std::fmt::Error> {
    let mut display = String::with_capacity(bytes.len());

    if bytes.is_empty() {
        return Ok(display);
    }

    write!(display, "<")?;

    for byte in bytes.iter() {
        if display.is_empty() {
            write!(display, "{byte:2X}")?;
        } else {
            write!(display, " {byte:2X}")?;
        }
    }

    writeln!(display, ">")?;

    Ok(display)
}

fn display_value(value: Value) -> Result<String, std::fmt::Error> {
    let mut display = String::with_capacity(16);

    match value {
        Value::Literal(value) => write!(display, "{value}")?,
        Value::Register(index) => write!(display, "@{index}")?,
        Value::Random(max) => write!(display, "%{max}")?,
    }

    Ok(display)
}

fn display_assetname(asset: AssetName) -> Result<String, reikura_util::encoding::InvalidSJIS> {
    sjis_to_utf8(asset.filename())
}

fn display_label(index: u16) -> Result<String, std::fmt::Error> {
    let mut display = String::from("LABEL_");
    write!(display, "{index:04}")?;
    Ok(display)
}
