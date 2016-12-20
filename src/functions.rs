pub fn function1() {
    call_me();
}

pub fn function2() {
   call_me_1(3);
}

fn call_me_1(value: i32) {
    for i in 0..value {
        println!("Ring! Call number {}", i+1);
    }
}

fn call_me() {}
