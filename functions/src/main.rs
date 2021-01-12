fn main() {
    println!("Hello, world!");

    //Rust code uses snake case as the conventional style for function and variable names. In snake case, all letters are lowercase and underscores separate words.
    another_function();

    another_function_parameter_1(5);

    another_function_parameter_2(5,6);

    function_body_with_statement();
    
    function_with_return_value_5();

    let x = function_with_return_value_5();
    println!("The value of x is: {} -> function with return value", x);

    function_with_return_value_plus_one(5);
    let plus_one = function_with_return_value_plus_one(5);
    println!("The value of x is: {} -> function plus one", plus_one);
}

fn function_with_return_value_plus_one(x: i32) -> i32 {
    x + 1
}

fn function_with_return_value_5() -> i32 {
    5
}

fn function_body_with_statement() {

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}

fn another_function_parameter_2(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn another_function_parameter_1(x: i32) {
    println!("The value of x is: {}", x);
}

fn another_function() {
    println!("Another function.");
}