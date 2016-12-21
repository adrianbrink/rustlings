pub fn function1() {
    call_me();
}

pub fn function2() {
   call_me_1(3);
}

pub fn function4() {
    let original_price = 51;
    println!("Your sale price is {}", sale_price(original_price));
}

pub fn function5() {
    let answer = square(3);
    println!("The answer is {}", answer);
}

fn square(num: i32) -> i32 {
    num * num
}

fn sale_price(price: i32) -> i32 {
    if is_even(price) {
        price - 10
    } else {
        price -3
    }
}

fn is_even(value: i32) -> bool {
    value % 2 == 0
}

fn call_me_1(value: i32) {
    for i in 0..value {
        println!("Ring! Call number {}", i+1);
    }
}

fn call_me() {}
