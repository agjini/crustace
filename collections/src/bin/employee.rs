use std::collections::HashMap;
use std::ptr::hash;

fn main() {
    let mut employees = vec!["Jean", "Marc", "Sally"];
}

struct PeopleByDepartment(HashMap<String, String>);

impl PeopleByDepartment {
    pub fn get_people_by_department(&self) -> String {
        "".to_string()
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn get_people_by_department_when_empty() {
        let value = PeopleByDepartment(HashMap::new());
        assert_eq!("", value.get_people_by_department());
    }

    #[test]
    fn get_people_by_department_when_filled_with_sally() {
        let value = PeopleByDepartment(HashMap::new());
        assert_eq!("Sales : Sally", value.get_people_by_department());
    }
}
