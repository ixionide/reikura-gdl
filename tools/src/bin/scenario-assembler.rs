use std::{
    collections::HashMap,
    fmt::Display,
    fs::File,
    io::{BufWriter, Cursor, Write},
    path::Path,
    str::FromStr,
};

use anyhow::{Context, anyhow, bail};
use reikura_gdl::{
    AssetName,
    format::isf::IsfMetadata,
    instruction::{INSTRUCTIONS, Value},
};
use reikura_util::io::{WriteEndian, WriteExt};

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
        let mut labels = parse_all_labels(scenario)?;
        let mut code = Cursor::new(Vec::with_capacity(1 << 20));
        let mut code_len: usize = 0;

        for (line_num, line) in scenario.lines().map(str::trim).enumerate() {
            let ctx = || format!("Error at line no {line_num}");

            if line.is_empty() || line.starts_with(';') {
                continue;
            }

            if let Some(label) = line.strip_circumfix('#', ':') {
                labels.new_subroutine(label, code_len).with_context(ctx)?;
                continue;
            }

            let (inst, mut parser) = parse_line(line);

            let Some(opcode) = opcodes.get(inst).copied() else {
                return Err(anyhow::anyhow!("invalid instruction {inst}")).with_context(ctx);
            };

            let mut fmt = Serializer::new(opcode);

            match inst {
                "ED" | "SRET" | "RT" => (),
                "LS" | "LSBS" => {
                    let asset = parser.read_assetname()?;
                    fmt.write_string(&asset);
                }
                "JP" | "JS" => {
                    let label = labels.get_label_index(&parser.read_label()?)?;
                    fmt.write_num(label);
                }
                "ONJP" | "ONJS" => {
                    let switch = parser.read_value()?;
                    fmt.write_value(switch);

                    let mut branches = Vec::new();
                    while let Ok(label) = parser.read_label() {
                        let label = labels.get_label_index(&label)?;
                        branches.push(label);
                    }

                    let Ok(branches_count) = u8::try_from(branches.len()) else {
                        return Err(anyhow::anyhow!(
                            "maximum branch count exceeded {}, max is 255",
                            branches.len()
                        ))
                        .with_context(ctx);
                    };

                    fmt.write_num(branches_count);
                    for branch in branches {
                        fmt.write_num(branch);
                    }
                }
                "CW" => {
                    let par1: u8 = parser.read_num()?;
                    fmt.write_num(par1);

                    for _ in 0..4 {
                        let value = parser.read_value()?;
                        fmt.write_value(value);
                    }

                    let cmd: u8 = parser.read_num()?;
                    fmt.write_num(cmd);
                }
                "CP" | "CIR" => {
                    for _ in 0..2 {
                        let par: u8 = parser.read_num()?;
                        fmt.write_num(par);
                    }

                    let asset = parser.read_assetname()?;
                    fmt.write_string(&asset);
                }
                "CPS" => {
                    let par: u8 = parser.read_num()?;
                    fmt.write_num(par);

                    for _ in 0..6 {
                        let r: u8 = parser.read_num()?;
                        let g: u8 = parser.read_num()?;
                        let b: u8 = parser.read_num()?;
                        fmt.write_num(r).write_num(g).write_num(b);
                    }
                }
                // "CIP"
                "CSET" => {
                    for _ in 0..2 {
                        let par: u8 = parser.read_num()?;
                        fmt.write_num(par);
                    }

                    for _ in 0..4 {
                        let value = parser.read_value()?;
                        fmt.write_value(value);
                    }

                    let string = parser.read_string()?;
                    fmt.write_string(&string);
                }
                "CWO" => {
                    let par1: u8 = parser.read_num()?;
                    let value = parser.read_value()?;
                    let par2: u8 = parser.read_num()?;
                    fmt.write_num(par1).write_value(value).write_num(par2);
                }
                "CWC" => {
                    let par1: u8 = parser.read_num()?;
                    fmt.write_num(par1);
                }
                "CC" => {
                    for _ in 0..4 {
                        let par1: u8 = parser.read_num()?;
                        fmt.write_num(par1);
                    }
                }
                // "CCLR"
                // "RESET"
                // "CRND"
                // "CTEXT"
                "WS" => {
                    let par: u8 = parser.read_num()?;
                    fmt.write_num(par);

                    for _ in 0..4 {
                        let value = parser.read_value()?;
                        fmt.write_value(value);
                    }

                    let par: u8 = parser.read_num()?;
                    fmt.write_num(par);
                }
                // "WP"
                "WL" => {
                    let par: u8 = parser.read_num()?;
                    let asset = parser.read_assetname()?;
                    fmt.write_num(par).write_string(&asset);
                }
                "WW" => {
                    let par: u8 = parser.read_num()?;
                    fmt.write_num(par);

                    for _ in 0..2 {
                        let value = parser.read_value()?;
                        fmt.write_value(value);
                    }

                    let par: u8 = parser.read_num()?;
                    fmt.write_num(par);
                }
                // "CN"
                "CNS" => {
                    for _ in 0..2 {
                        let par: u8 = parser.read_num()?;
                        fmt.write_num(par);
                    }

                    let name = parser.read_string()?;
                    fmt.write_string(&name);
                }
                "PF" | "PB" | "PJ" => {
                    let par: u8 = parser.read_num()?;
                    let value = parser.read_value()?;
                    fmt.write_num(par).write_value(value);
                }
                "WO" | "WC" => {
                    let par: u8 = parser.read_num()?;
                    fmt.write_num(par);
                }
                // PM
                // "PMP"
                "WSH" | "WSS" => {
                    let value = parser.read_value()?;
                    fmt.write_value(value);
                }
                "FLN" => {
                    let par: u16 = parser.read_num()?;
                    fmt.write_num(par);
                }
                "SK" => {
                    let par1: u16 = parser.read_num()?;
                    let par2: u8 = parser.read_num()?;
                    fmt.write_num(par1).write_num(par2);
                }
                "SKS" => {
                    let par1: u16 = parser.read_num()?;
                    let par2: u16 = parser.read_num()?;
                    let par3: u8 = parser.read_num()?;
                    fmt.write_num(par1).write_num(par2).write_num(par3);
                }
                "HF" => {
                    let par: u16 = parser.read_num()?;
                    let label = parser.read_label()?;
                    fmt.write_num(par)
                        .write_num(labels.get_label_index(&label)?);
                }
                "FT" => {
                    for _ in 0..3 {
                        let par: u16 = parser.read_num()?;
                        fmt.write_num(par);
                    }
                }
                // "SP"
                // "STS"
                "ES" | "EC" => {
                    for _ in 0..2 {
                        let par: u16 = parser.read_num()?;
                        fmt.write_num(par);
                    }
                }
                "STC" => {
                    for _ in 0..2 {
                        let par: u8 = parser.read_num()?;
                        fmt.write_num(par);
                    }

                    let label = parser.read_label()?;
                    fmt.write_num(labels.get_label_index(&label)?);
                }
                "HN" => {
                    let par: u16 = parser.read_num()?;
                    let label = parser.read_label()?;
                    fmt.write_num(par)
                        .write_num(labels.get_label_index(&label)?);
                }
                "HXP" => {
                    for _ in 0..2 {
                        let par: u8 = parser.read_num()?;
                        fmt.write_num(par);
                    }

                    let label = parser.read_label()?;
                    fmt.write_num(labels.get_label_index(&label)?);
                }
                "HLN" => {
                    let par: u16 = parser.read_num()?;
                    fmt.write_num(par);
                }
                "HS" => {
                    let par: u16 = parser.read_num()?;
                    let value = parser.read_value()?;
                    fmt.write_num(par).write_value(value);
                }
                "HINC" | "HDEC" => {
                    let par: u16 = parser.read_num()?;
                    fmt.write_num(par);
                }
                // "CALC"
                _ => {
                    let params = parser.params;

                    if let Some(params) = params.strip_circumfix('<', '>') {
                        let mut bytes = Vec::with_capacity(params.len() / 3);

                        for str in params.split_ascii_whitespace() {
                            let byte = u8::from_str_radix(str, 16)?;
                            bytes.push(byte);
                        }

                        fmt.write_bytes(&bytes);
                    }

                    return Err(anyhow::anyhow!(
                        "failed to assemble unimplemented instruction {inst}"
                    ))
                    .with_context(ctx);
                }
            }

            // make sure we get all the param read
            let param_read = parser.param_read;
            if !parser.is_exhausted() {
                eprintln!(
                    "Warning: expected param count {param_read} got {}",
                    parser.param_count()
                );
            }

            let written = fmt.serialize_into(&mut code)?;
            code_len += written;
        }

        let code_offset = 8 + (labels.subroutines.len() * size_of::<u32>());
        let md = IsfMetadata {
            bytecode_offset: code_offset as u32,
            version: [0x95, 0x97],
            xor_key: 0,
            _reserved: 0,
        };

        let mut out = BufWriter::new(File::create_new(outpath)?);

        out.put_le(md.bytecode_offset)?;
        out.put_le(md.version)?;
        out.put_le(md.xor_key)?;
        out.put_le(md._reserved)?;

        fn encrypt(data: &mut [u8]) {
            data.iter_mut().for_each(|byte| *byte = byte.rotate_left(2));
        }

        for sub in labels.subroutines {
            let mut bytes = sub.to_le_bytes();
            encrypt(&mut bytes);
            out.put_bytes(bytes)?;
        }

        let mut code = code.into_inner();
        encrypt(&mut code);
        out.put_bytes(code)?;

        Ok(())
    }
}

