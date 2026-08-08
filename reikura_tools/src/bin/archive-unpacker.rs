use std::{
    fs::File,
    io::{Read, Seek, SeekFrom::Start},
    path::Path,
};

use reikura_gdl::{
    ArchiveEntry,
    format::{drs::DrsArc, sm2mpx10::Sm2mpx10},
};

fn main() {
    let mut magic_buf = [0; 8];

    for arg in std::env::args().skip(1) {
        let out_dir = &format!("{arg}_out");

        let Ok(mut file) = File::open(&arg) else {
            eprintln!("failed to open {arg}");
            continue;
        };

        {
            if let Err(err) = file.read_exact(&mut magic_buf) {
                eprintln!("{arg} failed with err: {err}");
                continue;
            }
            if let Err(err) = file.rewind() {
                eprintln!("{arg} failed with err: {err}");
                continue;
            };
        }

        match &magic_buf {
            b"SM2MPX10" => {
                if let Ok(arc) = Sm2mpx10::parse(&mut file) {
                    unpack_entries(out_dir, arc.entries.into_iter(), &mut file);
                } else {
                    eprintln!("{arg}: invalid archive");
                    continue;
                };
            }
            b"SM2MPX20" => {
                eprintln!("{arg}: unsupported sm2mpx20 archive");
                continue;
            }
            _ => {
                if let Ok(arc) = DrsArc::parse(&mut file) {
                    unpack_entries(out_dir, arc.entries.into_iter(), &mut file);
                } else {
                    eprintln!("{arg}: invalid archive");
                    continue;
                }
            }
        }
    }
}

fn unpack_entries(
    out_path: impl AsRef<Path>,
    entries: impl Iterator<Item: TryInto<ArchiveEntry>>,
    arc_file: &mut File,
) {
    _ = std::fs::create_dir(&out_path);
    let out_path = out_path.as_ref();
    let mut buf = Vec::with_capacity(1 << 20);

    for entry in entries {
        let Ok(entry) = entry.try_into() else {
            continue;
        };

        let path = out_path.join(entry.filename);
        arc_file.seek(Start(entry.offset as _)).unwrap();

        if buf.len() < entry.length {
            buf.resize(entry.length, 0);
            let data = &mut buf[..entry.length];
            arc_file.read_exact(data).unwrap();
            std::fs::write(path, data).unwrap();
        }
    }
}
