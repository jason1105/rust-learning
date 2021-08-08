
use std::collections::HashMap;


pub struct Organization {
    org: HashMap<String, Vec<String>>
}

impl Organization {

    pub fn initialize() -> Organization {
        Organization {
            org: HashMap::new()
        }
    }

    pub fn add_employee(&mut self, dept: &str, name: &str) {
        let depts = self.org.entry(dept.to_string()).or_insert(vec![]);

        depts.push(name.to_string());

    }
    
    pub fn list_employee(&self, department: &str) {
        println!("Employees of Department {} :", department);

        if let Some(emps) = self.org.get(department) {
            for emp in emps {
                println!("{}", emp);
            }

        }
    }
    
    pub fn list_all(&self) {

        println!("All of Employees:");

        for (department, emps) in &self.org {

            for emp in emps {
                println!("{}: {}", department, emp);
            }

        }

    }
}

