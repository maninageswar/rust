enum Language {
    Formal,
    Informal,
}

fn get_greeting<'a>(lan: &Language, fg: &'a str, ifg: &'a str) -> &'a str {
    match lan {
        Language::Formal => fg,
        Language::Informal => ifg,
    }
}

fn main() {
    let language: Language = Language::Formal;
    let formal_greetings: String = String::from("Hi Subrat, Good morning");
    let informal_greetings: String = String::from("Yo bro, morning");
    let greeting = get_greeting(&language, &formal_greetings, &informal_greetings);
    println!("the geeting is {}", greeting);
}