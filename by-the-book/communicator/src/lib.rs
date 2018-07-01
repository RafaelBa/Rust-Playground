pub mod client;
pub mod network;


#[cfg(test)]
mod tests {
    use super::client;   // `super` references the current parent object
    // use ::client;     // `::` at the first position means we want to start at root
    
    #[test]
    fn it_works() {
        client::connect();
    }
}
