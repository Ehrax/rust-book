pub fn run() {
    // statement
    statement();

    // Expression
    let y = expression(5);
    println!("The value of y is: {y}")
}

// Statement
fn statement() {
    let x = 3;
    println!("The value of x is: {x}")
}

// Expression
fn expression(x: i32) -> i32 {
    x + 1
}
