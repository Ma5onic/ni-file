use crate::detect::NIFileType;
use std::io::Write;
use std::path::PathBuf;
use structopt::StructOpt;

pub(crate) fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("NI Extractor v0.0.1\n");

    let args = Args::from_args();
    let input = args.input;

    info!("Reading {:?}", input);

    let mut buffer = std::fs::read(input)?;

    if args.deflate {
        let (_, deflated) = crate::deflate::deflate(&buffer, 11).unwrap();
        let mut file = std::fs::File::create("output/arg.deflated").unwrap();
        file.write_all(&deflated).unwrap();
        buffer = deflated;
    }

    match crate::detect::filetype(&buffer) {
        NIFileType::NIContainer => crate::extractor::read(&buffer)?,
        NIFileType::NIKontaktMonolith => crate::monolith::read(&buffer)?,
        NIFileType::KoreSound => unimplemented!(),
        NIFileType::Unknown => panic!("unknown filetype!"),
    }

    if !args.extract {
        // std::fs::write(output, crate::ni_file::NIFile::from(segment).preset)?;
    }

    Ok(())
}

#[derive(StructOpt)]
#[structopt(name = "extract", about = "RSDK Extractor")]
struct Args {
    /// Input file
    #[structopt(parse(from_os_str))]
    input: PathBuf,

    /// Extract files
    #[structopt(long = "extract")]
    extract: bool,

    /// Instantly deflate file
    #[structopt(long = "deflate")]
    deflate: bool,
}
