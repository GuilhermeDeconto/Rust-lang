pub fn mutable_references() {
    let mut s = String::from("hello");

    change(&mut s);

    println!("Mutable reference -> {}", s)
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}