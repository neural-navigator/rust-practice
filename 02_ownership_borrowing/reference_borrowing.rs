// reference
/*
 - ``, which uses the `Display` trait
          - `?`, which uses the `Debug` trait
          - `e`, which uses the `LowerExp` trait
          - `E`, which uses the `UpperExp` trait
          - `o`, which uses the `Octal` trait
          - `p`, which uses the `Pointer` trait
          - `b`, which uses the `Binary` trait
          - `x`, which uses the `LowerHex` trait
          - `X`, which uses the `UpperHex` trait
 */

fn main() {
    let x = 5;
    let y = &x;
    println!("mem address of x = {}", y); // will show the value not address
    println!("mem address of x = {:p}", y);

    let mut s = String::from("hello");
    borrow_object(&s);

    let mut s1 = String::from("hello");
    push_str(&mut s1);
    println!("s1 = {}", s1);

    let ref s2 = s1;
    println!("s2 = {}", s2);
}

fn borrow_object(s: &String) {}

fn push_str(s: &mut String) {
    s.push_str(", world");
}

// ref can be used in place of &

/*
Lets see the below code

// Remove something to make it work
// Don't remove a whole line !
fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s; // either you can have one mutable reference or many immutable reference
    let r2 = &mut s; // so removing mut from both will work

    println!("{}, {}", r1, r2);

    println!("Success!");
}
 */

// you can not borrow an immutable object as a mutable, but you can borrow a
// mutable object as immutable

// This code has no errors!
fn main2() {
    let mut s = String::from("hello, ");

    borrow_object(&s);

    s.push_str("world");

    println!("Success!");
}

// fn borrow_object(s: &String) {} // this function can only be defined once

// NLL - non lexical lifetime
fn main() {
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    r1.push_str("world");
    let r2 = &mut s;
    r2.push_str("!");

    // println!("{}",r1); // as here we are using multiple mutable reference for s, if r1 is not
    // being used, then it will be marked as dead by compiler. So by commenting the
    // above line we are referring to r1
}