fn parse_line(line: &str) -> (&str, ParamParser<'_>) {
    let Some((mnemonic, params)) = line.split_once(' ') else {
        return (line, ParamParser::new(""));
    };

    (mnemonic.trim(), ParamParser::new(params.trim()))
}

fn parse_all_labels(scenario: &str) -> anyhow::Result<Labels> {
    let mut labels = Labels::new();

    for (line_num, line) in scenario.lines().map(str::trim).enumerate() {
        let ctx = || format!("Error at line no {line_num}");

        if let Some(label) = line.strip_circumfix('#', ':') {
            labels.new_label(label).with_context(ctx)?;
        }
    }

    Ok(labels)
}

struct Labels {
    tables: HashMap<String, usize>,
    subroutines: Vec<u32>,
}

impl Labels {
    fn new() -> Self {
        Self {
            tables: HashMap::new(),
            subroutines: Vec::new(),
        }
    }

    fn new_label(&mut self, label: &str) -> anyhow::Result<()> {
        let dup = self.tables.insert(label.to_owned(), self.subroutines.len());
        self.subroutines.push(0);

        if dup.is_some() {
            bail!("duplicate label {label}");
        }

        Ok(())
    }

    fn get_label_index(&self, label: &str) -> anyhow::Result<u16> {
        self.tables
            .get(label)
            .map(|index| *index as u16)
            .ok_or_else(|| anyhow!("undefined label {label}"))
    }

