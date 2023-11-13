fn main() {
    println!("learning loops");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    // for loop
    let a = [11; 3];
    for element in a {
        print!("{element} ");
    }
    print!("\n");

    // Range & RangeInclusive
    for i in 0..5 {
        print!("{i} ");
    }

    println!("");

    for j in 0..=5 {
        print!("{j} ");
    }
    println!("");

    // using reverse method (rev())
    for k in (0..=9).rev() {
        print!("{k} ");
    }
}
