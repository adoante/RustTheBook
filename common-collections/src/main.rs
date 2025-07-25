fn main() {
    let _v: Vec<i32> = Vec::new();

    let mut v2 = vec![1, 2, 3];

    v2.push(4);
    v2.push(5);
    v2.push(6);

    let third: &i32 = &v2[2];
    println!("The 3rd element is {}", third);

    let third: Option<&i32> = v2.get(2);
    match third {
        Some(third) => println!("The 3rd element is {}", third),
        None => println!("There is no third element"),
    }

    for i in &v2 {
        println!("{}", i);
    }

    for i in &mut v2 {
        println!("{}", *i + 1);
    }

    if let Some(last_element) = v2.pop() {
        println!("{last_element}");
    }

    let mut s = String::new();

    let data = "initial data";
    s = data.to_string();
    println!("{}", s);

    let mut s = String::from("initial data");
    println!("{}", s);

    let s2 = " i guess :/";
    s.push_str(s2);
    println!("{}", s);

    s.push('/');
    println!("{}", s);

    let s3 = s + &s2;
    println!("{}", s3);

    // highly doubt anyone is reading this but I thought and
    // am currently thinking this is funny, smile :)
    let c = String::from("cock");
    let b = String::from("ball");
    let t = String::from("torture");

    let cbt = format!("{c} and {b} {t}");

    println!("{cbt}");

    for c in cbt.chars() {
        match c {
            'c' => println!("found a c"),
            'b' => println!("found a b"),
            't' => println!("found a t"),
            _ => (),
        }
    }

    println!("YOU KNOW WHAT THAT STANDS FOR! C! B! T!");

    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let blue_score = scores.get("Blue").copied().unwrap_or(0);
    println!("Blue: {}", blue_score);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
}