    fn new_subroutine(&mut self, label: &str, offset: usize) -> anyhow::Result<()> {
        if let Some(idx) = self.tables.get(label).copied() {
            self.subroutines[idx] = offset as u32;
        } else {
            bail!("label {label} not found");
        }

        Ok(())
    }
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

    fn write_num<T: WriteEndian>(&mut self, param: T) -> &mut Self {
        self.params.put_le(param).unwrap();
        self
    }

    fn write_string(&mut self, param: &str) -> &mut Self {
        // TODO: encode to sjis
        self.write_bytes(param.as_bytes());
        self
    }

    fn write_bytes(&mut self, params: &[u8]) -> &mut Self {
        self.params.extend_from_slice(params);
        self
    }

    fn write_value(&mut self, value: Value) -> &mut Self {
        match value {
            Value::Literal(num) => self.params.put_le(num).unwrap(),
            Value::Register(idx) => {
                let idx = idx as u32;
                self.params.put_le(idx | Value::REG_TAG).unwrap();
            }
            Value::Random(max) => {
                let max = max | Value::RNG_TAG as i32;
                self.params.put_le(max).unwrap();
            }
        }

        self
    }

    fn serialize_into(self, writer: &mut impl Write) -> std::io::Result<usize> {
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

struct ParamParser<'a> {
    params: &'a str,
    param_read: usize,
}

impl<'a> ParamParser<'a> {
    fn new(params: &'a str) -> Self {
        Self {
            params,
            param_read: 0,
        }
    }

