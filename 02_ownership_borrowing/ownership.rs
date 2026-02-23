/*
In rust memory is managed through a system of ownership with 3 strict rules:

1. Each value has a single owner (a variable).
2. There can be only one owner at a time.
3. When the owner goes out of scope, the value is destroyed (dropped) and the memory is freed.

The solution to this is borrowing and reference. If the ownership always moves, passing 01_variables
into functions would be a nightmare.
*/

// ownership

fn main() {
    let x = 5;
    let y = x;

    /*
    Types that are stored in stack have a fixed size known at compile time. Rust simply copy those
    If the variable is stored in heap, then the variable is re-assigned.
     */
    println!("x = {}, y = {}", x, y);
    let s1 = String::from("hello");
    // this will yield error
    // let s2 = s1; // value of s1 is moved to s2, and s1 is uninitialized now, which is not copied.
    let s2 = takes_ownership(s1);
    println!("s2 = {}", s2);
    let s3 = give_ownership();

    // mutability snippets
    let s4 = String::from("hello");
    let mut s5 = s4; // without mut this will give error
    s5.push_str(", world!");
    println!("success!")
}

fn takes_ownership(s: String)  -> String {
    println!("takes_ownership: {}", s);
    return s;
}

fn give_ownership() -> String {
    let s = String::from("hello");
    // let _s = s.into_bytes(); // this line can cause error as the ownership transferred
    s
}

/*
The solution to the ownership can be solved using clone or copy:

- clone: The clone trait is for types that usually own data on heap.
Copying the heap data can be expensive. Its explicit and .clone() method must be called.
Types String, vec and most custom Structs.

- copy: Its implicit and faster. Generally used for the data stored in heap.
Supported types - i32, bool, f64, char, tuple/array with only copy types.
The original value remains valid and usable. The copy is executed, when y=x is called.
 */

// mutability: Mutability can be changed when ownership is transferred.

/*
Partial move: With the destructuring of a single variable, both by move and by reference pattern
binding can be used at the same time. Doing this will result in a partial move of the variable,
which means that parts of the variable will be moved while other parts stay. In such case,
the parent variable cannot be used afterwards as a whole, however the parts that are only
referenced (and not moved) can still be used.
 */
