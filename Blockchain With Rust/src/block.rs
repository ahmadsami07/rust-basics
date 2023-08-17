use crate::queue::{Task, WorkQueue};
use digest::consts::U32;
use sha2::digest::generic_array::GenericArray;
use sha2::{Digest, Sha256};
use std::fmt::Write;
use std::sync;

type Hash = GenericArray<u8, U32>;

#[derive(Debug, Clone)]

pub struct Block {
    prev_hash: Hash,
    generation: u64,
    difficulty: u8,
    data: String,
    proof: Option<u64>,
}

impl Block {
    pub fn initial(difficulty: u8) -> Block {
        // create and return a new initial block
        return Block {
            generation: 0,
            difficulty,
            data: String::new(),
            prev_hash: Hash::default(),
            proof: None,
        };
    }

    pub fn next(previous: &Block, data: String) -> Block {
        //Trying to calculate previous hash
        let previous_hash = Self::hash_for_proof(previous, previous.proof.unwrap());
        let mut prev_hash_hex = String::new();

        for prev_val in previous_hash.iter() {
            write!(&mut prev_hash_hex, "{:02x}", prev_val);
        }
        // create and return a block that could follow `previous` in the chain
        return Block {
            prev_hash: previous_hash,
            generation: previous.generation + 1,
            difficulty: previous.difficulty,
            data: data,
            proof: None,
        };
    }
    pub fn hash_string_for_proof(&self, proof: u64) -> String {
        let mut prev_hash_hex = String::new();

        for prev_val in self.prev_hash.iter() {
            write!(&mut prev_hash_hex, "{:02x}", prev_val);
        }
        let mut hash_string = String::new();
        write!(
            &mut hash_string,
            "{}:{}:{}:{}:{}",
            prev_hash_hex, self.generation, self.difficulty, self.data, proof
        );
        return hash_string;
    }

    pub fn hash_string(&self) -> String {
        // self.proof.unwrap() panics if block not mined
        let p = self.proof.unwrap();
        self.hash_string_for_proof(p)
    }

    pub fn hash_for_proof(&self, proof: u64) -> Hash {
        // return the block's hash as it would be if we set the proof to `proof`.
        let mut prev_hash_hex = String::new();

        for prev_val in self.prev_hash.iter() {
            write!(&mut prev_hash_hex, "{:02x}", prev_val);
        }
        let mut hash_string = String::new();
        write!(
            &mut hash_string,
            "{}:{}:{}:{}:{}",
            prev_hash_hex, self.generation, self.difficulty, self.data, proof
        );
        let mut hasher = Sha256::new();
        hasher.update(hash_string);
        let hash_result = hasher.finalize();
        return hash_result;
    }

    pub fn hash(&self) -> Hash {
        // self.proof.unwrap() panics if block not mined
        let p = self.proof.unwrap();
        self.hash_for_proof(p)
    }

    pub fn set_proof(self: &mut Block, proof: u64) {
        self.proof = Some(proof);
    }

    pub fn is_valid_for_proof(&self, proof: u64) -> bool {
        let hash = self.hash_for_proof(proof);
        let n_bytes = (self.difficulty / 8) as usize;
        let n_bits = (self.difficulty % 8) as usize;

        for i in 32 - n_bytes..32 {
            if hash[i] != 0u8 {
                return false;
            }
        }

        let difficulty_byte = hash[32 - n_bytes - 1];

        if n_bits > 0 {
            let divide = difficulty_byte % (1 << n_bits);
            if divide != 0 {
                return false;
            }
        }
        true
    }

    pub fn is_valid(&self) -> bool {
        if self.proof.is_none() {
            return false;
        }
        self.is_valid_for_proof(self.proof.unwrap())
    }

    // Mine in a very simple way: check sequentially until a valid hash is found.
    // This doesn't *need* to be used in any way, but could be used to do some mining
    // before your .mine is complete. Results should be the same as .mine (but slower).
    pub fn mine_serial(self: &mut Block) {
        let mut p = 0u64;
        while !self.is_valid_for_proof(p) {
            p += 1;
        }
        self.proof = Some(p);
    }

    pub fn mine_range(self: &Block, workers: usize, start: u64, end: u64, chunks: u64) -> u64 {
        // With `workers` threads, check proof values in the given range, breaking up
        // into `chunks` tasks in a work queue. Return the first valid proof found.
        // HINTS:
        // - Create and use a queue::WorkQueue.
        // - Use sync::Arc to wrap a clone of self for sharing.
        let mut work_queue = WorkQueue::new(workers);
        let each_chunk_size = (end - start) / chunks;

        for i in 0..chunks {
            let starting_chunk = start + i * each_chunk_size;
            let mut ending_chunk = 0;
            // Need to check for last chunkkk
            if i == chunks - 1 {
                ending_chunk = end
            } else {
                ending_chunk = starting_chunk + each_chunk_size
            }

            let mining_task =
                MiningTask::new(sync::Arc::new(self.clone()), starting_chunk, ending_chunk);
            work_queue.enqueue(mining_task).unwrap();
        }
        // Checking if proof is returned from any chunk
        let mut result = None;
        loop {
            let proof = work_queue.recv();
            result = Some(proof);
            println!("Result is{}", proof);
            work_queue.shutdown();
            break;
        }

        result.unwrap()
    }

    pub fn mine_for_proof(self: &Block, workers: usize) -> u64 {
        let range_start: u64 = 0;
        let range_end: u64 = 8 * (1 << self.difficulty); // 8 * 2^(bits that must be zero)
        let chunks: u64 = 2345;
        self.mine_range(workers, range_start, range_end, chunks)
    }

    pub fn mine(self: &mut Block, workers: usize) {
        self.proof = Some(self.mine_for_proof(workers));
    }
}

struct MiningTask {
    block: sync::Arc<Block>,
    start: u64,
    end: u64,
}

impl MiningTask {
    pub fn new(block: sync::Arc<Block>, start: u64, end: u64) -> MiningTask {
        return MiningTask { block, start, end };
    }
}

impl Task for MiningTask {
    type Output = u64;

    fn run(&self) -> Option<u64> {
        println!("TASK START {}", self.start);
        println!("TASK END {}", self.end);

        for proof in self.start..=self.end {
            if self.block.is_valid_for_proof(proof) {
                println!("PROOF FOUND...{}", proof);
                return Some(proof);
            }
        }
        println!("PROOF NOT FOUND...");
        None
    }
}
