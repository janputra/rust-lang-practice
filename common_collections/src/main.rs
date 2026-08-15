use std::collections::HashMap;

fn main() {
    vector_example();
    string_example();
    hashmap_example();
}

#[derive(Debug)]
enum Spreadsheetcell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn vector_example() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2]; // direct borrow access
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);

    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    }

    let v = vec![1, 2, 3, 4, 5];
    for i in &v {
        println!("{i}");
    }
    let mut v = vec![1, 2, 3, 4, 5];
    for i in &mut v {
        *i += 10;
    }
    println!("{v:?}");

    // Using an Enum to Store Multiple Types
    let row = vec![
        Spreadsheetcell::Int(3),
        Spreadsheetcell::Text(String::from("test")),
        Spreadsheetcell::Float(3.14),
    ];

    println!("{row:?}");
}

fn string_example() {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s1 is {s1}");
    println!("s2 is {s2}");

    let s3 = s1 + &s2;
    println!("s3 is {s3}");

    let s = format!("{s2}-{s3}");
    println!("s is {s}");
}

fn hashmap_example() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{scores:?}");

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1; // deference (similar to accessing value of pointer)
    }
    println!("{map:?}");
}
