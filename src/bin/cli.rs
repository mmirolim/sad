extern crate sad;

use std::fs::File;
use std::io::prelude::*;
use std::io::{stdout, BufReader, SeekFrom, BufWriter};
use std::error::Error;

use sad::api::Cli;
use sad::storage::Storage;
use sad::parser::{parse_tsv, write_tsv};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    // configure cli
    let opts = Cli::new(&args);
    
    let mut file = File::open(&opts.file)?;
	// check for BOM
	let mut buf = [0; 3];
	file.read(&mut buf)?;
	
	if buf != "\u{feff}".as_bytes() {
		file.seek(SeekFrom::Start(0))?;
	}
	// init storage
	let project_id = opts.project;
	let mut storage = Storage::new(vec![Box::new(move |p| {
		if let Some(id) = &project_id {
			return id == &p.id;
		}
		true
	})]);
	// parse data into storage
	parse_tsv(Box::new(BufReader::new(file)), &mut storage);
	
	if opts.sort_by_start_date {
		storage.sort_by_start_date();
	}

	// output data
	write_tsv(Box::new(BufWriter::new(stdout())), &storage);
	
	Ok(())
}

