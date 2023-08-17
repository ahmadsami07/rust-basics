#[cfg(test)]
mod block_tests {
    use crate::block::Block;

    #[test]
    //Need to test if block validating functions are working first.
    fn block_validity_testing() {
        let mut n1 = 1234;
        {
            let n2 = &mut n1;
            *n2 = 4567;
            println!("{}", *n2); // prints 4567
        } // n2 is out of scope here, so no longer has a reference
        println!("LAALALALLALLAL");
        println!("Hello {}", n1);
        n1 = 7890;
        println!("{}", n1); // prints 7890
        let mut b0 = Block::initial(10);
        b0.set_proof(391);
        let mut b1 = Block::next(&b0, String::from("TESTING"));
        b1.set_proof(755);
        assert!(b0.is_valid_for_proof(391) == true);
        assert!(b1.is_valid_for_proof(755) == true);
    }

    #[test]
    fn test_mining() {
        // Test mining a block and checking its validity with the mined proof
        let mut b0 = Block::initial(20);
        b0.mine(10);
        let mut b1 = Block::initial(10);
        b1.mine(5);
        let mut b2 = Block::initial(5);
        b2.mine(5);
        let mut b3 = Block::initial(1);
        b3.mine(5);
        let mut b4 = Block::initial(1);
        b4.mine(5);
        assert!(b0.is_valid());
        assert!(b1.is_valid());
        assert!(b2.is_valid());
        assert!(b3.is_valid());
        assert!(b4.is_valid());
    }

    #[test]
    fn block_chain_next_test() {
        let mut b0 = Block::initial(10);
        b0.mine(4);
        let mut b1 = Block::next(&b0, String::from("TESTING"));
        b1.mine(1);
        let mut b2 = Block::next(&b0, String::from("TESTINGAGAIN"));
        b2.mine(1);
        let mut b3 = Block::next(&b0, String::from("TESTING_AGAIN_AGAIN"));
        b3.mine(1);
        assert!(b0.is_valid());
        assert!(b1.is_valid());
        assert!(b2.is_valid());
        assert!(b3.is_valid());
    }

    #[test]
    fn hash_string_test() {
        let mut b0 = Block::initial(10);
        b0.set_proof(123456);
        let mut b1 = Block::next(&b0, String::from("TESTING"));
        b1.set_proof(78910);
        assert!(b1.hash_string()=="175ae88e38f0de689c37dbeba5f6b6eb5a881dab5dbff9d897dc9622fda79ae0:1:10:TESTING:78910");

        let mut b2: Block = Block::initial(7);
        b2.set_proof(88736);
        assert_eq!(
            b2.hash_string(),
            "0000000000000000000000000000000000000000000000000000000000000000:0:7::88736"
        );

        let mut b3: Block = Block::next(&b2, String::from("TESTING_AGAIN"));
        b3.set_proof(73647);
        assert_eq!(
            b3.hash_string(),
            "78f53693dd0993552ad21a36ee66feacce5efec299e277a2125bdd44ffc9434a:1:7:TESTING_AGAIN:73647"
    );
    }
}
