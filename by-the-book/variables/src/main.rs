fn main() {
    // mutable vs immutable

    /*
        let x = 5;
        x = 6;
        This would lead to a compiler error
    */
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // shodowing

    let y = 5;
    let y = y + 1;
    let y = y * 2;

    println!("The value of y is: {}", y);

    /*
        let mut spaces = "    ";
        spaces = spaces.len();
        This does not compile as `spaces` is mutable and of type string.
        So it stays as type of string and when trying to reassign the compiler complains.
        Shadowing on the other hand removes the old variable and its type and
        gives you a new variable with whatever type you want.
    */
    let spaces = "    ";
    let spaces = spaces.len();
    println!("Spaces len is: {}", spaces);
}
