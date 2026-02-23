fn main() {
    let (x, y) = (1, 2);
    let s = sum(x, y);

    assert_eq!(s, 3);
    println!("s = {:?}", s);
}

fn sum(a: i32, b: i32) -> i32 {
    // if return statement type is not given then a+b will yield error
    a+b // also can be return a+b;, else a+b only
}


fn main() {
    print();
}

fn print() -> i32 {
    println!("success!") // expected i32 found unit, will give error
}

fn main() {
    never_return();
    println!("failed");
}

fn never_return() {
    // the signature is empty
}


// Diverging functions - Diverging functions never return to the caller, so they may used in places
// where a value of any type is expected.

fn main() {
    println!("Success!");
}

