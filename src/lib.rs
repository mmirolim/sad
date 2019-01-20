extern crate chrono;

pub mod api;
pub mod parser;
pub mod storage;

pub mod project {
    use chrono::NaiveDateTime;
    use std::collections::HashMap;
    use std::fmt;

    pub const NULL_STR: &str = "NULL";
    pub const COLUMN_NAMES: [&'static str; 8] = [
        "Project",
        "Description",
        "Start date",
        "Category",
        "Responsible",
        "Savings amount",
        "Currency",
        "Complexity",
    ];

    #[derive(Debug, Clone, PartialEq)]
    pub struct Project {
        pub id: String,
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
                // TODO rm
                println!("new record {:?}", record);
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

        pub fn as_dic(&self) -> HashMap<&str, String> {
            let mut dic: HashMap<&str, String> = HashMap::with_capacity(8);

            let savings_amount = match &self.savings_amount {
                Some(ref v) => v.to_string(),
                _ => String::from(""),
            };

            let currency = match &self.currency {
                Some(ref v) => v.to_string(),
                _ => String::from(""),
            };

            dic.insert(COLUMN_NAMES[0], self.id.clone());
            dic.insert(COLUMN_NAMES[1], self.description.clone());
            dic.insert(
                COLUMN_NAMES[2],
                self.start_date.format("%Y-%m-%d %H:%M:%S.%3f").to_string(),
            );
            dic.insert(COLUMN_NAMES[3], self.category.clone());
            dic.insert(COLUMN_NAMES[4], self.responsible.clone());
            dic.insert(COLUMN_NAMES[5], savings_amount);
            dic.insert(COLUMN_NAMES[6], currency);
            dic.insert(COLUMN_NAMES[7], self.complexity.to_string());

            dic
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Currency {
        EUR,
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
                },
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
                _ => Err(format!("unsupported complexity {}", s).to_owned()),
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
    use std::collections::HashMap;
    use test_util::*;

    #[test]
    fn project_from_tsv_line() {
        struct Data(Vec<&'static str>, Project);
        let cases = vec![
            Data(get_record(1), get_project(1)),
            Data(get_record(2), get_project(2)),
        ];

        for Data(input, expected) in cases {
            assert_eq!(Project::new(input).unwrap(), expected);
        }
    }

    #[test]
    fn project_as_dic() {
        struct Data(Project, HashMap<&'static str, String>);
        let cases = vec![
            Data(get_project(1), get_project_as_dic(1)),
            Data(get_project(2), get_project_as_dic(2)),
        ];

        for Data(input, expected) in cases {
            assert_eq!(input.as_dic(), expected);
        }
    }
}

pub mod test_util {
    use chrono::NaiveDate;
    use project::*;
    use std::collections::HashMap;

    pub fn get_project_as_dic(id: usize) -> HashMap<&'static str, String> {
        let mut p1: HashMap<&str, String> = HashMap::new();
        let mut p2: HashMap<&str, String> = HashMap::new();

        let vals1 = vec![
            "1",
            "Harmonize Lactobacillus acidophilus sourcing",
            "2014-01-01 00:00:00.000",
            "Dairy",
            "Daisy Milks",
            "",
            "",
            "Simple",
        ];
        let vals2 = vec![
            "2",
            "Substitute Crème fraîche with evaporated milk in ice-cream products",
            "2013-01-01 00:00:00.000",
            "Dairy",
            "Daisy Milks",
            "141415.942696",
            "EUR",
            "Moderate",
        ];
        for (i, v) in vals1.iter().enumerate() {
            p1.insert(COLUMN_NAMES[i], v.to_string());
        }

        for (i, v) in vals2.iter().enumerate() {
            p2.insert(COLUMN_NAMES[i], v.to_string());
        }

        let records = vec![p1, p2];

        records[id - 1].clone()
    }
    pub fn get_record(id: usize) -> Vec<&'static str> {
        let records = vec![
            vec![
                "1",
                "Harmonize Lactobacillus acidophilus sourcing",
                "2014-01-01 00:00:00.000",
                "Dairy",
                "Daisy Milks",
                "NULL",
                "NULL",
                "Simple",
            ],
            vec![
                "2",
                "Substitute Crème fraîche with evaporated milk in ice-cream products",
                "2013-01-01 00:00:00.000",
                "Dairy",
                "Daisy Milks",
                "141415.942696",
                "EUR",
                "Moderate",
            ],
        ];

        records[id - 1].clone()
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
            Project {
                id: String::from("2"),
                description: String::from(
                    "Substitute Crème fraîche with evaporated milk in ice-cream products",
                ),
                start_date: NaiveDate::from_ymd(2013, 1, 1).and_hms_milli(0, 0, 0, 000),
                category: String::from("Dairy"),
                responsible: String::from("Daisy Milks"),
                savings_amount: Some(Money::new(141415.942696)),
                currency: Some(Currency::EUR),
                complexity: Complexity::Moderate,
            },
        ];

        projects[id - 1].clone()
    }

}
