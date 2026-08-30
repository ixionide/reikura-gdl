use std::{
    fmt::{Display, Write as _},
    fs::File,
    io::{BufWriter, Write as _},
    path::Path,
};

use anyhow::bail;
use reikura_gdl::{
    AssetName, Parser, Scenario,
    instruction::{CHARSET, INSTRUCTIONS, Instruction, InstructionInfo, ParamString, Value},
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
                let Some(exe_path) = args.next() else {
                    continue;
                };

                let Ok(executable) = std::fs::read(&exe_path) else {
                    eprintln!("failed to read executable {exe_path}");
                    continue;
                };

                deopfuscator = Deobfuscator::try_filter_search(&executable);
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
                if let Err(err) = disassemble(&outpath, scenario) {
                    _ = std::fs::remove_file(outpath);
                    eprintln!("failed to disassemble {arg}");
                    eprintln!("Error: {err}");
                };
            }
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
            "PM" => {
                let par: u8 = parser.read_param()?;
                fmt.add_param(par);

                'param: loop {
                    let cmd: u8 = parser.read_param()?;

                    match cmd {
                        0x00 => {
                            if let Some(0) = parser.peek_opcode() {
                                parser.state.ip += 1;
                            };

                            break 'param;
                        }
                        0x01 => {
                            fmt.add_param(cmd);

                            for _ in 0..4 {
                                let par: u8 = parser.read_param()?;
                                fmt.add_param(par);
                            }
                        }
                        0x02 | 0x03 | 0x06 => {
                            fmt.add_param(cmd);
                        }
                        0x04 => {
                            let par: u8 = parser.read_param()?;
                            fmt.add_param(cmd).add_param(par);
                        }
                        0x08 | 0x11 => {
                            let value = parser.read_param()?;
                            fmt.add_param(cmd).add_param(display_value(value));
                        }
                        0x13 => {
                            let asset = parser.read_param()?;
                            fmt.add_param(cmd).add_param(display_assetname(asset)?);
                        }
                        0xFF => {
                            let mut msg_buffer = Vec::with_capacity(inst_info.param_len - 1);

                            'msg: loop {
                                match parser.read_param::<u8>()? {
                                    0 => break 'msg,
                                    index @ 1..0x7F => {
                                        msg_buffer.extend(CHARSET[index as usize]);
                                    }
                                    0x7F => {
                                        let byte: u8 = parser.read_param()?;
                                        msg_buffer.push(byte);
                                    }
                                    byte @ 0x80.. => {
                                        msg_buffer.push(byte);
                                        let byte: u8 = parser.read_param()?;
                                        msg_buffer.push(byte);
                                    }
                                };
                            }

                            let message = sjis_to_utf8(&msg_buffer)?;
                            fmt.add_param(display_message(&message));
                        }
                        _ => {
                            eprint!("unknown PM cmd: {cmd}");
                            fmt.add_param(cmd);
                        }
                    }
                }
            }
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
            // "GSCRL"
            "GV" => {
                let par: u16 = parser.read_param()?;
                fmt.add_param(par);

                for _ in 0..2 {
                    let par: u8 = parser.read_param()?;
                    fmt.add_param(par);
                }

                let value = parser.read_param()?;
                fmt.add_param(display_value(value));
            }
            // "GAL"
            "GAOPEN" => {
                let value = parser.read_param()?;
                let asset = parser.read_param()?;
                fmt.add_param(display_value(value))
                    .add_param(display_assetname(asset)?);
            }
            // "GASET"
            // "GAPOS"
            // "GACLOSE"
            // "GADELETE"
            // "SGL"
            "ML" => {
                let asset = parser.read_param()?;
                let par: u8 = parser.read_param()?;
                fmt.add_param(display_assetname(asset)?).add_param(par);
            }
            "MP" => {
                let par: u8 = parser.read_param()?;
                fmt.add_param(par);

                if inst_info.param_len == 5 {
                    let value = parser.read_param()?;
                    fmt.add_param(display_value(value));
                }
            }
            "MF" => {
                let value = parser.read_param()?;
                fmt.add_param(display_value(value));
            }
            "MS" => (),
            "SER" => {
                let asset = parser.read_param()?;
                let value = parser.read_param()?;
                fmt.add_param(display_assetname(asset)?)
                    .add_param(display_value(value));
            }
            "SEP" => {
                let value = parser.read_param()?;
                fmt.add_param(display_value(value));

                if inst_info.param_len == 8 {
                    let value = parser.read_param()?;
                    fmt.add_param(display_value(value));
                }
            }
            "SED" => {
                let value = parser.read_param()?;
                fmt.add_param(display_value(value));
            }
            "PCMON" => {
                let par: u8 = parser.read_param()?;
                fmt.add_param(par);
            }
            "PCML" => {
                let asset = parser.read_param()?;
                fmt.add_param(display_assetname(asset)?);
            }
            "PCMS" => {
                let value = parser.read_param()?;
                fmt.add_param(display_value(value));
            }
            "PCMEND" => (),
            "SES" => {
                for _ in 0..2 {
                    let value = parser.read_param()?;
                    fmt.add_param(display_value(value));
                }
            }
            // "BGMGETPOS"
            // "SEGETPOS"
            // "PCMGETPOS"
            // "PCMCN"
            "IM" => {
                let par: u8 = parser.read_param()?;
                let asset = parser.read_param()?;
                fmt.add_param(par).add_param(display_assetname(asset)?);
            }
            "IC" => match inst_info.param_len {
                1 => {
                    let par: u8 = parser.read_param()?;
                    fmt.add_param(par);
                }
                4 => {
                    let value = parser.read_param()?;
                    fmt.add_param(display_value(value));
                }
                _ => eprintln!("unknown IC param len"),
            },
            // "IMS"
            "IXY" => {
                for _ in 0..2 {
                    let value = parser.read_param()?;
                    fmt.add_param(display_value(value));
                }
            }
            "IH" => {
                let par: u8 = parser.read_param()?;
                fmt.add_param(par);

                for _ in 0..4 {
                    let value = parser.read_param()?;
                    fmt.add_param(display_value(value));
                }

                let par1: u8 = parser.read_param()?;
                let par2: u16 = parser.read_param()?;
                fmt.add_param(par1).add_param(par2);

                for _ in 0..3 {
                    let par: u8 = parser.read_param()?;
                    fmt.add_param(par);
                }
            }
            "IG" => {
                for _ in 0..2 {
                    let par: u16 = parser.read_param()?;
                    fmt.add_param(par);
                }

                for _ in 0..2 {
                    let par: u8 = parser.read_param()?;
                    fmt.add_param(par);
                }
            }
            "IGINIT" | "IGRELEASE" => (),
            "IHK" => {
                let par: u8 = parser.read_param()?;
                fmt.add_param(par);

                for _ in 0..8 {
                    let value = parser.read_param()?;
                    fmt.add_param(display_value(value));
                }
            }
            "IHKDEF" => {
                let value = parser.read_param()?;
                fmt.add_param(display_value(value));
            }
            "IHGL" => {
                let asset = parser.read_param()?;
                fmt.add_param(display_assetname(asset)?);

                for _ in 0..2 {
                    let value = parser.read_param()?;
                    fmt.add_param(display_value(value));
                }
            }
            "IHGC" => (),
            // "IHGP"
            "CLK" => {
                let par: u8 = parser.read_param()?;
                let value = parser.read_param()?;
                fmt.add_param(par).add_param(display_value(value));
            }
            "IGN" => {
                let value = parser.read_param()?;
                fmt.add_param(display_value(value));
            }
            // "DAE"
            "DAP" => {
                let value = parser.read_param()?;
                let par: u8 = parser.read_param()?;
                fmt.add_param(display_value(value)).add_param(par);

                if inst_info.param_len == 11 {
                    let value = parser.read_param()?;
                    fmt.add_param(display_value(value));
                }
            }
            "DAS" => {
                let value = parser.read_param()?;
                fmt.add_param(display_value(value));
            }
            "SETINSIDEVOL" => {
                let par: u8 = parser.read_param()?;
                let value = parser.read_param()?;
                fmt.add_param(par).add_param(display_value(value));
            }
            // "KIDCLR"
            // "KIDMOJI"
            // "KIDPAGE"
            // "KIDSET"
            // "KITEND"
            "KIDFN" => {
                let par: u32 = parser.read_param()?;
                fmt.add_param(par);
            }
            "KIDHABA" => {
                let par: u8 = parser.read_param()?;
                fmt.add_param(par);

                for _ in 0..2 {
                    let par: u16 = parser.read_param()?;
                    fmt.add_param(par);
                }
            }
            "KIDSCAN" => {
                let par: u16 = parser.read_param()?;
                let value = parser.read_param()?;
                fmt.add_param(par).add_param(display_value(value));
            }
            "SETKIDWNDPUTPOS" | "SETMESWNDPUTPOS" => {
                let par: u8 = parser.read_param()?;
                fmt.add_param(par);

                for _ in 0..4 {
                    let value = parser.read_param()?;
                    fmt.add_param(display_value(value));
                }
            }
            // "INNAME"
            // "NAMECOPY"
            // "CHANGEWALL"
            // "MSGBOX"
            // "SETSMPRATE"
            // "CLKEXMCSET"
            // "IRCLK"
            // "IROPN"
            // "PPTL"
            // "PPABL"
            // "PPTYPE"
            // "PPORT"
            // "PPCRT"
            // "SABL"
            // "MPM"
            // "VOC"
            // "PM2"
            // "MPM2"
            "TAGSET" => {
                let par: u8 = parser.read_param()?;
                let string: ParamString = parser.read_param()?;
                fmt.add_param(par)
                    .add_param(display_string(&string.decode_sjis()?));
            }
            "FRAMESET" => {
                for _ in 0..2 {
                    let par: u8 = parser.read_param()?;
                    fmt.add_param(par);
                }

                let string: ParamString = parser.read_param()?;
                fmt.add_param(display_string(&string.decode_sjis()?));
            }
            "RBSET" | "CBSET" => {
                for _ in 0..3 {
                    let par: u8 = parser.read_param()?;
                    fmt.add_param(par);
                }

                let par: u16 = parser.read_param()?;
                let string: ParamString = parser.read_param()?;
                fmt.add_param(par)
                    .add_param(display_string(&string.decode_sjis()?));
            }
            "SLDRSET" => {
                for _ in 0..4 {
                    let par: u8 = parser.read_param()?;
                    fmt.add_param(par);
                }

                for _ in 0..3 {
                    let string: ParamString = parser.read_param()?;
                    fmt.add_param(display_string(&string.decode_sjis()?));
                }

                let par: u8 = parser.read_param()?;
                fmt.add_param(par);

                for _ in 0..3 {
                    let value = parser.read_param()?;
                    fmt.add_param(display_value(value));
                }

                let par: u8 = parser.read_param()?;
                fmt.add_param(par);

                for _ in 0..2 {
                    let par: u16 = parser.read_param()?;
                    fmt.add_param(par);
                }
            }
            "OPSL" => {
                let par: u8 = parser.read_param()?;
                fmt.add_param(par);
            }
            "OPPROP" => (),
            // "DISABLE"
            // "ENABLE"
            // "TITLE"
            // "EXT2"
            "CNF" => {
                let par: u8 = parser.read_param()?;
                let asset = parser.read_param()?;
                fmt.add_param(par).add_param(display_assetname(asset)?);
            }
            "ATIMES" => {
                let value = parser.read_param()?;
                fmt.add_param(display_value(value));
            }
            "AWAIT" => (),
            "AVIP" => {
                for _ in 0..4 {
                    let value = parser.read_param()?;
                    fmt.add_param(display_value(value));
                }

                let asset = parser.read_param()?;
                fmt.add_param(display_assetname(asset)?);
            }
            "PPF" | "SVF" => {
                let par: u8 = parser.read_param()?;
                fmt.add_param(par);
            }
            // "PPE"
            "SETGAMEINFO" => {
                let string: ParamString = parser.read_param()?;
                fmt.add_param(display_string(&string.decode_sjis()?));
            }
            "SETFONTSTYLE" => {
                for _ in 0..2 {
                    let par: u8 = parser.read_param()?;
                    fmt.add_param(par);
                }
            }
            "SETFONTCOLOR" => {
                for _ in 0..2 {
                    let par: u8 = parser.read_param()?;
                    fmt.add_param(par);
                }

                match inst_info.param_len {
                    5 => {
                        for _ in 0..3 {
                            let par: u8 = parser.read_param()?;
                            fmt.add_param(par);
                        }
                    }
                    14 => {
                        for _ in 0..3 {
                            let value = parser.read_param()?;
                            fmt.add_param(display_value(value));
                        }
                    }
                    _ => eprintln!("unknown SETFONTCOLOR param len"),
                }
            }
            "TIMERSET" => {
                let value = parser.read_param()?;
                fmt.add_param(display_value(value));
            }
            "TIMEREND" => {
                if inst_info.param_len == 4 {
                    let value = parser.read_param()?;
                    fmt.add_param(display_value(value));
                }
            }
            "TIMERGET" => {
                let par: u16 = parser.read_param()?;
                fmt.add_param(par);
            }
            // "GRPOUT"
            // "BREAK"
            // "EXT"
            _ => {
                let params = parser.read_bytes(inst_info.param_len)?;
                fmt.add_param(display_bytes(params)?);
            }
        }

        fmt.write(&mut out)?;

        let mut first = true;
        while Some(parser.state.ip) == parser.state.scenario.sub_offset(sub_index) {
            writeln!(
                out,
                "{}#LABEL_{sub_index:04}:",
                if first { "\n" } else { "" }
            )?;
            sub_index += 1;
            first = false;
        }
    }

    Ok(())
}

