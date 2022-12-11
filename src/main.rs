fn main() {
    println!("Hello, merkle tree!");
}

struct Node {
    hash: String,
}
struct MerkleTree {
    height: usize,
    length: usize,
    nodes: Vec<Node>,
}
