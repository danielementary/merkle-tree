type HashFunction = fn(String) -> String;

fn main() {
    println!("Hello, merkle tree!");
}

fn dummy_hash(input: String) -> String {
    "Hash of ({input})".to_string()
}

#[derive(Clone)]
struct Node {
    hash: String,
}

struct MerkleTree {
    hash_function: HashFunction,
    height: usize,
    length: usize,
    nodes: Vec<Option<Node>>,
}

impl MerkleTree {
    fn from_height(hash_function: HashFunction, height: usize) -> Self {
        MerkleTree {
            hash_function,
            height,
            length: 0,
            nodes: vec![None; Self::sum_of_powers_of_two(height)],
        }
    }

    fn sum_of_powers_of_two(n: usize) -> usize {
        2 ^ (n + 1) - 1
    }
}
