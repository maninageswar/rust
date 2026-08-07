mod student;
use student::*;

fn main() {
    let student1: Student = Student {
        name: String::from("sai"),
        gender: Gender::Male,
        age: 33,
    };
    println!("the student1 is {:#?}", student1);
}
