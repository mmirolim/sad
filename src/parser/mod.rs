
	use storage::Storage;
	use std::io::{BufRead, Write};
	
	const SEP: &str = "\t";
	const COMMENT_SYMBOL: &str = "#";
	
	// TODO comment that it panics
	pub fn parse_tsv(reader: Box<dyn BufRead>, storage: &mut Storage) {
		let mut is_header = true;
		for line in reader.lines() {
			let s = line.unwrap();
			// skip empty and comment lines
			if s.is_empty() || s.trim().starts_with(COMMENT_SYMBOL) {
				continue;
			}
			
			let record: Vec<&str> = s.split(SEP).
			        map(|s| s.trim()).collect();
			
			if !is_header {
				storage.insert(record).unwrap_or_else(|err| {
					panic!("row parsing error {}", err);
				});
			} else {
				storage.set_columns(record);
			    is_header = false;
			}
		}
	}
	
	pub fn write_tsv(mut writer: Box<dyn Write>, storage: &Storage) {
		writer.write(storage.get_columns().join(SEP).as_bytes()).unwrap();
		writer.write(b"\n").unwrap();
		
		for p in storage.iter() {
			writer.write(p.as_vec().join(SEP).as_bytes()).unwrap();
			writer.write(b"\n").unwrap();
		}
	}

