use std::mem::size_of_val;

fn main() {
    let c1 = "a";
    assert_eq!(size_of_val(c1), 1);

    let c2 = '中';
    assert_eq!(size_of_val(&c2),4);

    let c3: bool = false;

    if !c3 {
        println!("c3 is not true");
    }

    let f = true;
    let t = true || false;
    assert_eq!(t, f);

    let _v: () = ();

    let v = (2, 3);
    assert_eq!(_v, implicit_ret_unit());

    let unit: () = ();
    assert!(size_of_val(&unit) == 4);

    println!("success!")
}

fn implicit_ret_unit() {
    println!("implicit ret_unit");
}

fn explicit_ret_unit() -> () {
    println!("explicit ret_unit");
}

