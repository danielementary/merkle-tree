use merkle_tree::{Hash, HashFunction, MerkleTree};

fn main() {
    println!("Hello, merkle tree!");

    let dummy_hash: HashFunction = |input: String| -> Hash { format!("Hash of ({})", input) };
    let mut mt = MerkleTree::from_height(dummy_hash, 2);

    mt.insert("Hello".to_string());
    mt.insert("Merkle".to_string());
    mt.insert("Tree".to_string());

    mt.update_internal_nodes();

    println!("root: {}", mt.get_root());

    let value_index = 2;
    let value = mt.get_value(value_index);

    println!("value: {}", value);
}
