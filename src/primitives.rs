pub fn primitive1() {
    let is_morning = true;
    if is_morning {
        println!("Good morning!");
    }

    let is_evening = true;
    if is_evening {
        println!("Good evening!");
    }
}

pub fn primitive2() {
    let my_first_initial = 'A';
    if my_first_initial.is_alphabetic() {
        println!("Alphabetical");
    } else if my_first_initial.is_numeric() {
        println!("Numerical");
    } else {
        println!("It's neither.");
    }
}

pub fn primitive3() {
    let a: [i32; 100] = [0; 100];

    if a.len() >= 100 {
        println!("Wow, that's a big array.");
    } else {
        println!("That's tiny.");
    }
}

pub fn primitive4() {
    let a = [1, 2, 3, 4, 5];

    let nice_slice = &a[1..4];

    if nice_slice == &[2, 3, 4] {
        println!("That's a nice slice.");
    } else {
        println!("This ain'ta nice slice. {:?}", nice_slice);
    }
}

pub fn primitive5() {
    let cat = ("Fido", 5);
    let (name, age) = cat;

    println!("{} is {} years old.", name, age);
}

pub fn primitive6() {
    let numbers = (1, 2, 3);
    println!("The 2nd number is {}", numbers.1);
}
