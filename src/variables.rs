pub fn variable1() {
    let x = 5;
    println!("{}", x);
}

pub fn variable2() {
    let x = 10;
    if x == 10 {
        println!("{}", x);
    } else {
        println!("Not ten.");
    }
}

pub fn variable3() {
    let mut x = 3;
    println!("{}", x);
    x = 5;
    println!("{}", x);
}

pub fn variable4() {
    let x: i32 = 42;
    println!("{}", x);
}
