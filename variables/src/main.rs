// use dotenv::dotenv;
// use typeinfo:

const TWO: u32 = 1 + 1; // can use const in global scope
                        // let y = 20; // cannot use let in global scope
                        // RUST_BACKTRACE=1;
fn main() {
    // using let & mut keywords
    println!("Hello, world!");
    let mut x = 5; // let keyword can only be used in functions or classes
    println!("the value of x = {x}");
    x = 6;
    println!("the new value of x = {x}");

    // using const keyword
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("THREE_HOURS_IN_SECONDS = {THREE_HOURS_IN_SECONDS}");
    println!("{TWO}");

    // variable SHADOWING
    let z = 217;
    let z = 217 * 2;

    {
        let z = z + 100;
        println!("inside @ z = {z}"); // this will print 534
    }
    println!("outside @ z = {z}"); // this will print 434

    let spaces = "    ";
    println!("1st spaces = `{spaces}`");
    let spaces = spaces.len();
    println!("2nd spaces = {spaces}");

    // explicit data types
    let guess: u32 = "217".parse().expect("not a number!!!");
    println!("guess = {guess}");

    // compound types: tuples
    let tuple: (i32, f64, u8) = (-21, 2.17, 99); // assign a tuple with compund types
    let (x, y, z) = tuple; // deconstruct teh tuple to access individual values
    println!("y = {y}");
    let val1 = tuple.0; // access an element in tuple
    let val2 = tuple.1;
    let val3 = tuple.2;
    println!("val1 = {val1}");
    println!("val2 = {val2}");
    println!("val3 = {val3}");
    println!("tuple = {:?}", tuple);
    println!("tuple = {:#?}", tuple);

    // unit tuple
    let unit = ();
    println!("unit = {:?}", unit);

    // compound type: array
    // array have fixed length
    // all elements in an array are of same type

    let my_array: [i32; 5] = [-1, 2, 3, 4, 5];
    println!("my_array = {:?}", my_array);

    let my_array2 = [217; 5];
    println!("my_array2 = {:?}", my_array2);

    // accessing elements of an array
    println!("my_array[0] = {:?}", my_array[0]);
    println!("my_array[4] = {:?}", my_array[4]);

    // mutable compound types
    // my_array[0] = 100;
    let mut tp: (char, i32) = ('q', 17);
    println!("tp = {:?}", tp);
    println!("tp.0 = {:?}", tp.0);
    tp.0 = 'w';
    println!("tp.0 = {:?}", tp.0);

    another_function(1_000, '🦀');
}

fn another_function(x: i32, y: char) {
    if x < 1_000 {
        println!("Inside another function: x = {x} & y = {y}");
    } else {
        println!(">= 1_000")
    }

    // exploring if conditional
    let condition = true;
    let number = if condition { 100 } else { 200 };
    println!("number = {number}");

    // loop {
    //     print!("again..");
    // }

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
            // break;
        }
    };
    println!("result = {result}");
}
