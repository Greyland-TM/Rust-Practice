pub trait Summary {
    fn summarize_author(&self) -> String {
        String::from("Unknown Author")
    }

    fn summarize(&self) -> String {
        format!("Read more from {}...", self.summarize_author())
        // This method will be used for the NewsArticle trait since its summarize method is
        // commented out.
    }
    // fn test(&self) -> String; // All trait methods must be implemented..
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// This is how you tweak a method defination fot a trait
impl Summary for NewsArticle {
    // fn summarize(&self) -> String {
    //     format!("{}, by {} ({})", self.headline, self.author, self.headline)
    // }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    // fn summarize(&self) -> String {
    //     format!("{}, by {}", self.username, self.content)
    // }
}
