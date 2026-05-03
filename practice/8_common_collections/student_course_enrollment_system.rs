use::std::collections::HashMap;

fn main() {
    let mut university: HashMap<String, Vec<String>> = HashMap::new();
    add_new_course(&mut university, String::from("ECE"));
    add_new_course(&mut university, String::from("CSE"));
    add_new_course(&mut university, String::from("EEE"));
    add_new_course(&mut university, String::from("MECH"));
    
    add_student_to_a_course(&mut university, String::from("ECE"), String::from("sai"));
    add_student_to_a_course(&mut university, String::from("ECE"), String::from("kumar"));
    add_student_to_a_course(&mut university, String::from("ECE"), String::from("pavan"));

    add_student_to_a_course(&mut university, String::from("CSE"), String::from("bharath"));
    add_student_to_a_course(&mut university, String::from("CSE"), String::from("nandhini"));
    add_student_to_a_course(&mut university, String::from("CSE"), String::from("pooja"));

    add_student_to_a_course(&mut university, String::from("MECH"), String::from("nikhal"));
    add_student_to_a_course(&mut university, String::from("MECH"), String::from("sampath"));
    add_student_to_a_course(&mut university, String::from("MECH"), String::from("snehith"));

    add_student_to_a_course(&mut university, String::from("EEE"), String::from("shalini"));
    add_student_to_a_course(&mut university, String::from("EEE"), String::from("sukumar"));

    println!("the students of CSE in alphabetical order is {:#?}", retrieve_all_students_of_specific_course_in_alphabetical_order(&university, "CSE"));

    println!("{:#?}", university);

    println!("all students of the university in alphabetical order {:#?}", retrieve_only_all_students_in_alphabetical_order(&university));
}

fn add_new_course(university: &mut HashMap<String, Vec<String>>, course: String) {
    if !university.contains_key(&course) {
        university.insert(course, Vec::<String>::new());
    } else {
        println!("the {} already present in the university", course);
    }
}

fn add_student_to_a_course(university: &mut HashMap<String, Vec<String>>, course: String, name: String) {
    if university.contains_key(&course) {
        university.get_mut(&course).unwrap().push(name);
    } else {
        println!("the university does not contain the course {} to add", course);
    }
}

fn retrieve_all_students_of_specific_course_in_alphabetical_order(university: &HashMap<String, Vec<String>>, course: &str) -> Vec<String> {
    if university.contains_key(course) {
        let mut students = university.get(course).unwrap().clone();
        students.sort();
        students
    } else {
        println!("the university does not contain the course {} to retrieve", course);
        Vec::new()
    }
}

fn retrieve_only_all_students_in_alphabetical_order(university: &HashMap<String, Vec<String>>) -> Vec<String> {
    let mut all_students: Vec<String> = Vec::new();
    for (course, students_of_course) in university {
        all_students.extend(students_of_course.clone());
    }
    // println!("{:#?}",university);
    // println!();
    all_students.sort();
    all_students
}
