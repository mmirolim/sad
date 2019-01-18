extern crate chrono;

pub mod parser;
pub mod storage;
pub mod api;

pub mod project {
	use chrono::NaiveDateTime;
	use std::fmt;

	const NULL_STR: &str = "NULL";

	#[derive(Debug, Clone, PartialEq)]
	pub struct Project {
		pub id : String,
		pub description: String,
		pub start_date: NaiveDateTime,
		pub category: String,
		pub responsible: String,
		pub savings_amount: Option<Money>,
		pub currency: Option<Currency>,
		pub complexity: Complexity,
	}

	impl Project {
		pub fn new(record: Vec<&str>) -> Result<Project, String> {
		    if record.len() != 8 {
		    	return Err("wrong number of columns".to_string());
		    }
		    let start_date: NaiveDateTime;
		    
			if let Ok(v) = NaiveDateTime::parse_from_str(record[2], "%Y-%m-%d %H:%M:%S.%f") {
				start_date = v;
			} else {
				return Err(format!("start date parse error {}", record[2]));
			}
			
		    Ok(Project {
		        id: String::from(record[0]),
		        description: String::from(record[1]),
		        start_date,
		        category: String::from(record[3]),
		        responsible: String::from(record[4]),
		        savings_amount: Money::from_str(record[5])?,
		        currency: Currency::from_str(record[6])?,
		        complexity: Complexity::from_str(record[7])?,
		    })
		}
		
		pub fn as_vec(&self) -> Vec<String> {
			let savings_amount = match &self.savings_amount {
			    Some(ref v) => v.to_string(),
			    _ => NULL_STR.to_string(),
			};
			
			let currency = match &self.currency {
				Some(ref v) => v.to_string(),
				_ => NULL_STR.to_string(), 
			};
			
			vec![
			self.id.clone(),
			self.description.clone(),
			self.start_date.format("%Y-%m-%d %H:%M:%S.%3f").to_string(),
			self.category.clone(),
			self.responsible.clone(),
			savings_amount,
			currency,
			self.complexity.to_string(),
			]
		}
		
	}

	#[derive(Debug, Clone, PartialEq)]
	pub enum Currency {
		EUR
	}

	impl Currency {
		fn from_str(s: &str) -> Result<Option<Currency>, String> {
			match s {
				NULL_STR => Ok(None),
				"EUR" => Ok(Some(Currency::EUR)),
				_ => Err(format!("currency parse error {}", s)),
			}
		}

	}
	
	impl fmt::Display for Currency {
		fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
			write!(f, "{:?}", self)
		}
	}


	#[derive(Debug, Clone, PartialEq)]
	pub struct Money(f64);

	impl Money {
		pub fn new(f: f64) -> Money {
			Money(f)
		}
		fn from_str(s: &str) -> Result<Option<Money>, String> {
			match s {
				NULL_STR => Ok(None),
				_ => match s.parse() {
					Ok(v) => Ok(Some(Money(v))),
					Err(e) => return Err(format!("money parse error {}", e)), 
				}
			}
		}
		
	}

	impl fmt::Display for Money {
		fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
			write!(f, "{:.6}", self.0)
		}
	}

	#[derive(Debug, Clone, PartialEq)]
	pub enum Complexity {
		Simple,
		Moderate,
		Hazardous,
	}

	impl Complexity {
		fn from_str(s: &str) -> Result<Complexity, String> {
			match s {
				"Simple" => Ok(Complexity::Simple),
				"Moderate" => Ok(Complexity::Moderate),
				"Hazardous" => Ok(Complexity::Hazardous),
				_ => Err(format!("unsupported complexity {}", s).to_owned())
			}
		}
	}
	
	impl fmt::Display for Complexity {
		fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
			write!(f, "{:?}", self)
		}
	}
}

#[cfg(test)]
mod tests {
    use project::*;
	use test_util::*;    
   
    #[test]
    fn project_from_tsv_line() {
    	struct Data(Vec<&'static str>, Project);
    	let cases: Vec<Data> = vec![Data(get_record(1), get_project(1)),
        Data(get_record(2), get_project(2))];      
        
        for Data(input, expected) in cases {
        	assert_eq!(Project::new(input).unwrap(), expected);
        }
    }
}

pub mod test_util {
	use chrono::NaiveDate;
	use project::*;
	
	pub fn get_record(id: usize) -> Vec<&'static str> {
		let records = vec![
			vec!["1", "Harmonize Lactobacillus acidophilus sourcing", "2014-01-01 00:00:00.000", "Dairy", "Daisy Milks", "NULL", "NULL" ,"Simple"],
			vec!["2", "Substitute Crème fraîche with evaporated milk in ice-cream products", "2013-01-01 00:00:00.000", "Dairy", "Daisy Milks", "141415.942696", "EUR", "Moderate"],
		];
		
		records[id-1].clone()
	}
	
	pub fn get_project(id: usize) -> Project {
    	let projects: Vec<Project> = vec![
    	Project {
            id: String::from("1"),
            description: String::from("Harmonize Lactobacillus acidophilus sourcing"),
            start_date: NaiveDate::from_ymd(2014, 1, 1).and_hms_milli(0, 0, 0, 000),
            category: String::from("Dairy"),
            responsible: String::from("Daisy Milks"),
            savings_amount: None,
            currency: None,
            complexity: Complexity::Simple,
        },
    	Project{
        	id: String::from("2"),
        	description: String::from("Substitute Crème fraîche with evaporated milk in ice-cream products"),
        	start_date: NaiveDate::from_ymd(2013, 1, 1).and_hms_milli(0, 0, 0, 000),
            category: String::from("Dairy"),
            responsible: String::from("Daisy Milks"),
            savings_amount: Some(Money::new(141415.942696)),
            currency: Some(Currency::EUR),
            complexity: Complexity::Moderate,
        }];
        
        projects[id-1].clone()
    }

}
