mod ownership;
mod return_value;
mod mutable_references;
mod slice_type;

fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`


    //Deeply copying data

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    //The following code doesn't need to call clone because integers that have a known size at compile time are stored entirely on the stack, so copies of the actual values are quick to make. That means there’s no reason we would want to prevent x from being valid after we create the variable y.
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);


    let _hello = String::from("hello");  // hello comes into scope
    takes_ownership(s);                // hello's value moves into the function...
    // ... and so is no longer valid here

    let x = 5;                             // x comes into scope

    makes_copy(x);                  // x would move into the function,
    // but i32 is Copy, so it’s okay to still
    // use x afterward

    ownership::ownership_main();
    return_value::return_value();
    mutable_references::mutable_references();
    slice_type::slice_types();

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
// special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
