use merkle_tree::{Hash, HashFunction, MerkleTree};

fn main() {
    println!("Hello, merkle tree!");

    // define a dummy hash function that hashes "x" into "Hash of (x)"
    let dummy_hash: HashFunction = |input: String| -> Hash { format!("Hash of ({})", input) };

    // create a merkle tree with the given dummy function and height of 2
    let mut mt = MerkleTree::from_height(dummy_hash, 2);

    // insert three elements into the merkle tree
    mt.insert("Hello".to_string());
    mt.insert("Merkle".to_string());
    mt.insert("Tree".to_string());

    // update the state of the internal nodes
    mt.update_internal_nodes();

    // get and print the root's hash of the merkle tree
    println!("root: {}", mt.get_root());

    // get and print the hash corresponding to value "Tree"
    let value_index = 2;
    println!("value {}: {}", value_index, mt.get_value(value_index));
}
