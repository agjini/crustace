use crate::employee::Company;

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
fn add_people_from_user_input_with_return_carriage() {
    // Arrange
    let mut company = Company::new();
    let user_input = "Add Fabien to Bi\n";

    // Action
    company.exec_user_input(user_input);

    // Assert
    let bi = company.get_by_department("Bi".to_string());
    assert_eq!("Fabien", bi)
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

#[test]
fn add_people_from_user_input_without_uppercase() {
    // Arrange
    let mut company = Company::new();
    let user_input = "add Fabien to Bi";

    // Action
    company.exec_user_input(user_input);

    // Assert
    let bi = company.get_by_department("Bi".to_string());
    assert_eq!("Fabien", bi)
}
