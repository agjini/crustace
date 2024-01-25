use std::collections::HashMap;
use std::ptr::hash;

fn main() {
    let mut employees = vec!["Jean", "Marc", "Sally"];
}

struct Company {
    people_by_department: HashMap<String, Vec<String>>,
}

impl Company {
    pub fn get_people_by_department(&self) -> String {
        return format!(
            "Sales : {}",
            self.people_by_department.get("Sales").unwrap().join(", ")
        );
        // let mut people_by_department: HashMap<String, Vec<String>> = HashMap::new();
        // people_by_department = self
        //     .people_by_department
        //     .iter()
        //     .filter(|e| e.1 == department)
        //     .collect();
        // return format!("{} : {}", people_by_department);
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn get_people_by_department_when_empty() {
        let company = Company {
            people_by_department: HashMap::new(),
        };
        assert_eq!("", company.get_people_by_department());
    }

    #[test]
    fn get_people_by_department_when_filled_with_sally() {
        let mut people_by_department = HashMap::new();
        people_by_department.insert("Sales".to_string(), vec!["Sally".to_string()]);
        let company = Company {
            people_by_department,
        };
        assert_eq!("Sales : Sally", company.get_people_by_department());
    }

    #[test]
    fn get_people_by_department_when_filled_with_more_than_Sally() {
        let mut people_by_department = HashMap::new();
        people_by_department.insert(
            "Sales".to_string(),
            vec!["Sally".to_string(), "Marc".to_string()],
        );
        let company = Company {
            people_by_department,
        };
        assert_eq!("Sales : Sally, Marc", company.get_people_by_department());
    }
}
