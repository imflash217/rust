use dotenv::dotenv;

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
    let guess: u32 = "hariom".parse().expect("not a number!!!");
    println!("guess = {guess}");
}
