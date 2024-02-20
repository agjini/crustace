use std::collections::HashMap;
use std::io;

use regex;
use regex::Match;

fn main() {
    println!("Wait a few minutes");

    println!("Add an employee to a department : Add [name] to [Departments]");

    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");
}

struct Company {
    people_by_department: HashMap<String, Vec<String>>,
}

impl Company {
    pub fn new() -> Company {
        Self {
            people_by_department: HashMap::new(),
        }
    }

    pub fn exec_user_input(&mut self, user_input: &str) {
        let (dept, name) = Company::parse_user_input(user_input);
        self.add_people(name, dept)
    }

    pub fn add_people(&mut self, name: &str, department: &str) {
        let people = self
            .people_by_department
            .entry(department.to_string())
            .or_insert(vec![]);
        people.push(name.to_string());
    }

    pub fn all_people_in_the_company_by_department(&self) -> String {
        let mut all_people = "".to_string();
        let mut entries: Vec<(&String, &Vec<String>)> = self.people_by_department.iter().collect();
        entries.sort_by_key(|e| e.0);
        for (department, peoples) in entries {
            let mut sorted_peoples = peoples.clone();
            sorted_peoples.sort();
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

    pub fn parse_user_input(input: &str) -> (&str, &str) {
        let r = regex::Regex::new("^Add (.*) to (.*)\\.?$");
        let matches = r.unwrap().captures(input).unwrap();
        let res: Vec<Option<Match>> = matches.iter().collect();
        let res: Vec<&str> = res.iter().map(|m| m.unwrap().as_str()).collect();
        println!("{res:?}");
        if res.len() != 3 {
            panic!("Unable to extract people and department from the given input");
        }

        (res[2], res[1])
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn get_people_by_department_when_empty() {
        let company = Company::new();
        assert_eq!("", company.all_people_in_the_company_by_department());
    }

    #[test]
    fn get_people_by_department_when_filled_with_sally() {
        let mut company = Company::new();
        company.add_people("Sally", "Sales");
        assert_eq!(
            "- Sales : Sally",
            company.all_people_in_the_company_by_department()
        );
    }

    #[test]
    fn get_people_by_department_when_filled_with_more_than_sally() {
        let mut company = Company::new();
        company.add_people("Sally", "Sales");
        company.add_people("Marc", "Sales");
        assert_eq!(
            "- Sales : Marc, Sally",
            company.all_people_in_the_company_by_department()
        );
    }

    #[test]
    fn get_people_by_department_when_filled_with_many_department() {
        let mut company = Company::new();
        company.add_people("Sally", "Sales");
        company.add_people("Marc", "Sales");
        company.add_people("Bill", "Devs");
        company.add_people("Ned", "Devs");
        assert_eq!(
            "- Devs : Bill, Ned\n- Sales : Marc, Sally",
            company.all_people_in_the_company_by_department()
        );
    }

    #[test]
    fn get_people_by_department_are_sorted() {
        let mut company = Company::new();
        company.add_people("Sally", "Sales");
        company.add_people("Marc", "Sales");
        company.add_people("Bill", "Devs");
        company.add_people("Ned", "Devs");
        company.add_people("Booby", "Artisans");
        assert_eq!(
            "- Artisans : Booby\n- Devs : Bill, Ned\n- Sales : Marc, Sally",
            company.all_people_in_the_company_by_department()
        );
    }

    #[test]
    fn get_sales_people() {
        let mut company = Company::new();
        company.add_people("Sally", "Sales");
        company.add_people("Marc", "Sales");
        company.add_people("Bill", "Devs");
        company.add_people("Ned", "Devs");
        let result = company.get_by_department("Sales".to_string());
        assert_eq!("Marc, Sally", result)
    }

    #[test]
    fn get_devs_people() {
        let mut company = Company::new();
        company.add_people("Sally", "Sales");
        company.add_people("Marc", "Sales");
        company.add_people("Bill", "Devs");
        company.add_people("Ned", "Devs");
        let result = company.get_by_department("Devs".to_string());
        assert_eq!("Bill, Ned", result)
    }

    #[test]
    fn guess_user() {
        let user_input = "Add Amir to Sales";

        let result = Company::parse_user_input(user_input);

        assert_eq!(("Sales", "Amir"), result);
    }

    #[test]
    fn guess_user_long_department() {
        let user_input = "Add Amir to Human Resources";

        let result = Company::parse_user_input(user_input);

        assert_eq!(("Human Resources", "Amir"), result);
    }

    #[test]
    fn guess_user_long_name() {
        let user_input = "Add Paul Jean to Sales";

        let result = Company::parse_user_input(user_input);

        assert_eq!(("Sales", "Paul Jean"), result);
    }

    #[test]
    fn add_people_from_user_input() {
        // Arrange
        let mut company = Company::new();
        let user_input = "Add Paul Jean to Sales";

        // Action
        company.exec_user_input(user_input);

        // Assert
        let sales = company.get_by_department("Sales".to_string());
        assert_eq!("Paul Jean", sales)
    }
    #[test]
    fn add_many_people_from_user_input() {
        // Arrange
        let mut company = Company::new();
        let user_input = ["Add Paul Jean to Sales", "Add Jack² to Sales"];

        // Action
        for x in user_input.iter() {
            company.exec_user_input(x);
        }

        // Assert
        let sales = company.get_by_department("Sales".to_string());
        assert_eq!("Jack², Paul Jean", sales)
    }

    #[test]
    fn add_many_people_from_user_input_to_different_departments() {
        // Arrange
        let mut company = Company::new();
        let user_input = ["Add Paul Jean to Sales", "Add Jack² to Human Resources"];

        // Action
        for x in user_input.iter() {
            company.exec_user_input(x);
        }

        // Assert
        let sales = company.get_by_department("Sales".to_string());
        let human_resources = company.get_by_department("Human Resources".to_string());
        assert_eq!("Jack²", human_resources);
        assert_eq!("Paul Jean", sales);
    }
}
