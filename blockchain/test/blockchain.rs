#[cfg(test)]
mod tests {
    use crate::blockchain::Chain;

    #[test]
    fn test_create_new_blockchain() {
        let test_miner_address = String::from("test_miner_address");
        let mut chain = Chain::new(test_miner_address, 1);
    }

    #[test]
    fn test_new_transaction() {
        let test_miner_address = String::from("test_miner_address");
        let test_sender = String::from("test_sender");
        let test_receiver = String::from("test_receiver");
        let mut chain = Chain::new(test_miner_address, 1);

        let res = chain.new_transaction(test_sender, test_receiver, 1000.0);

        match res {
            true => (),
            false => panic!("Test for new transaction failed!"),
        }
    }
}
