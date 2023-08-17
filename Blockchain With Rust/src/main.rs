use a3::block::Block;
use a3::queue::{Task, WorkQueue};
use digest::consts::U32;
use sha2::digest::generic_array::GenericArray;
use sha2::{Digest, Sha256};
use std::fmt::Write;
use std::sync;

type Hash = GenericArray<u8, U32>;

fn main() {
    // let mut b0 = Block::initial(16);
    // // // or b0.set_proof(56231);
    // println!("CHECKING {}", b0.is_valid_for_proof(56231));
    // b0.set_proof(56231);
    // println!("is valid : {}", b0.is_valid());

    // let mut b0 = Block::initial(19);
    // b0.set_proof(87745);
    // let mut b1 = Block::next(&b0, String::from("hash example 1234"));
    // b1.set_proof(346082);
    // println!("is valid : {}", b1.is_valid());

    // let mut b1 = Block::next(&b0, String::from("message"));
    // b1.set_proof(2159); // or b1.set_proof(2159);
    // let mut b2 = Block::next(&b1, String::from("messenger"));
    // b2.set_proof(2159);

    // let mut b0 = Block::initial(19);
    // println!("CHECKING {}", b0.is_valid());
    // b0.set_proof(87745);
    // println!("CHECKING {}", b0.is_valid());

    // let mut b1 = Block::next(&b0, String::from("hash example 1234"));
    // b1.set_proof(1407891);
    // println!("CHECKING {}", b1.is_valid());
    // let mut b0 = Block::initial(10);
    // b0.mine(1);
    // println!("{}", b0.hash_string());
    // println!("{:02x}", b0.hash());
    // let mut b1 = Block::next(&b0, String::from("this is an interesting message"));
    // b1.mine(1);
    // println!("{}", b1.hash_string());
    // println!("{:02x}", b1.hash());
    // let mut b2 = Block::next(&b1, String::from("this is not interesting"));
    // b2.mine(1);
    // println!("{}", b2.hash_string());
    // println!("{:02x}", b2.hash());

    // let mut b0 = Block::initial(9);
    // b0.mine(2);

    // assert_eq!(
    //     b0_hash_to_string,
    //     "fd6ac46407cc53176559b0fe224e0168d1d10aea69f201e4afbfbc6ad131563d"
    // ); //test hash()

    // let mut b1: Block = Block::next(&b0, String::from("Basic Test"));
    // b1.set_proof(1101);
    // // b1 hash string: fd6ac46407cc53176559b0fe224e0168d1d10aea69f201e4afbfbc6ad131563d:1:10:Basic Test:1101
    // // b1 hash: 07c055b0ccc6811694643b226bd5c0e17d86b9144c1fdf2444720e8ab60cdf51
    // assert_eq!(
    //     b1.hash_string(),
    //     "fd6ac46407cc53176559b0fe224e0168d1d10aea69f201e4afbfbc6ad131563d:1:10:Basic Test:1101"
    // ); //test hash_string()

    // let b1_hash_for_proof_to_string = hash_to_string(b1.hash_for_proof(1101));
    // // hash of b1 with proof 1101 = 07c055b0ccc6811694643b226bd5c0e17d86b9144c1fdf2444720e8ab60cdf51
    // assert_eq!(
    //     b1_hash_for_proof_to_string,
    //     "07c055b0ccc6811694643b226bd5c0e17d86b9144c1fdf2444720e8ab60cdf51"
    // ); // test hash_for_proof(proof)

    // let b2: Block = Block::next(&b1, String::from("Hash This String"));
    // // b2 with proof = 98756 hash string: 07c055b0ccc6811694643b226bd5c0e17d86b9144c1fdf2444720e8ab60cdf51:2:10:Hash This String:98756
    // assert_eq!(
    //         b2.hash_string_for_proof(98756),
    //         "07c055b0ccc6811694643b226bd5c0e17d86b9144c1fdf2444720e8ab60cdf51:2:10:Hash This String:98756"
    //     ); // test hash_string_for_proof(proof)
    // println!("{}", b0.hash_string());
    // println!("{:02x}", b0.hash());
    // let mut b1 = Block::next(&b0, String::from("this is an interesting message"));
    // b1.mine(1);
    // println!("{}", b1.hash_string());
    // println!("{:02x}", b1.hash());
    // let mut b2 = Block::next(&b1, String::from("this is not interesting"));
    // b2.mine(1);
    // println!("{}", b2.hash_string());
    // println!("{:02x}", b2.hash());

    // // Nothing is required here, but it may be useful for testing.
    // let mut b0 = Block::initial(10);
    // b0.mine(5);
    // println!("{}", b0.hash_string());
    // println!("{:02x}", b0.hash());
    // let mut b1 = Block::next(&b0, String::from("TESTING"));
    // b1.mine(5);
    // println!("{}", b1.hash_string());
    // println!("{:02x}", b1.hash());
    // let mut b2 = Block::next(&b1, String::from("this is not interesting"));
    // b2.mine(5);
    // println!("{}", b2.hash_string());
    // println!("{:02x}", b2.hash());
}
