
	use std::slice::Iter;
	use project::Project;
	
	pub struct Storage {
		columns: Vec<String>,
		projects: Vec<Project>,
		filters: Vec<Box<dyn Fn(&Project) -> bool>>,
	}

	impl Storage {
		pub fn new(filters: Vec<Box<dyn Fn(&Project)->bool>>) -> Storage {
			Storage{
				columns: vec![],
				projects: vec![],
				filters,
				}
		}
		
		pub fn set_columns(&mut self, record: Vec<&str>) {
			for v in record.iter() {
				self.columns.push(v.to_string());
			}
		}
		
		pub fn get_columns(&self) -> Vec<String> {
			self.columns.clone()
		} 
		
		pub fn sort_by_start_date(&mut self) {
			self.projects.sort_by(|a, b| a.start_date.cmp(&b.start_date));
		}
		
		pub fn insert(&mut self, record: Vec<&str>) -> Result<(), String> {
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
	}



#[cfg(test)]
mod tests {
    use super::*;
    use test_util::*;

    #[test]
    fn storage_sort_by_start_date() {
    	let mut storage: Storage = Storage::new(vec![Box::new(|_p| true)]);
        storage.projects.append(&mut vec![get_project(1), get_project(2)]);

    	storage.sort_by_start_date();
    	assert_eq!(storage.projects.iter().map(|p| p.id.as_ref()).collect::<Vec<&str>>(), vec!["2", "1"]);
    }
    
    #[test]
    fn storage_insert_without_filter() {
    	let mut storage: Storage = Storage::new(vec![Box::new(|_p| true)]);
    	storage.insert(get_record(1)).unwrap();
    	storage.insert(get_record(2)).unwrap();
    	assert_eq!(storage.projects.len(), 2);
    }
    
    #[test]
    fn storage_insert_with_filter() {
    	let mut storage: Storage = Storage::new(vec![Box::new(|p| p.id == "2")]);
    	storage.insert(get_record(1)).unwrap();
    	storage.insert(get_record(2)).unwrap();
    	assert_eq!(storage.projects.len(), 1);
    	assert_eq!(storage.projects[0].id, "2");
    }
}
