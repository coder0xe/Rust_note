use std::fmt::Display;

pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("brother {} is a thief ", self.summarize_author())
    }
}

pub struct NewsArtical {
    pub headline : String,
    pub location : String,
    pub author : String,
    pub content : String,
}

impl Summary for NewsArtical {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username : String,
    pub content : String,
    pub reply : bool,
    pub retweet : bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// pub fn notify (item : impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }

pub fn notify<T: Summary + Display>(item : T) {
    println!("Breaking news! {}", item.summarize());
}


pub fn notify1<T>(item : T) 
where T: Summary + Display
{
    println!("Breaking news! {}", item.summarize());
}