use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq, Debug)] // have to learn why do we need this
enum Department {
    Sales,
    It,
    Hr,
    Nerds,
}

fn main() {
    let mut company: HashMap<Department, Vec<String>> = HashMap::new();
    add_employee_to_the_department(&mut company, Department::Sales, String::from("ravi"));
    add_employee_to_the_department(&mut company, Department::Sales, String::from("kumar"));
    add_employee_to_the_department(&mut company, Department::Sales, String::from("sai"));
    add_employee_to_the_department(&mut company, Department::Sales, String::from("bharath"));
    add_employee_to_the_department(&mut company, Department::Sales, String::from("naveen"));
    add_employee_to_the_department(&mut company, Department::It, String::from("tarun"));
    add_employee_to_the_department(&mut company, Department::It, String::from("vasu"));

    add_new_department(&mut company, Department::Hr);
    add_employee_to_the_department(&mut company, Department::Hr, String::from("sumanth"));
    add_employee_to_the_department(&mut company, Department::Hr, String::from("komali"));
    add_employee_to_the_department(&mut company, Department::It, String::from("nathan"));

    println!("company {:#?}", company);
    println!();

    // println!(
    //     "employees of Sales department {:#?}",
    //     get_employee_names_by_department_in_alphabetical_order(&mut company, Department::Nerds)
    //         .unwrap() // works fine for Some variant but panics at run time
    // );

    match get_employee_names_by_department_in_alphabetical_order(&mut company, Department::Nerds) {
        Option::Some(sorted_employee_names) => { println!("employees of Sales department {:#?}", sorted_employee_names); },
        Option::None => { println!("the department does not exist in this company"); }
    }
}

fn add_new_department(company: &mut HashMap<Department, Vec<String>>, department: Department) {
    company.entry(department).or_insert(Vec::<String>::new());
}

fn add_employee_to_the_department(
    company: &mut HashMap<Department, Vec<String>>,
    department: Department,
    name: String,
) {
    // check if the department exitst if it does then add
    company
        .entry(department)
        .or_insert(Vec::<String>::new())
        .push(name);
}

fn get_employee_names_by_department_in_alphabetical_order(
    company: &mut HashMap<Department, Vec<String>>,
    department: Department,
) -> Option<&Vec<String>> {
    if let Some(employee_names) = company.get_mut(&department) {
        employee_names.sort();
        company.get(&department)
    } else {
        None
    }
}
