type Hash = String;
type HashFunction = fn(String) -> Hash;

fn main() {
    println!("Hello, merkle tree!");

    let mut mt = MerkleTree::from_height(dummy_hash, 2);

    mt.insert("Hello".to_string());
    mt.insert("Merkle".to_string());
    mt.insert("Tree".to_string());

    mt.update_internal_nodes();

    println!("root: {}", mt.get_root());

    let value_index = 2;
    let value = mt.get_value(value_index);
    let opening = mt.get_opening(value_index);

    println!("value: {}", value);
    println!("partner: {}", opening.partner_hash);
    println!("root child: {}", opening.root_child_hash);
}

fn dummy_hash(input: String) -> Hash {
    format!("Hash of ({})", input)
}

#[derive(Clone)]
struct Node {
    hash: Hash,
}

struct Opening {
    partner_hash: Hash,
    root_child_hash: Hash,
}

struct MerkleTree {
    hash_function: HashFunction,
    height: usize,
    length: usize,
    first_leaf_node_index: usize,
    nodes: Vec<Option<Node>>,
}

impl MerkleTree {
    fn from_height(hash_function: HashFunction, height: usize) -> Self {
        if height <= 0 || height > 10 {
            panic!(
                "The height of the merkle tree cannot be less or equal to 0 or greater than 10."
            );
        }

        MerkleTree {
            hash_function,
            height,
            length: 0,
            first_leaf_node_index: Self::sum_of_powers_of_two(height - 1) + 1,
            nodes: vec![None; Self::sum_of_powers_of_two(height) + 1],
        }
    }

    fn insert(&mut self, value: String) {
        let next_leaf_node_index = self.first_leaf_node_index + self.length;

        if next_leaf_node_index >= self.nodes.len() {
            panic!("The merkle tree is already full.")
        }

        let hash = (self.hash_function)(value);

        self.nodes[next_leaf_node_index] = Some(Node { hash });
        self.length += 1;

        let mut i = next_leaf_node_index / 2;
        for _ in 0..self.height {
            self.nodes[i] = None;
            i /= 2;
        }
    }

    fn get_node_hash(&self, index: usize) -> Hash {
        if index >= self.nodes.len() {
            panic!("This node is out of bounds.");
        }

        match &self.nodes[index] {
            Some(node) => node.hash.clone(),
            None if index >= self.first_leaf_node_index => {
                (self.hash_function)("empty node".to_string())
            }
            _ => panic!("Internal nodes cannot be None."),
        }
    }

    fn update_internal_nodes(&mut self) {
        for i in (1..self.first_leaf_node_index).rev() {
            if self.nodes[i].is_none() {
                let left_child_index = 2 * i;

                let left_child_hash = self.get_node_hash(left_child_index);
                let right_child_hash = self.get_node_hash(left_child_index + 1);

                let hash =
                    (self.hash_function)(format!("{} | {}", left_child_hash, right_child_hash));

                self.nodes[i] = Some(Node { hash });
            }
        }
    }

    fn get_root(&self) -> Hash {
        self.get_node_hash(1)
    }

    fn get_value(&self, value_index: usize) -> Hash {
        let index = self.first_leaf_node_index + value_index;

        self.get_node_hash(index)
    }

    fn get_opening(&self, value_index: usize) -> Opening {
        let node_index = self.first_leaf_node_index + value_index;

        let partner_index;
        if node_index % 2 == 0 {
            partner_index = node_index + 1;
        } else {
            partner_index = node_index - 1;
        }

        let mut root_child_index = node_index;
        for _ in 0..(self.height - 1) {
            root_child_index /= 2;
        }

        Opening {
            partner_hash: self.get_node_hash(partner_index),
            root_child_hash: self.get_node_hash(root_child_index),
        }
    }

    fn sum_of_powers_of_two(n: usize) -> usize {
        2usize.pow((n + 1) as u32) - 1
    }
}
