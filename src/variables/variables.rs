const SOME_CONST: u32 = 5;

pub fn run() {
    //
    // Variables in rust are per default immutable.
    // To make a variable mutable, use the mut keyword.
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    //
    // Constants are always immutable.
    println!("The value of our CONST is: {SOME_CONST}");

    //
    // Shadowing;
    // You can declare a new variable with the same name as a previous variable,
    // and the new variable shadows the previous variable.
    {
        let x = x * 5; // should be 30
        println!("The value of x inner scope is: {x}");
    }
}
