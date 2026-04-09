use std::collections::HashMap;

use regex;
use regex::Match;

pub struct Company {
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
        let r = regex::Regex::new("^[Aa]dd (.*) to (.*)\\.?\\s*$");
        let matches = r.unwrap().captures(input).unwrap();
        let res: Vec<Option<Match>> = matches.iter().collect();
        let res: Vec<&str> = res.iter().map(|m| m.unwrap().as_str()).collect();

        if res.len() != 3 {
            panic!("Unable to extract people and department from the given input");
        }

        (res[2], res[1])
    }
}
