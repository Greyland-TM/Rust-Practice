pub trait Summary {
    fn summarize(&self) -> String;
    // fn test(&self) -> String; // All trait methods must be implemented..
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// This is how you tweak a method defination for a trait
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.headline)
    }
}

pub struct Tweet {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.headline)
    }
}

fn main() {
    let tweet = Tweet {
        headline: String::from("X is a stuipid name."),
        location: String::from("Portland, Or"),
        author: String::from("Greyland M"),
        content: String::from("Listen, X is just a stipid name, alright?!"),
    };

    println!("1 new tweet: {}", tweet.summarize());
}
