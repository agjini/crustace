use std::collections::HashMap;

fn main() {
    let mut employees = vec!["Jean", "Marc", "Sally"];
}

struct Company {
    people_by_department: HashMap<String, Vec<String>>,
}

impl Company {
    pub fn all_people_in_the_company_by_department(&self) -> String {
        let mut all_people = "".to_string();
        let mut entries: Vec<(&String, &Vec<String>)> = self.people_by_department.iter().collect();
        entries.sort_by_key(|e| e.0);
        for (department, peoples) in entries {
            let s = format!("- {} : {}", department, peoples.join(", "));
            if !all_people.is_empty() {
                all_people.push('\n');
            }
            all_people.push_str(s.as_str());
        }
        all_people
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
        assert_eq!("", company.all_people_in_the_company_by_department());
    }

    #[test]
    fn get_people_by_department_when_filled_with_sally() {
        let mut people_by_department = HashMap::new();
        people_by_department.insert("Sales".to_string(), vec!["Sally".to_string()]);
        let company = Company {
            people_by_department,
        };
        assert_eq!(
            "- Sales : Sally",
            company.all_people_in_the_company_by_department()
        );
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
        assert_eq!(
            "- Sales : Sally, Marc",
            company.all_people_in_the_company_by_department()
        );
    }

    #[test]
    fn get_people_by_department_when_filled_with_many_department() {
        let mut people_by_department = HashMap::new();
        people_by_department.insert(
            "Sales".to_string(),
            vec!["Sally".to_string(), "Marc".to_string()],
        );
        people_by_department.insert(
            "Devs".to_string(),
            vec!["Bill".to_string(), "Ned".to_string()],
        );
        let company = Company {
            people_by_department,
        };
        assert_eq!(
            "- Devs : Bill, Ned\n- Sales : Sally, Marc",
            company.all_people_in_the_company_by_department()
        );
    }

    #[test]
    fn get_people_by_department_are_sorted() {
        let mut people_by_department = HashMap::new();
        people_by_department.insert(
            "Sales".to_string(),
            vec!["Sally".to_string(), "Marc".to_string()],
        );
        people_by_department.insert(
            "Devs".to_string(),
            vec!["Bill".to_string(), "Ned".to_string()],
        );
        people_by_department.insert("Artisans".to_string(), vec!["Booby".to_string()]);
        let company = Company {
            people_by_department,
        };
        assert_eq!(
            "- Artisans : Booby\n- Devs : Bill, Ned\n- Sales : Sally, Marc",
            company.all_people_in_the_company_by_department()
        );
    }
}
