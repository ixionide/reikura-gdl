use std::{
    fmt::{Display, Write as _},
    fs::File,
    io::{BufWriter, Write as _},
    path::Path,
};

use reikura_gdl::{
    AssetName, Parser, Scenario,
    instruction::{INSTRUCTIONS, Instruction, InstructionInfo, Value},
    secretfilter::{Deobfuscator, SIGNATURE, filters::get_known_filter},
};
use reikura_util::encoding::sjis_to_utf8;

fn get_obfuscated(data: &mut [u8]) -> Option<&mut [u8]> {
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
                    deopfuscator = get_known_filter(&title_id).map(Deobfuscator::new)
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
            mnemonic = format!("<{:02X}>", inst.opcode)
        }

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
        let indent = "    ";

        if let Some(err) = err {
            return Err(std::io::Error::other(err));
        };

        if params.is_empty() {
            writeln!(out, "{indent}{mnemonic}")
        } else {
            writeln!(out, "{indent}{mnemonic:16} {params}")
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
                fmt.add_param(display_label(label));
            }
            "ONJP" | "ONJS" => {
                let switch = parser.read_param()?;
                let branch_count: u8 = parser.read_param()?;
                fmt.add_param(display_value(switch));

                for _ in 0..branch_count {
                    let label = parser.read_param()?;
                    fmt.add_param(display_label(label));
                }
            }
            "CW" => {
                let par1: u8 = parser.read_param()?;
                fmt.add_param(par1);

                for _ in 0..4 {
                    let value = parser.read_param()?;
                    fmt.add_param(display_value(value));
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
                    fmt.add_param(display_value(value));
                }

                let bytes = parser.read_bytes(inst_info.param_len - 18)?;
                let string = sjis_to_utf8(bytes)?;

                if let Some(c) = string.strip_circumfix('"', '"') {
                    fmt.add_param(display_string(c));
                } else {
                    fmt.add_param(display_string(&string));
                }
            }
            "CWO" => {
                let par1: u8 = parser.read_param()?;
                let value = parser.read_param()?;
                let par2: u8 = parser.read_param()?;
                fmt.add_param(par1)
                    .add_param(display_value(value))
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
                    fmt.add_param(display_value(value));
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
                    fmt.add_param(display_value(value));
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

                let name = parser.read_bytes(inst_info.param_len - 2)?;
                fmt.add_param(display_bytes_as_string(name)?);
            }
            "PF" | "PB" | "PJ" => {
                let par: u8 = parser.read_param()?;
                fmt.add_param(par);
                let value = parser.read_param()?;
                fmt.add_param(display_value(value));
            }
            "WO" | "WC" => {
                let par: u8 = parser.read_param()?;
                fmt.add_param(par);
            }
            // "PM"
            // "PMP"
            "WSH" | "WSS" => {
                let value = parser.read_param()?;
                fmt.add_param(display_value(value));
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
                fmt.add_param(par).add_param(display_label(label));
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
            "STC" => {
                for _ in 0..2 {
                    let par: u8 = parser.read_param()?;
                    fmt.add_param(par);
                }

                let label = parser.read_param()?;
                fmt.add_param(display_label(label));
            }
            "HN" => {
                let par: u16 = parser.read_param()?;
                let label = parser.read_param()?;
                fmt.add_param(par).add_param(display_label(label));
            }
            "HXP" => {
                for _ in 0..2 {
                    let par: u8 = parser.read_param()?;
                    fmt.add_param(par);
                }

                let label = parser.read_param()?;
                fmt.add_param(display_label(label));
            }
            "HLN" => {
                let par: u16 = parser.read_param()?;
                fmt.add_param(par);
            }
            "HS" => {
                let par: u16 = parser.read_param()?;
                let value = parser.read_param()?;
                fmt.add_param(par).add_param(display_value(value));
            }
            "HINC" | "HDEC" => {
                let par: u16 = parser.read_param()?;
                fmt.add_param(par);
            }
            // "CALC"
            "HSG" => {
                for _ in 0..2 {
                    let par: u16 = parser.read_param()?;
                    fmt.add_param(par);
                }

                let value = parser.read_param()?;
                fmt.add_param(display_value(value));
            }
            "HT" => {
                for _ in 0..3 {
                    let par: u16 = parser.read_param()?;
                    fmt.add_param(par);
                }
            }
            // "IF"
            "EXA" => {
                for _ in 0..2 {
                    let par: u16 = parser.read_param()?;
                    fmt.add_param(par);
                }
            }
            "EXS" | "EXC" => {
                for _ in 0..3 {
                    let value = parser.read_param()?;
                    fmt.add_param(display_value(value));
                }

                let par: u8 = parser.read_param()?;
                fmt.add_param(par);
            }
            "SCP" | "SSP" => {
                let par1: u16 = parser.read_param()?;
                let par2: u8 = parser.read_param()?;
                fmt.add_param(par1).add_param(par2);
            }
            "VSET" | "GN" => {
                for _ in 0..3 {
                    let value = parser.read_param()?;
                    fmt.add_param(display_value(value));
                }
            }
            "GF" | "GI" => (),
            "GC" => {
                let value = parser.read_param()?;
                fmt.add_param(display_value(value));
                let r: u8 = parser.read_param()?;
                let g: u8 = parser.read_param()?;
                let b: u8 = parser.read_param()?;
                fmt.add_param(r).add_param(g).add_param(b);
            }
            "GO" => {
                let value = parser.read_param()?;
                fmt.add_param(display_value(value));
                let r: u8 = parser.read_param()?;
                let g: u8 = parser.read_param()?;
                let b: u8 = parser.read_param()?;
                let par: u8 = parser.read_param()?;
                fmt.add_param(r).add_param(g).add_param(b).add_param(par);
            }
            "GL" => {
                let value = parser.read_param()?;
                let asset = parser.read_param()?;
                fmt.add_param(display_value(value))
                    .add_param(display_assetname(asset)?);
            }
            // "GP"
            "GB" => {
                let value = parser.read_param()?;
                fmt.add_param(display_value(value));
                let r: u8 = parser.read_param()?;
                let g: u8 = parser.read_param()?;
                let b: u8 = parser.read_param()?;
                fmt.add_param(r).add_param(g).add_param(b);

                let par: u8 = parser.read_param()?;
                fmt.add_param(par);
                for _ in 0..4 {
                    let value = parser.read_param()?;
                    fmt.add_param(display_value(value));
                }
            }
            "GPB" => {
                let value = parser.read_param()?;
                fmt.add_param(display_value(value));
            }
            // "GPJ"
            // "PR"
            // "GASTART"
            // "GASTOP"
            // "GPI"
            // "GPO"
            "GGE" => {
                for _ in 0..5 {
                    let value = parser.read_param()?;
                    fmt.add_param(display_value(value));
                }

                let asset = parser.read_param()?;
                fmt.add_param(display_assetname(asset)?);
            }
            // "GPE"
            "GSCRL" => {
                let value = parser.read_param()?;
                fmt.add_param(display_value(value));
            }
            _ => {
                let params = parser.read_bytes(inst_info.param_len)?;
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
        return Ok(String::new());
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

fn display_string(string: &str) -> String {
    string.escape_default().to_string()
}

fn display_value(value: Value) -> String {
    match value {
        Value::Literal(value) => format!("{value}"),
        Value::Register(index) => format!("@{index}"),
        Value::Random(max) => format!("%{max}"),
    }
}

fn display_bytes_as_string(bytes: &[u8]) -> anyhow::Result<String> {
    let string = sjis_to_utf8(bytes)?;
    Ok(display_string(&string))
}

fn display_assetname(asset: AssetName) -> anyhow::Result<String> {
    display_bytes_as_string(asset.name())
}

fn display_label(index: u16) -> String {
    format!("LABEL_{index:04}")
}
