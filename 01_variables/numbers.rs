use std::any::type_name;
use std::ops::Range;
use std::ops::RangeInclusive;

fn main() {
    let x: i32 = 5;
    let mut y: i32 = 5;

    y=x;
    let z = 12;
    println!("The value of z is: {}", z);

    let v: u16 = 38_u8 as u16;
    println!("The value of v is: {}", v);

    // assert_eq!("u32".to_string(), type_of(&x));
    println!("{} - {}", i8::MAX, u8::MAX);

    let v1 = 251_i16 + 8;
    let v2 = i16::checked_add(251, 8).unwrap();
    println!("{},{}",v1,v2);

    // different type of int sum (default conversion)
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    println!("{}",v);
    assert!(0.1_f32+0.2_f32==0.3_f32);

    // range implementation
    for i in -3..2 {
        println!("item :: {}", i);
    }

    assert_eq!((1..5), Range{start:1, end:5});
    assert_eq!((1..=5), RangeInclusive::new(1, 5));
}

fn type_of<T>(_: &T) -> String {
    format!("{}", type_name::<T>())
}