fn display_bytes(bytes: &[u8]) -> Result<String, std::fmt::Error> {
    let mut display = String::with_capacity(bytes.len());

    if bytes.is_empty() {
        return Ok(String::new());
    }

    for byte in bytes.iter() {
        if display.is_empty() {
            write!(display, "<{byte:02X}")?;
        } else {
            write!(display, " {byte:02X}")?;
        }
    }

    write!(display, ">")?;

    Ok(display)
}

fn display_string(string: &str) -> String {
    let mut display = String::with_capacity(string.len() + 8);

    display.push('"');

    for char in string.chars() {
        match char {
            '\"' => display.push_str("\\\""),
            '\\' => display.push_str("\\\\"),
            '\n' => display.push_str("\\n"),
            '\r' => display.push_str("\\r"),
            _ => display.push(char),
        }
    }

    display.push('"');

    display
}

fn display_message(message: &str) -> String {
    let mut display = String::with_capacity(message.len() + 16);

    display.push('[');

    for char in message.chars() {
        match char {
            '[' => display.push_str("\\["),
            ']' => display.push_str("\\]"),
            '\\' => display.push_str("\\\\"),
            '\n' => display.push_str("\\n"),
            '\r' => display.push_str("\\r"),
            _ => display.push(char),
        }
    }

    display.push(']');

    display
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

struct Formatter {
    mnemonic: String,
    params: String,
    err: Option<std::fmt::Error>,
}

impl Formatter {
    fn new(inst: &Instruction) -> Self {
        let mut mnemonic = inst.name.to_owned();

        if mnemonic.is_empty() {
            mnemonic = format!("<{:02X}>", inst.opcode);
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

        let result = write!(
            self.params,
            "{}{param}",
            if self.params.is_empty() { "" } else { ", " }
        );

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
