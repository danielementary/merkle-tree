pub type Hash = String;
pub type HashFunction = fn(String) -> Hash;

#[derive(Clone)]
struct Node {
    hash: Hash,
}

pub struct MerkleTree {
    hash_function: HashFunction,
    height: usize,
    length: usize,
    first_leaf_node_index: usize,
    nodes: Vec<Option<Node>>,
}

impl MerkleTree {
    pub fn from_height(hash_function: HashFunction, height: usize) -> Self {
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

    pub fn insert(&mut self, value: String) {
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

    pub fn update_internal_nodes(&mut self) {
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

    pub fn get_root(&self) -> Hash {
        self.get_node_hash(1)
    }

    pub fn get_value(&self, value_index: usize) -> Hash {
        let index = self.first_leaf_node_index + value_index;

        self.get_node_hash(index)
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

    fn sum_of_powers_of_two(n: usize) -> usize {
        2usize.pow((n + 1) as u32) - 1
    }
}
