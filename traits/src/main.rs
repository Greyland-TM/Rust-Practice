use aggregator::{NewsArticle, Summary, Tweet};
use std::fmt::{Debug, Display};
use std::ops::Add;

fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }

fn _article_number<T: Summary, U: Add + Debug>(item: &T, number: &U) {
    println!("{}, {:?}", item.summarize(), number);
}

// Use the where syntax for functions that use many different traits
fn article_number_alt<T, U>(item: &T, number: &U)
where
    T: Summary,
    U: Add + Debug,
{
    println!("{}, {:?}", item.summarize(), number);
}

// An important nots that this syntax for returning an object with a specific trait will only work
// if you return exactly one objec.
fn create_tweet() -> impl Summary {
    let tweet = Tweet {
        username: String::from("Billy Joe"),
        content: String::from("Howdy y'all"),
        retweet: false,
        reply: false,
    };
    tweet
    // This would not work because the function could return either a Tweet or a NewsArticle.
    // I'll learn in a later chapter how to do something like this.

    // if true {
    //     let tweet = Tweet {
    //         username: String::from("Billy Joe"),
    //         content: String::from("Howdy y'all"),
    //         retweet: false,
    //         reply: false,
    //     };
    //     tweet
    // } else {
    //     let news_article = NewsArticle {
    //         headline: String::from("X is a stuipid name."),
    //         location: String::from("Portland, Or"),
    //         author: String::from("Greyland M"),
    //         content: String::from("Listen, X is just a stipid name, alright?!"),
    //     };
    // }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {x, y}
    }
}

impl <T: Display, PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x > self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn main() {
    // A new comment
    let tweet = Tweet {
        username: String::from("Greyland M"),
        content: String::from("Listen, X is just a stipid name, alright?!"),
        retweet: false,
        reply: false,
    };

    let news_article = NewsArticle {
        headline: String::from("X is a stuipid name."),
        location: String::from("Portland, Or"),
        author: String::from("Greyland M"),
        content: String::from("Listen, X is just a stipid name, alright?!"),
    };

    println!("1 new tweet: {}", tweet.summarize());
    println!("New article available! {}", news_article.summarize());

    notify(&news_article);
    let article_id = 14;
    article_number_alt(&news_article, &article_id);

    let _new_tweet = create_tweet();

