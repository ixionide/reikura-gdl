use std::{collections::HashMap, fs::File, io::BufWriter, path::Path};

use reikura_gdl::instruction::INSTRUCTIONS;
use reikura_util::io::WriteExt;

const CAP: usize = 256;

fn main() {
    let mut opcodes = HashMap::with_capacity(256);

    for inst in INSTRUCTIONS {
        if inst.name.is_empty() {
            continue;
        }

        let dup = opcodes.insert(inst.name.to_owned(), inst.opcode);
        assert!(dup.is_none(), "duplicate instruction {}", inst.name);
    }

    for ref arg in std::env::args().skip(1) {
        let path = Path::new(arg);

        let Ok(scenario) = std::fs::read_to_string(path)
            .inspect_err(|err| eprintln!("failed to read file {arg} with err: {err}"))
        else {
            continue;
        };

        let outpath = path.with_extension("isf");
        if let Err(err) = assemble(&outpath, &scenario, &opcodes) {
            _ = std::fs::remove_file(outpath);
            eprintln!("failed to assemble {arg}");
            eprintln!("Error: {err}");
        }
    }

    fn assemble(
        outpath: &Path,
        scenario: &str,
        opcodes: &HashMap<String, u8>,
    ) -> anyhow::Result<()> {
        let mut labels: HashMap<String, usize> = HashMap::with_capacity(CAP);
        let mut subroutines = Vec::with_capacity(CAP);
        let mut out = BufWriter::new(File::create_new(outpath)?);
        let mut out_len: usize = 0;

        for (line_num, line) in scenario.lines().map(str::trim).enumerate() {
            if line.is_empty() {
                continue;
            }

            if let Some(label) = line.strip_circumfix('#', ':') {
                labels.insert(label.to_owned(), subroutines.len());
                subroutines.push(out_len);
                continue;
            }

            let (inst, params) = parse_line(line);

            let Some(opcode) = opcodes.get(inst).copied() else {
                anyhow::bail!("invalid instruction {inst} at line {line_num}");
            };

            let mut serializer = Serializer::new(opcode);

            let _parser = ParamParser::new(params);
            match inst {
                "ED" | "SRET" | "RT" => (),
                "LS" | "LSBS" => {
                    // let asset = parser.read_param()?;
                    // fmt.add_param(display_assetname(asset)?);
                }
                _ => anyhow::bail!("cannot assemble instruction {inst}: unimplemented"),
            }

            if params.is_empty() {
                out.put_le(2_u8)?;
            }

            let written = serializer.serialize_into(&mut out)?;
            out_len += written;
        }

        Ok(())
    }
}

fn parse_line(line: &str) -> (&str, &str) {
    let Some((mnemonic, params)) = line.split_once(' ') else {
        return (line, "");
    };

    (mnemonic.trim(), params.trim())
}

struct Serializer {
    opcode: u8,
    params: Vec<u8>,
}

impl Serializer {
    fn new(opcode: u8) -> Self {
        Self {
            opcode,
            params: Vec::with_capacity(CAP),
        }
    }

    fn add_param(&mut self, params: &[u8]) {
        self.params.extend_from_slice(params);
    }

    fn serialize_into(self, writer: &mut BufWriter<File>) -> std::io::Result<usize> {
        writer.put_le(self.opcode)?;
        let mut inst_len = self.params.len() + 2; // plus the opcode and the len itself;

        if inst_len > 0x7f {
            inst_len += 1; // the len is two bytes here
            writer.put_le(0x80_00 | u16::try_from(inst_len).unwrap())?;
        } else {
            writer.put_le(u8::try_from(inst_len).unwrap())?;
        };

        writer.put_bytes(self.params)?;

        Ok(inst_len)
    }
}

#[allow(unused)]
struct ParamParser<'a>(&'a str);

#[allow(unused)]
impl<'a> ParamParser<'a> {
    fn new(params: &'a str) -> Self {
        Self(params)
    }

    #[allow(unused)]
    fn next_param(&mut self) -> Option<&str> {
        if let Some((param, rest)) = self.0.split_once(',') {
            self.0 = rest.trim();
            return Some(param.trim());
        }

        let param = self.0;

        if param.is_empty() {
            return None;
        }

        self.0 = "";
        Some(param)
    }

    // fn read_string(&mut self) -> {}
}
