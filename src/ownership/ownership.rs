pub fn run() {
    // The Stack and the Heap
    // https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#the-stack-and-the-heap

    // Ownership Rules
    // 1. Each value in Rust has a variable that’s called its owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

    // let mut s = String::from("Hello");
    // s.push_str("World");

    // println!("{}", s);

    // When a variable goes out of scope, Rust calls a special function for us.
    // This function is called drop, and it’s where the author of String can put
    // the code to return the memory. Rust calls drop automatically at the closing
    // curly bracket.

    //
    // Ownership and Functions
    //
    // let s1 = gives_ownership();
    // let s2 = String::from("Hello");
    // let s3 = takes_ownership_and_gives_back(s2);
    // let (s4, length) = calc_length_without_ref(s1);
    // println!("The length of '{s4}' is {length}");

    // With Reference x)
    // let s = String::from("Hello");
    // let length = calc_length_with_ref(&s);
    // println!("The length of '{s}' is {length}");

    //
    // Mutable References
    //
    let mut s = String::from("Hello");
    mutate(&mut s);
}

fn take_ownership(s: String) {
    println!("{}", s)
}

fn make_copy(x: i32) {
    println!("{}", x)
}

fn gives_ownership() -> String {
    let some_string = String::from("Your String");

    some_string
}

fn takes_ownership_and_gives_back(s: String) -> String {
    println!("{}", s);

    s
}

fn calc_length_without_ref(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

fn calc_length_with_ref(s: &String) -> usize {
    s.len()
}

fn mutate(s: &mut String) {
    s.push_str(" World");
}
