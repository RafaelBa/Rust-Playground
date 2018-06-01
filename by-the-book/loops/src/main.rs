fn main() {
    // loop: endless loop
    let mut i = 0;

    loop {
        println!("loop again!");
        i += 1;
        if i == 10 {
            break;  // only way to stop 'loop', since loop is endless as it has no condition
        }
    }

    // while: typical while loop
    let mut j = 3;

    while j != 0 {
        println!("while {}", j);
        j = j - 1;
    }

    println!("while LIFTOFF");

    let array_one = [10, 20, 30, 40, 50];
    let mut index_one = 0;

    while index_one < 5 {
        println!("while array value: {}", array_one[index_one]);

        index_one = index_one + 1;
    }

    // for: classic foreach
    let array_two = [10, 20, 30, 40, 50];

    for element_two in array_two.iter() {
        println!("for array value: {}", element_two);
    }

    for number_one in (1..4).rev() {
        println!("for countdown: {}", number_one);
    }

    println!("for LIFTOFF");
}
