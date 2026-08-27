use std::{
    fs::File,
    io::{BufWriter, Cursor, Write},
    path::{Path, PathBuf},
};

use reikura_gdl::format::sm2mpx10::Sm2mpx10;

const ALIGN: u32 = 16;

fn main() {
    'arg: for ref arg in std::env::args().skip(1) {
        let path = Path::new(arg);

        let Ok(dir) = std::fs::read_dir(path) else {
            eprintln!("failed to read dir {arg}");
            continue;
        };

        let mut files = Vec::with_capacity(dir.size_hint().0);
        for entry in dir.filter_map(Result::ok) {
            if entry.metadata().is_ok_and(|md| md.is_file()) {
                files.push(entry.path());
            }
        }

        files.sort_by(|a, b| {
            a.file_name()
                .unwrap()
                .as_encoded_bytes()
                .iter()
                .map(|it| it.to_ascii_lowercase())
                .cmp(
                    b.file_name()
                        .unwrap()
                        .as_encoded_bytes()
                        .iter()
                        .map(|it| it.to_ascii_lowercase()),
                )
        });

        let name = {
            let mut buf = [0; 12];
            let filename = path.file_name().unwrap_or_default().as_encoded_bytes();
            let len = std::cmp::min(filename.len(), buf.len());
            buf[..len].copy_from_slice(&filename[..len]);
            buf
        };

        let Ok(header) = create_header(name, &files)
            .inspect_err(|err| eprintln!("failed to create header with err: {err}"))
        else {
            continue;
        };

        let out_path = {
            let out_dir = path.parent().unwrap().join("_pack");
            _ = std::fs::create_dir(&out_dir);
            out_dir.join(path.file_name().unwrap())
        };
        let Ok(mut writer) = File::create_new(&out_path).map(BufWriter::new) else {
            eprintln!("failed to create file {}", out_path.display());
            continue;
        };

        if let Err(err) = writer.write_all(&header) {
            eprintln!(
                "{} failed to write header with err: {err}",
                out_path.display()
            );
            continue;
        };

        for file in files {
            let Ok(data) = std::fs::read(&file) else {
                eprintln!("failed to read file {}", file.display());
                continue 'arg;
            };

            if let Err(err) = writer.write_all(&data) {
                eprintln!(
                    "{} failed to write data with err: {err}",
                    out_path.display()
                );
                continue 'arg;
            };

            let data_len = data.len();
            let padded_len = data_len.next_multiple_of(ALIGN as usize);
            if data_len != padded_len
                && let Err(err) = writer.write_all(&vec![0; padded_len - data_len])
            {
                eprintln!(
                    "{} failed to write data with err: {err}",
                    out_path.display()
                );
                continue 'arg;
            }
        }
    }
}

fn create_header(name: [u8; 12], entries: &[PathBuf]) -> std::io::Result<Vec<u8>> {
    let count = entries.len() as u32;
    let table_start: u32 = 32;
    let header_len = table_start + count * 20;
    let data_start = header_len.next_multiple_of(ALIGN);
    let mut header = Cursor::new(Vec::with_capacity(data_start as usize));

    header.write_all(Sm2mpx10::MAGIC)?;
    header.write_all(&count.to_le_bytes())?;
    header.write_all(&header_len.to_le_bytes())?;
    header.write_all(&name)?;
    header.write_all(&table_start.to_le_bytes())?;

    let mut name = name;
    let mut addr = data_start;
    for entry in entries {
        let size = entry.metadata()?.len() as u32;

        name.fill(0);
        let filename = entry.file_name().unwrap_or_default().as_encoded_bytes();
        if !filename.is_ascii() || filename.len() > 12 {
            return Err(std::io::Error::other(format!(
                "illegal filename {:?}, filename should only contain ascii and be less than 12 bytes",
                filename
            )));
        }
        name[..filename.len()].copy_from_slice(filename);

        header.write_all(&name)?;
        header.write_all(&addr.to_le_bytes())?;
        header.write_all(&size.to_le_bytes())?;

        addr = (addr + size).next_multiple_of(ALIGN);
    }

    let mut bytes = header.into_inner();
    // padding
    if bytes.len() != data_start as usize {
        bytes.resize(data_start as usize, 0);
    }

    Ok(bytes)
}
