use std::collections::HashMap;

fn main() {
    let mut employees = vec!["Jean", "Marc", "Sally"];
}

struct Company {
    people_by_department: HashMap<String, Vec<String>>,
}

impl Company {
    pub fn new(people_by_department: HashMap<String, Vec<String>>) -> Company {
        let mut entries: Vec<(&String, &Vec<String>)> = people_by_department.iter().collect();
        entries.sort_by_key(|e| e.0);

        for entry in entries {
            let mut people = entry.1;
            let mut cloned = people.clone();
            cloned.sort();
        }

        Self {
            people_by_department,
        }
    }

    pub fn all_people_in_the_company_by_department(&self) -> String {
        let mut all_people = "".to_string();
        let mut entries: Vec<(&String, &Vec<String>)> = self.people_by_department.iter().collect();
        //entries.sort_by_key(|e| e.0);
        for (department, peoples) in entries {
            let mut sorted_peoples = peoples.clone();
            //sorted_peoples.sort();
            let s = format!("- {} : {}", department, sorted_peoples.join(", "));
            if !all_people.is_empty() {
                all_people.push('\n');
            }
            all_people.push_str(s.as_str());
        }
        all_people
    }
    pub fn get_by_department(&self, department: String) -> String {
        let people = self.people_by_department.get(&department);
        match people {
            None => "".to_string(),
            Some(p) => {
                let mut ret = p.clone();
                ret.sort();
                ret.join(", ")
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn get_people_by_department_when_empty() {
        let company = Company::new(HashMap::new());
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
    fn get_people_by_department_when_filled_with_more_than_sally() {
        let mut people_by_department = HashMap::new();
        people_by_department.insert(
            "Sales".to_string(),
            vec!["Sally".to_string(), "Marc".to_string()],
        );
        let company = Company::new(people_by_department);
        assert_eq!(
            "- Sales : Marc, Sally",
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
        let company = Company::new(people_by_department);
        assert_eq!(
            "- Devs : Bill, Ned\n- Sales : Marc, Sally",
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
        let company = Company::new(people_by_department);
        assert_eq!(
            "- Artisans : Booby\n- Devs : Bill, Ned\n- Sales : Marc, Sally",
            company.all_people_in_the_company_by_department()
        );
    }

    #[test]
    fn get_sales_people() {
        let mut people_by_department = HashMap::new();
        people_by_department.insert(
            "Sales".to_string(),
            vec!["Sally".to_string(), "Marc".to_string()],
        );
        people_by_department.insert(
            "Devs".to_string(),
            vec!["Bill".to_string(), "Ned".to_string()],
        );

        let company = Company::new(people_by_department);

        let result = company.get_by_department("Sales".to_string());

        assert_eq!("Marc, Sally", result)
    }
}
