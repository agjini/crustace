use std::io;

use crate::employee::Company;

mod employee;

#[cfg(test)]
mod test;

fn main() {
    println!("Wait a few minutes");

    let mut company = Company::new();
    loop {
        println!("Add an employee to a department : Add [name] to [Departments]");

        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        company.exec_user_input(&user_input);

        let display = company.all_people_in_the_company_by_department();
        println!("{display}")
    }
}
