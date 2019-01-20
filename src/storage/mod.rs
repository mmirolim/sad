use project::{Project, COLUMN_NAMES};
use std::collections::HashMap;
use std::slice::Iter;

pub struct Storage {
    columns: HashMap<String, usize>,
    projects: Vec<Project>,
    filters: Vec<Box<dyn Fn(&Project) -> bool>>,
}

impl Storage {
    pub fn new(filters: Vec<Box<dyn Fn(&Project) -> bool>>) -> Storage {
        let mut storage = Storage {
            columns: HashMap::with_capacity(8),
            projects: vec![],
            filters,
        };
        storage.set_columns(COLUMN_NAMES.to_vec());

        storage
    }

    pub fn set_columns(&mut self, record: Vec<&str>) {
        for (i, v) in record.iter().enumerate() {
            self.columns.insert(v.to_string(), i);
        }
    }

    pub fn get_columns(&self) -> Vec<&str> {
        let mut columns = [""; 8];
        for (k, v) in self.columns.iter() {
            columns[*v as usize] = k;
        }

        columns.to_vec()
    }

    pub fn sort_by_start_date(&mut self) {
        self.projects
            .sort_by(|a, b| a.start_date.cmp(&b.start_date));
    }

    pub fn insert(&mut self, record: Vec<&str>) -> Result<(), String> {
        let record = COLUMN_NAMES
            .iter()
            .map(|c| {
                let id = self.columns.get(c as &str).unwrap();
                record[*id]
            })
            .collect();

        let p = Project::new(record)?;

        for f in self.filters.iter() {
            if !(f)(&p) {
                return Ok(());
            }
        }
        self.projects.push(p);
        Ok(())
    }

    pub fn iter(&self) -> Iter<Project> {
        self.projects.iter()
    }

    pub fn size(&self) -> usize {
        self.projects.len()
    }

    pub fn clear(&mut self) {
        self.columns.clear();
        self.projects.clear();
        self.filters.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_util::*;

    #[test]
    fn test_sort_by_start_date() {
        let mut storage: Storage = Storage::new(vec![Box::new(|_p| true)]);
        storage
            .projects
            .append(&mut vec![get_project(1), get_project(2)]);

        storage.sort_by_start_date();
        assert_eq!(
            storage
                .projects
                .iter()
                .map(|p| p.id.as_ref())
                .collect::<Vec<&str>>(),
            vec!["2", "1"]
        );
    }

    #[test]
    fn test_insert_without_filter() {
        let mut storage: Storage = Storage::new(vec![Box::new(|_p| true)]);

        storage.insert(get_record(1)).unwrap();
        storage.insert(get_record(2)).unwrap();
        assert_eq!(storage.projects.len(), 2);
    }

    #[test]
    fn test_insert_with_filter() {
        let mut storage: Storage = Storage::new(vec![Box::new(|p| p.id == "2")]);

        storage.insert(get_record(1)).unwrap();
        storage.insert(get_record(2)).unwrap();
        assert_eq!(storage.projects.len(), 1);
        assert_eq!(storage.projects[0].id, "2");
    }
}
