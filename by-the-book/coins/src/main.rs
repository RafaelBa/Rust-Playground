#[derive(Copy, Clone, Debug)]
enum UsState {
    Alabama,
    Alaska
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn main() {
    let coin1 = Coin::Quarter(UsState::Alaska);

    // exhaustive match with enum destructuring
    let amount = match coin1 {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("A quarter from {:?}", state);
            25
        }
    };

    // if let for one armed match
    let is_alaska_coin = if let Coin::Quarter(UsState::Alaska) = coin1 {
        true
    } else {
        false
    };

    // if let for handling one case and ignoring every other case
    if let Coin::Quarter(UsState::Alaska) = coin1 {
        println!("We have an Alaska coin");
    };

    println!("coin: {:#?}", coin1);
    println!("coin's worth: {}", amount);
    println!("The coin is from Alaska: {}", is_alaska_coin);
}
