use std::fmt::Display

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // Rust give a lifetime error here becuase we dont know if the return type is a ref to x or y
    // we need the 'a syntax here to fix that...
    // By setting the 'a lifetime we say the returned reference will be valid as long a both
    // parameters are valid....
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Lifetimes can be used in a struct as well
struct ImportantExcerpt<'a> {
    _part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

// This function uses generic type perameters, trait bounds and lifetimes. Crazy...
fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str 
where 
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("test");
    let string2 = "testing";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    // As another example:
    let string3 = String::from("This string is long");
    let result;
    {
        let string4 = String::from("short");
        result = longest(string3.as_str(), string4.as_str());
        println!("The longest string is {}", result); // This works because the shortest lifetime
                                                      // is for string4, and longest is called within the scope of this shortest variable. So
                                                      // when longest expects a &'a str no longer than the shortes 'a it will be allowed
    }

    //println!("The longest string is {}", result); // This will fail because the lifetime on string4
    // will be out of scope

    // This will also be out of scope, lifetime will fail.
    // {
    //     let novel = String::from("My name is unimportant. Because I am the one without...");
    //     let first_scentence = novel.split('.').next().expect("Could not find a '.'");
    // }
    let novel = String::from("My name is unimportant. Because I am the one without...");
    let first_scentence = novel.split('.').next().expect("Could not find a '.'");

    let i = ImportantExcerpt {
        _part: first_scentence,
    };

    let _level = i.level();
}
