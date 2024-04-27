use std::collections::HashMap; // Bring in HashMap with use

// defining a vector this way in not necessary 
// let my_vector = Vec<i32>::new();

// This is a much more consice way to create a vector of i32 integers.
// const my_vec = vec![3, 4, 5]

fn main() {
    // Vectors are also imutable by default and must set mut explicately
    let mut new_vec: Vec<i32> = vec![]; 

    new_vec.push(1);
    new_vec.push(2);
    new_vec.push(3);
    new_vec.push(4);

    let third_elm: &i32 = &new_vec[2];
    println!("The third element is {third_elm}.");

    let fifth: Option<&i32> = new_vec.get(4);
    match fifth {
        Some(fifth) => println!("The fifth element is {fifth})"),
        None => println!("There is no fifth element")
    }

    for index in &new_vec{
        println!("Index: {index}");
    }

    // This will add 20 to each vector value. 
    // The index must be dereferenced first with *.
    for index in &mut new_vec {
        *index += 20;
    }

    // Strings
    let mut s1 = "foo".to_string();
    let s2 = "bar";
    s1.push_str(s2);
    println!("{s1}, {s2}");
    
    let test_str_1 = String::from("lo");
    let test_str_2 = String::from("l");
    let test_str = test_str_1 + &test_str_2; // Strings can be concatonated with +, but the second
                                             // valu must be a reference. 
    println!("{test_str}");

    // HASH MAPS
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10); // insert() replaces existing values.
    scores.insert(String::from("Yellow"), 30);
    
    let team = "Blue".to_string();
    let score: i32 = scores.get(&team).copied().unwrap_or(0); // .get() retrieved a Option<&v>,
                                                              // wich is then turned to an
                                                              // Option<v> with copied(). And set
                                                              // to 0 if None by unwrap_or().
    println!("The {team} team score is {score}");

    scores.entry(String::from("Blue")).or_insert(50); // .entry() returns an Entry enum and
                                                      // or_insert() is an Entry method that inserts
                                                      // the given value
    scores.entry(String::from("Green")).or_insert(25); 

    // HashMaps can be loopes over just like Strings.
    for (key, value) in scores {
        println!("{key} {value}")
    }
    
    let demo_text = "Check out this awesome message message";
    let mut map = HashMap::new();
    for word in demo_text.split_whitespace() {
       let count = map.entry(word).or_insert(0);
       *count += 1;
    }

    println!("{:?}", map);
}
