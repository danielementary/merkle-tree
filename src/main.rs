type HashFunction = fn(String) -> String;

fn main() {
    println!("Hello, merkle tree!");
}

fn dummy_hash(input: String) -> String {
    format!("Hash of ({})", input)
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

    fn insert(mut self, value: String) {
        let index = Self::sum_of_powers_of_two(self.height - 1) + self.length;
        let hash = (self.hash_function)(value);

        self.nodes[index] = Some(Node { hash });

        self.length += 1;

        let mut i = index / 2;
        while i > 0 {
            self.nodes[i] = None;
            i /= 2;
        }
    }

    fn get_node_hash(&self, index: usize) -> String {
        match &self.nodes[index] {
            Some(node) => node.hash.clone(),
            None => (self.hash_function)("empty node".to_string()),
        }
    }

    fn update_internal_nodes(&mut self) {
        for i in (0..Self::sum_of_powers_of_two(&self.height - 1)).rev() {
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

    fn get_root(self) -> String {
        match &self.nodes[0] {
            Some(root) => root.hash.clone(),
            None => "error".to_string(),
        }
    }

    fn sum_of_powers_of_two(n: usize) -> usize {
        2 ^ (n + 1) - 1
    }
}
