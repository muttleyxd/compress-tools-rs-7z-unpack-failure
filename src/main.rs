use compress_tools::*;
use std::fs::File;
use std::path::Path;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        panic!("Usage: {} <7z archive>", args[0]);
    }

    let mut source = File::open(&args[1])?;
    let dest = Path::new("/tmp/dest");
    uncompress_archive(&mut source, &dest, Ownership::Ignore)?;

    Ok(())
}
