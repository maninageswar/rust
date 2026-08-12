pub trait Registrable {
    fn register(&mut self) -> String;

    fn un_register(&mut self) -> String;

    fn is_registration_available(&self) -> bool;

    fn capacity(&self) -> usize;
}