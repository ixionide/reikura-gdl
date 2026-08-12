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

    fn param(&mut self, param: impl Display) -> &mut Self {
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
            writeln!(out, "{mnemonic}")
        } else {
            writeln!(out, "{mnemonic:15} {params}")
        }
    }
}

fn disassemble(outpath: &Path, scenario: Scenario) -> anyhow::Result<()> {
    let mut parser = Parser::new(scenario);
    let mut sub_index = 0;
    let mut out = BufWriter::new(File::create_new(outpath)?);

    while Some(parser.state.ip) == parser.state.scenario.sub_offset(sub_index) {
        writeln!(out, "#LABEL_{:04}:", sub_index)?;
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
            "ED" | "RT" | "SRET" => (),
            _ => {
                let params = &parser.state.scenario.code[parser.state.ip..][..inst_info.param_len];
                parser.state.ip += inst_info.param_len;
                let params = display_bytes(params)?;
                fmt.param(params);
            }
        }

        fmt.write(&mut out)?;

        while Some(parser.state.ip) == parser.state.scenario.sub_offset(sub_index) {
            writeln!(out, "#LABEL_{:04}:", sub_index)?;
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

fn display_assetname(name: AssetName) -> Result<String, std::fmt::Error> {
    Ok(display)
}
