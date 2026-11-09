trait Employee {
    fn work(&self);
}

struct Developer {
    name: String,
}

impl Employee for Developer {
    fn work(&self) {
        println!("{} is writing code", self.name);
    }
}

struct Designer {
    name: String,
}

impl Employee for Designer {
    fn work(&self) {
        println!("{} is designing the ui", self.name);
    }
}

struct Manager {
    name: String,
}

impl Employee for Manager {
    fn work(&self) {
        println!("{} is monitoring the employees", self.name);
    }
}

fn main() {
    let employees: Vec<Box<dyn Employee>> = vec![
        Box::new(Developer { name: String::from("sai") }),
        Box::new(Designer { name: String::from("kumar") }),
        Box::new(Manager { name: String::from("sanjana") })
    ];

    for employee in employees {
        employee.work();
    }
}