    fn next(&mut self) -> Option<&str> {
        if let Some((param, rest)) = self.params.split_once(',') {
            self.params = rest.trim();
            return Some(param.trim());
        }

        let param = self.params;

        if param.is_empty() {
            return None;
        }

        self.params = "";
        self.param_read += 1;
        Some(param)
    }

    fn read_num<T>(&mut self) -> anyhow::Result<T>
    where
        T: FromStr,
        <T as FromStr>::Err: Display,
    {
        let param = self
            .next()
            .ok_or_else(|| anyhow!("unexpected end of param"))?;
        T::from_str(param).map_err(|err| anyhow!("{err}"))
    }

    fn read_assetname(&mut self) -> anyhow::Result<String> {
        let string = self.read_string()?;

        if string.as_bytes().len() > AssetName::LEN {
            bail!(
                "asset name is too long: {string}, max length is {} bytes",
                AssetName::LEN
            );
        }

        Ok(string)
    }

    fn read_string(&mut self) -> anyhow::Result<String> {
        let strip = self
            .params
            .strip_prefix('"')
            .ok_or_else(|| anyhow!("param is not a string"))?;

        let mut string = String::with_capacity(CAP);
        let mut escaped = false;
        let mut char_indices = strip.char_indices();
        let end = loop {
            let (i, chr) = char_indices
                .next()
                .ok_or_else(|| anyhow!("unexpected end of string"))?;

            if escaped {
                match chr {
                    '"' | '\\' => string.push(chr),
                    'n' => string.push('\n'),
                    'r' => string.push('\r'),
                    _ => bail!("invalid escape character {chr}"),
                }
                escaped = false;
            } else {
                match chr {
                    '"' => break i,
                    '\\' => escaped = true,
                    _ => string.push(chr),
                }
            }
        };

        self.params = &strip[end..];
        let _ = self.next();
        Ok(string)
    }

    fn read_label(&mut self) -> anyhow::Result<String> {
        let param = self
            .next()
            .ok_or_else(|| anyhow!("unexpected end of param"))?;

        if param.starts_with(|a: char| a.is_numeric() || a == '"') {
            bail!("invalid label {param}");
        }

        Ok(param.to_owned())
    }

    fn read_value(&mut self) -> anyhow::Result<Value> {
        let param = self
            .next()
            .ok_or_else(|| anyhow!("unexpected end of param"))?;

        let value = {
            if let Some(idx) = param.strip_prefix('@') {
                idx.parse().map(Value::Register).context("parse register")?
            } else if let Some(max) = param.strip_prefix('~') {
                max.parse().map(Value::Random).context("parse random")?
            } else if let Ok(num) = param.parse::<i32>() {
                Value::Literal(num)
            } else {
                bail!("invalid value param: {param}");
            }
        };

        Ok(value)
    }

    fn is_exhausted(&mut self) -> bool {
        self.next().is_none()
    }

    fn param_count(&mut self) -> usize {
        while let Some(_) = self.next() {}
        self.param_read
    }
}

#[test]
fn test_param_parser() {
    let mut parser = ParamParser::new(
        r#"
            989,"Test", "Foo\nBar", "Foo\s", 87, 54535,-2
        "#
        .trim(),
    );

    assert_eq!(parser.next().unwrap(), "989");
    assert_eq!(parser.read_string().unwrap(), "Test");
    assert_eq!(parser.read_string().unwrap(), "Foo\nBar");
    assert!(parser.read_string().is_err());
    assert_eq!(parser.next().unwrap(), "\"Foo\\s\"");
    assert_eq!(parser.read_num::<u8>().unwrap(), 87);
    assert_eq!(parser.read_num::<u16>().unwrap(), 54535);
    assert_eq!(parser.read_num::<i32>().unwrap(), -2);

    let mut parser = ParamParser::new(
        r#"
            @78,%-12,%99,7892,   -232
        "#
        .trim(),
    );
    assert_eq!(parser.read_value().unwrap(), Value::Register(78));
    assert_eq!(parser.read_value().unwrap(), Value::Random(-12));
    assert_eq!(parser.read_value().unwrap(), Value::Random(99));
    assert_eq!(parser.read_value().unwrap(), Value::Literal(7892));
    assert_eq!(parser.read_value().unwrap(), Value::Literal(-232));
}
