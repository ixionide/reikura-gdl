use std::path::Path;

const FILTER_LEN: usize = 2048;

fn main() {
    for ref arg in std::env::args().skip(1) {
        let path = Path::new(arg);

        if path
            .extension()
            .is_none_or(|ext| !ext.eq_ignore_ascii_case("exe"))
        {
            eprintln!("{arg} is not an executable");
            continue;
        }

        let Ok(data) = std::fs::read(path) else {
            eprintln!("failed to read {arg}");
            continue;
        };
        let out = path.with_extension("filter");

        let Some(pos) = data
            .array_windows::<8>()
            .position(|slice| slice == b"UOB0GMVM")
        else {
            eprintln!("no filter is found in {arg}");
            continue;
        };

        let end = pos + FILTER_LEN;

        let Some(filter) = data
            .get(pos..end)
            .filter(|filter| filter.iter().all(|byte| byte.is_ascii_alphanumeric()))
        else {
            eprintln!("invalid filter is found in {arg}");
            continue;
        };

        if let Err(err) = std::fs::write(&out, filter) {
            eprintln!(
                "failed to write filter to file {}, with err: {err}",
                out.display()
            );
        }
    }
}
