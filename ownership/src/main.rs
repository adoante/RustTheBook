fn main() {
    {
        let _s = "string";
    }

    let _s = String::from("hello");

    let mut s = String::from("hello");

    s.push_str(", world!"); // appends literal to strings

    println!("{s}");

    {
        let _s = String::from("hello");
    }

    let s1 = String::from("hello"); // s1 gets dropped, "replaced" wholly by s2
    let _s2 = s1;

    //println!("{s1}, world!") // won't compile

    let mut s = String::from("hello");
    s = String::from("ahoy");

    println!("{s}, world!");

    let s1 = String::from("hello");
    let s2 = s1.clone(); // makes a deep copy of the stack values AND heap data

    println!("s1 = {s1}, s2 = {s2}");

    let x = 5; // x doesn't get "replaced" by y
    let y = x;

    println!("x = {x}, y = {y}");

    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
    // ... and so is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x); // because i32 implements the Copy trait,
    // x does NOT move into the function,

    println!("{}", x);

    let s1 = gives_ownership(); // gives_ownership moves its return
    // value into s1

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into
    // takes_and_gives_back, which also
    // moves its return value into s3
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
    // moves out to the calling
    // function
}

// This function takes a String and returns a String.
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}
