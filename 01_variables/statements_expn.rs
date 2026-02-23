fn main() {
    let x = 5u32; // also can be written as 5_u32

    let y = {
        let x_squared = x*x;
        let x_cube = x_squared * x;
        // this will be assigned to y
        x_cube + x_squared + x // if semicolon added, then y will be unit type
    };
    let z = {
        2 * x  // if semicolon added, then z will be unit type
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}