fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);

    //This won't work if x was defined without "mut" because variables are immutable by default
    x = 6;

    println!("The value of x is: {}", x);

    //Rust’s naming convention for constants is to use all uppercase with underscores between words, and underscores can be inserted in numeric literals to improve readability
    const MAX_POINTS: u32 = 100_000;

    //Shadowing

    let s = 5;

    let s = s + 1; //first s will be = 5 and then after the expression (s +1) it will be 6

    let s = s * 2;

    println!("The value of s is: {}", s);

    //The following expression will throw an error if the variable is initialized with mut since we cannot mutate the variable's type
    let spaces = "   ";
    let spaces = spaces.len();

    println!("The value of spaces is: {}", spaces);

    // Scalar Types -> Integers, floating-point numbers, Booleans, and characters

    // Length	Signed	Unsigned
    // 8-bit	 i8	      u8
    // 16-bit	 i16	  u16
    // 32-bit	 i32      u32
    // 64-bit	 i64	  u64
    // 128-bit	 i128	  u128
    // arch	     isize	  usize

    //Floating-Point Types
    //f32 and f64
    //Default type is f64

    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    //Numeric operations
    //Addition, subtraction, multiplication, division, and remainder

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;


    //Boolean Type
    //Two possible types true or false

    let t = true;

    let f: bool = false; // with explicit type annotation

    //Character Type
    //Char literals are specified with single quotes, as opposed to string literals, which use double quotes

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    //Compound Types
    //Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

    //The Tuple Type
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);


    let tup1: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = tup1.0;

    let six_point_four = tup1.1;

    let one = tup1.2;

    //Array Type

    let array = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    let array2: [i32; 5] = [1, 2, 3, 4, 5]; //Here, i32 is the type of each element. After the semicolon, the number 5 indicates the array contains five elements.

    let first = array[0];
    let second = array[1];

}