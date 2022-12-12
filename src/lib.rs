pub type Hash = String;
pub type HashFunction = fn(String) -> Hash;

/// a single node of the merkle tree
#[derive(Clone)]
struct Node {
    /// a node only contains the hash corresponding to its position in the merkle tree
    hash: Hash,
}

pub struct MerkleTree {
    hash_function: HashFunction,
    height: usize,
    /// the `length` represents the number of elements inserted in the merkle tree
    length: usize,
    /// `first_leaf_node_index` corresponds to
    /// the index of the first inserted element in the merkle tree
    first_leaf_node_index: usize,
    /// the merkle tree is implemented as a linear array of `Option<Node>`,
    /// where the nodes are sorted in a breadth first fashion
    /// `nodes[0]` is always `None`
    /// `nodes[1]` is either None or `Some(node)`, where `node` is the root of the merkle tree
    /// the following nodes up to `first_leaf_node_index` correspond to
    /// the internal nodes of the merkle tree followed by
    /// the external nodes, which correspond to the hash of the inserted values
    nodes: Vec<Option<Node>>,
}

impl MerkleTree {
    /// create an empty merkle tree with the provided `hash_function` and `height`
    ///
    /// panic if the height is less or equal to 0 or greater than 10
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
            // internal nodes are stored from 1 to 1 + 2 + ... + 2^(height - 1) + 1
            first_leaf_node_index: Self::sum_of_powers_of_two(height - 1) + 1,
            // we need 1 empty slot for the first `None` + 1 + 2 + ... 2^height to store the merkle tree
            nodes: vec![None; Self::sum_of_powers_of_two(height) + 1],
        }
    }

    /// insert a new value into the merkle tree and
    /// set to `None` all the corresponding parents up to the root
    ///
    /// panic if the merkle tree is already full
    pub fn insert(&mut self, value: String) {
        let next_leaf_node_index = self.first_leaf_node_index + self.length;

        if next_leaf_node_index >= self.nodes.len() {
            panic!("The merkle tree is already full.")
        }

        let hash = (self.hash_function)(value);

        self.nodes[next_leaf_node_index] = Some(Node { hash });
        self.length += 1;

        // find parents up to the root
        // by iteratively performing an integer division of the index by 2 and
        // set them to `None`
        let mut i = next_leaf_node_index;
        for _ in 0..self.height {
            i /= 2;
            self.nodes[i] = None;
        }
    }

    /// update the state of the internal nodes
    /// by computing iteratively from the last internal node to the root
    pub fn update_internal_nodes(&mut self) {
        for i in (1..self.first_leaf_node_index).rev() {
            // compute only nodes set to `None`
            if self.nodes[i].is_none() {
                let left_child_index = 2 * i;

                let left_child_hash = self.get_node_hash(left_child_index);
                let right_child_hash = self.get_node_hash(left_child_index + 1);

                // the hash of a node is the hash of the concatenation of its children's hashes
                let hash =
                    (self.hash_function)(format!("{} | {}", left_child_hash, right_child_hash));

                self.nodes[i] = Some(Node { hash });
            }
        }
    }

    /// get the root hash
    ///
    /// panic if the root is `None`
    pub fn get_root(&self) -> Hash {
        self.get_node_hash(1)
    }

    /// get the node hash corresponding to the `value_index`'th value inserted into the merkle tree
    ///
    /// panic if the node is out of bounds
    pub fn get_value(&self, value_index: usize) -> Hash {
        let index = self.first_leaf_node_index + value_index;

        self.get_node_hash(index)
    }

    /// get a node hash corresponding to its position in the merkle tree
    ///
    /// panic if the node is out of bounds or
    /// if the node is internal and `None`, i.e. not computed with `update_internal_nodes`
    fn get_node_hash(&self, index: usize) -> Hash {
        if index >= self.nodes.len() {
            panic!("This node is out of bounds.");
        }

        match &self.nodes[index] {
            Some(node) => node.hash.clone(),
            // if the node is external and was not inserted yet, it is replaced by "empty node" hash
            None if index >= self.first_leaf_node_index => {
                (self.hash_function)("empty node".to_string())
            }
            _ => panic!("Internal nodes cannot be None."),
        }
    }

    /// formula to compute 1 + 2 + ... + 2^n
    fn sum_of_powers_of_two(n: usize) -> usize {
        2usize.pow((n + 1) as u32) - 1
    }
}
