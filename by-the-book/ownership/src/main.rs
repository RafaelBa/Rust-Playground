fn main() {
    let a = String::from("hello");
    let b = a;
    // println!("a: {}", a);
    // Can't use a here, as it's pointer has been moved to b
    println!("b: {}", b);

    let a = 1;
    let b = a;
    // This works, because a and b are here on the stack, not the heap, therefore it is copied, not moved
    println!("a: {}, b: {}", a, b);

    let s = String::from("hello");
    take_ownership(s);
    // This doesn't work:
    // println!("this is s after taking the ownership: {}", s);
    // The pointer of s is moved into s inside of take_ownership

    let i = 2;
    make_copy(i);
    println!("i: {}", i);  // This works, as the valu is copied

    let s = String::from("hello");
    let t = take_and_return_ownership(s);
    println!("got owernship back for t: {}", t); // got the same pointer from the beginnging back
                                                 // important part is, that ownership was gone in the meanwhile
}

fn take_ownership(s: String) {
    println!("Got ownership for {}", s);
}

fn make_copy(i: u8) {
    println!("Copied value: {}", i);
}

fn take_and_return_ownership(s: String) -> String {
    println!("Got ownership and will return it for {} a", s);
    s
} // ownership and life cycle of variable and pointers ends at closing braces
  // unless they are returned, then the ownership moves to the receiver
