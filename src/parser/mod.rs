use std::collections::HashMap;
use std::io::{BufRead, Write};
use storage::Storage;

const SEP: &str = "\t";
const COMMENT_SYMBOL: &str = "#";

// parse_tsv parses line by line in tsv format and puts into storage
// panics on errors
pub fn parse_tsv<R: BufRead>(reader: R, storage: &mut Storage) {
    let mut is_header = true;
    for (i, line) in reader.lines().enumerate() {
        let s = line.unwrap();
        // skip empty and comment lines
        if s.is_empty() || s.trim().starts_with(COMMENT_SYMBOL) {
            continue;
        }

        let record: Vec<&str> = s.split(SEP).map(|s| s.trim()).collect();

        if !is_header {
            storage.insert(record).unwrap_or_else(|err| {
                panic!("on line: {}, parse error: {}", i + 1, err);
            });
        } else {
            storage.set_columns(record);
            is_header = false;
        }
    }
}

// write_tsv writes tsv format storage data into writer
// panics on errors
pub fn write_tsv<W: Write>(mut writer: W, storage: &Storage) {
    let columns = storage.get_columns();
    writer.write(columns.join(SEP).as_bytes()).unwrap();
    writer.write(b"\n").unwrap();

    let mut vals: Vec<String> = Vec::with_capacity(8);
    let mut dic: HashMap<&str, String>;
    for p in storage.iter() {
        dic = p.as_dic();
        columns.iter().for_each(|c| {
            vals.push(dic.get(c as &str).unwrap().to_owned());
        });

        writer.write(vals.join(SEP).as_bytes()).unwrap();
        writer.write(b"\n").unwrap();
        vals.clear();
    }
}

#[cfg(test)]
mod tests {
	use std::str;
	
    use super::*;
	
    #[test]
    fn test_parse_tsv() {
        struct Data(&'static str, usize);

        let cases = vec![
			Data(
"# Input file contents are between the two lines marked with stars
Project	Description	Start date	Category	Responsible	Savings amount	Currency	Complexity
1	Harmonize Lactobacillus acidophilus sourcing	2014-01-01 00:00:00.000	Dairy	Daisy Milks	NULL	NULL	Simple"
			, 1),
			Data(
"Project	Description	Start date	Category	Responsible	Savings amount	Currency	Complexity
# comment line

1	Harmonize Lactobacillus acidophilus sourcing	2014-01-01 00:00:00.000	Dairy	Daisy Milks	NULL	NULL	Simple
2	Substitute Crème fraîche with evaporated milk in ice-cream products	2013-01-01 00:00:00.000	Dairy	Daisy Milks	141415.942696	EUR	Moderate"
			,2),
			Data(
"Complexity	Project	Description	Start date	Category	Responsible	Savings amount	Currency
Simple	1	Harmonize Lactobacillus acidophilus sourcing	2014-01-01 00:00:00.000	Dairy	Daisy Milks	NULL	NULL
Moderate	2	Substitute Crème fraîche with evaporated milk in ice-cream products	2013-01-01 00:00:00.000	Dairy	Daisy Milks	141415.942696	EUR",
			2),
		];
        let mut storage: Storage = Storage::new(vec![Box::new(|_p| true)]);

        for Data(input, expected) in cases {
            parse_tsv(Box::new(input.as_bytes()), &mut storage);

            assert_eq!(storage.size(), expected);

            storage.clear();
        }
    }

    #[test]
    fn test_write_tsv() {
    // column, projects, output
    struct Data(Vec<&'static str>, Vec<Vec<&'static str>>, &'static str);

        let cases = vec![
			Data(
			vec!["Project", "Description", "Start date", "Category", "Responsible", "Savings amount", "Currency", "Complexity"],
			vec![vec![
            "1",
            "Harmonize Lactobacillus acidophilus sourcing",
            "2014-01-01 00:00:00.000",
            "Dairy",
            "Daisy Milks",
            "NULL",
            "NULL",
            "Simple"],
            vec![
             "2",
            "Substitute Crème fraîche with evaporated milk in ice-cream products",
            "2013-01-01 00:00:00.000",
            "Dairy",
            "Daisy Milks",
            "141415.942696",
            "EUR",
            "Moderate"]],
"Project	Description	Start date	Category	Responsible	Savings amount	Currency	Complexity
1	Harmonize Lactobacillus acidophilus sourcing	2014-01-01 00:00:00.000	Dairy	Daisy Milks			Simple
2	Substitute Crème fraîche with evaporated milk in ice-cream products	2013-01-01 00:00:00.000	Dairy	Daisy Milks	141415.942696	EUR	Moderate\n"),
			Data(
			vec!["Complexity", "Project", "Description", "Start date", "Category", "Responsible", "Savings amount", "Currency"],
			vec![vec![
			"Simple",
            "1",
            "Harmonize Lactobacillus acidophilus sourcing",
            "2014-01-01 00:00:00.000",
            "Dairy",
            "Daisy Milks",
            "NULL",
            "NULL"],
            vec![
            "Moderate",
             "2",
            "Substitute Crème fraîche with evaporated milk in ice-cream products",
            "2013-01-01 00:00:00.000",
            "Dairy",
            "Daisy Milks",
            "141415.942696",
            "EUR"]],
"Complexity	Project	Description	Start date	Category	Responsible	Savings amount	Currency
Simple	1	Harmonize Lactobacillus acidophilus sourcing	2014-01-01 00:00:00.000	Dairy	Daisy Milks		\nModerate	2	Substitute Crème fraîche with evaporated milk in ice-cream products	2013-01-01 00:00:00.000	Dairy	Daisy Milks	141415.942696	EUR\n"),
		];
        let mut storage: Storage = Storage::new(vec![Box::new(|_p| true)]);

        for Data(columns, records, expected) in cases {
        	storage.set_columns(columns);
        	for rec in records.iter() {
        		storage.insert(rec.to_vec()).unwrap();
        	}

			let mut buf: Vec<u8> = Vec::new();
            write_tsv(&mut buf, &mut storage);

            assert_eq!(str::from_utf8(buf.as_slice()).unwrap(), expected);

            storage.clear();
        }
    }

}
