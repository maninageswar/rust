#[derive(Debug)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Debug)]
pub struct Student {
    pub name: String,
    pub gender: Gender,
    pub age: u32, 
} 