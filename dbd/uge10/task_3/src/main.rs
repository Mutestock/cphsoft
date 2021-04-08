use itertools::Itertools;
use std::collections::HashMap;

#[derive(Clone, PartialEq, Eq, Debug)]
struct Node {
    character: Option<char>,
    value: Option<usize>,
    frequency: Option<usize>,
    /// Reallocation of data from the stack to the heap.
    /// Note that leaf nodes will always be in the bottom of the hierarchy.
    left_node: Option<Box<Node>>,
    right_node: Option<Box<Node>>,
}

/// Alias for a table containing characters and frequence of appearance.
type FrequencyTable = HashMap<char, usize>;

impl Node {
    fn generate(input: &str) -> Node {
        let mut frequency_table: FrequencyTable = HashMap::with_capacity(input.len());

        // Populating hashmap with characters as keys and frequency as values.
        // Key duplicates increases the value instead.

        for c in input.to_lowercase().chars() {
            *frequency_table.entry(c).or_insert(0) += 1;
        }

        // Reaping the benefits of vectors.

        let mut count_vec: Vec<(&char, &usize)> = frequency_table.iter().collect();

        // Sorting highest to lowest frequency.

        count_vec.sort_by(|a, b| b.1.cmp(a.1));

        // Check two values at a time starting at index 0 and index 1.

        let complete_tree = Self::build_tree(None, &mut count_vec);
        complete_tree.unwrap()
    }

    // Need a case for split without two available indices.

    fn build_tree(top_node: Option<Node>, entries: &mut Vec<(&char, &usize)>) -> Option<Node> {
        if !entries.is_empty() {
            match top_node {
                Some(node) => {
                    let mut node_buffer: Option<Node> = None;

                    if entries[0].1 < &node.value? && entries.len() > 2 {
                        node_buffer = Self::build_tree(None, entries);
                        let unwrapped_node_buffer = node_buffer.clone()?;
                        let frequency_sum = &unwrapped_node_buffer.value? + node.value?;

                        let value_node =
                            Self::new_value_node(frequency_sum, node, unwrapped_node_buffer);
                        node_buffer = Some(value_node.clone());
                        return Self::build_tree(Some(value_node), entries);
                    }
                    else if !entries.is_empty() {
                        let leaf_node = Self::new_leaf_from_entries(entries);
                        let frequency_sum = leaf_node.frequency? + node.value?;
                        match node_buffer {
                            Some(n) => {
                                return Self::build_tree(Some(n), entries)
                            },
                            None => {
                                let value_node =
                                    Self::new_value_node(frequency_sum, leaf_node, node);
                                return Self::build_tree(Some(value_node), entries);
                            }
                        }
                    } else {
                        return node_buffer;
                    }
                }
                None => {
                    if entries.len() > 2 {
                        // Creating bottom leaf nodes.

                        let leaf_node1 = Self::new_leaf_from_entries(entries);
                        let leaf_node2 = Self::new_leaf_from_entries(entries);

                        let frequency_sum =
                            leaf_node1.frequency? + leaf_node2.frequency?;

                        let value_node =
                            Self::new_value_node(frequency_sum, leaf_node1, leaf_node2);

                        return Self::build_tree(Some(value_node), entries);
                    } else {
                        panic!(
                            "You need at least two characters in your string to use this program."
                        );
                    }
                }
            }
        } else {
            top_node
        }
    }
    fn new_leaf(character: char, frequency: usize) -> Self {
        return Self {
            character: Some(character),
            value: None,
            frequency: Some(frequency),
            left_node: None,
            right_node: None,
        };
    }

    fn new_leaf_from_entries(entries: &mut Vec<(&char, &usize)>) -> Self {
        let node_entry = entries[entries.len() - 1];
        let node2 = Self::new_leaf(*node_entry.0, *node_entry.1);
        entries.pop();
        node2
    }

    fn new_value_node(value: usize, left_node: Node, right_node: Node) -> Self {
        return Self {
            character: None,
            value: Some(value),
            frequency: None,
            left_node: Some(Box::new(left_node)),
            right_node: Some(Box::new(right_node)),
        };
    }

    pub fn set_bit_string_thing(node: Box<Node>, annotations: &mut Vec<(char, (usize, &[u8]))>) {
        if node.character != None {
            if annotations.len() == 0 {
                annotations.push((node.character.unwrap(), (node.frequency.unwrap(), &[000])));
            }
            else{
                let stuff = annotations[annotations.len()-1];
                let trash = stuff.1;
                let flerp = trash.1.len();
                let mut garbage = trash.1[flerp];
                garbage = garbage + 1;
                let shite = (node.character.unwrap(), (node.frequency.unwrap(), &[garbage]));
            }
        }
        else{
            Self::set_bit_string_thing(node.left_node.unwrap(), annotations);
            Self::set_bit_string_thing(node.right_node.unwrap(), annotations);
        }
    }

    pub fn compress(input: &str)
     //-> (&[u8], &[u8]) 
     {
        let tree = Node::generate(input);
        let mut annotations = Vec::new();
        Self::set_bit_string_thing(Box::new(tree), &mut annotations);
        println!("{:#?}", annotations);
    }

    pub fn decompress(&self) -> String {
        todo!()
    }
}

fn main() {
    let input = "beeps beeps!!!!! their eerie ears hear pears";
    let tree: Node = Node::generate(input);
    let tree_res = format!("{:#?}", tree);
    //println!("{:#?}", tree);
    //println!("{}", tree_res);
    let stuff = "pete is here";
    let tree02 = Node::generate(stuff);
    //println!("{:#?}", tree02);
    let mut res: u32 = 0;
    for byte in input.as_bytes() {
        res = res + byte.clone() as u32;
    }
    println!("{}",res);
    let input03 = "We weep for the blood of a bird, but not for the blood of a fish. Blessed are those who have voice.";
    let tree03 = Node::generate(input03);
    println!("{:#?}", tree03);
}
