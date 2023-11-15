// learning about safety of rust programs

fn read(y: bool) {
    if y {
        println!("y is true");
    }
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}

fn greet(g1: &String, g2: &String) {
    println!("{}, {}", g1, g2);
}

fn main() {
    println!("Learning Ownership");
    let x = true;
    read(x);

    // using heap to store large data and create pointers to it
    let ptr1 = Box::new([7; 10]);
    println!("initial ptr1 = {:?}", ptr1);
    let ptr2 = ptr1;
    // println!("again ptr1 = {:?}", ptr1);

    // checking ownership transfer
    let first = String::from("Ferris");
    let full = add_suffix(first);
    // println!("first = {first}");
    println!("full = {full}");

    // quiz-2
    let s = String::from("hello");
    let s2;
    let b = false;
    if b {
        s2 = s;
    }
    // println!("{}", s);

    // references are non-owning pointers
    let m1 = String::from("Hariom");
    let m2 = String::from("Guru");

    greet(&m1, &m2);
    println!("{}...{}", m1, m2);
}
