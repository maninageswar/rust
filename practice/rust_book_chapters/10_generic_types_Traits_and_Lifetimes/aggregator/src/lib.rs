pub trait Summary {
    fn summarize_author(&self) -> String;
    // fn summarize_author(&self) -> String {
    //     println!("running form default method form trait:");
    //     format!("{}",self.author)
    // }

    fn summarize(&self) -> String {
        println!("running form default method form trait:");
        // we cannot use struct fields inside default methods of trait because traits can be implemented on any structs
        // and we never know that struct will always have that particular field we are using in this method exist in struct
        // so the below line will give compile time error
        // format!("read more articles like this by @{}", self.author)
        format!("read more articles like this by @{}", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    // if you comment the below method then the default method of the trait will run comment it anc check
    // fn summarize(&self) -> String {
    //     println!("running form impl of NewsArticle:");
    //     format!("read more articles like this by @{}", self.summarize_author())
    // }

    fn summarize_author(&self) -> String {
        println!("running form impl of NewsArticle:");
        format!("{}",self.author)
    }
}