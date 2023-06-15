use berg::{BionicTransformer, EpubDocument};
use std::{error::Error, fs, io::Write, time::Instant};
use std::env;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} source.epub dest.epub", args[0]);
        return Ok(());
    }

    let start = Instant::now();
    let mut epub = EpubDocument::open(&args[1])?;
    let mut out = fs::File::create(&args[2]).unwrap();


    epub.transform(BionicTransformer::new(), &mut out)?;

    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);

    out.flush()?;
    Ok(())
}
