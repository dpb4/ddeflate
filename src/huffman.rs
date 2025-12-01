use bitvec::prelude::*;
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug)]
pub enum HuffmanNode<T> {
    InternalNode(f64, (Box<HuffmanNode<T>>, Box<HuffmanNode<T>>)),
    LeafNode(f64, T),
}

pub trait Encodable: Eq + Ord + std::hash::Hash {}

impl<T: Eq + Ord + std::hash::Hash> Encodable for T {}

impl<T: Encodable> HuffmanNode<T> {
    pub fn weight(&self) -> f64 {
        match self {
            HuffmanNode::InternalNode(w, _) => *w,
            HuffmanNode::LeafNode(w, _) => *w,
        }
    }
}

impl<T: Encodable> Ord for HuffmanNode<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.weight().total_cmp(&other.weight())
    }
}

impl<T: Encodable> PartialOrd for HuffmanNode<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Encodable> PartialEq for HuffmanNode<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::InternalNode(l0, l1), Self::InternalNode(r0, r1)) => l0 == r0 && l1 == r1,
            (Self::LeafNode(l0, l1), Self::LeafNode(r0, r1)) => l0 == r0 && l1 == r1,
            _ => false,
        }
    }
}

impl<T: Encodable> Eq for HuffmanNode<T> {}

fn huffman_treeify<T: Encodable>(leaves: Vec<HuffmanNode<T>>) -> HuffmanNode<T> {
    let mut pq = BinaryHeap::from(leaves);

    while pq.len() > 1 {
        let left = pq.pop().unwrap();
        let right = pq.pop().unwrap();
        let new_internal_node = HuffmanNode::InternalNode(
            left.weight() + right.weight(),
            (Box::new(left), Box::new(right)),
        );
        pq.push(new_internal_node);
    }

    pq.pop().unwrap()
}

pub fn get_encoding_map<T: Encodable>(ht_root: HuffmanNode<T>) -> HashMap<T, BitVec> {
    let mut encoding = HashMap::new();
    _get_encoding_map_acc(ht_root, &BitVec::new(), &mut encoding);

    encoding
}

fn _get_encoding_map_acc<T: Encodable>(
    ht_node: HuffmanNode<T>,
    path: &BitVec,
    encoding: &mut HashMap<T, BitVec>,
) {
    match ht_node {
        HuffmanNode::InternalNode(_, (left, right)) => {
            let mut left_path = path.clone();
            left_path.push(false);
            let mut right_path = path.clone();
            right_path.push(true);
            _get_encoding_map_acc(*left, &mut left_path, encoding);
            _get_encoding_map_acc(*right, &mut right_path, encoding);
        }
        HuffmanNode::LeafNode(_, item) => {
            encoding.insert(item, path.clone());
        }
    }
}

pub fn decode<T: Encodable>(coding: &BitSlice, ht_node: HuffmanNode<T>) -> T {
    if coding.len() > 1 {
        let (l, r) = match ht_node {
            HuffmanNode::InternalNode(_, (l, r)) => (*l, *r),
            _ => panic!("decode failure"),
        };
        match coding[0] {
            false => decode(&coding[1..], l),
            true => decode(&coding[1..], r),
        }
    } else {
        match ht_node {
            HuffmanNode::LeafNode(_, item) => item,
            HuffmanNode::InternalNode(_, _) => panic!("decode failure"),
        }
    }
}

pub fn canonicalize<T: Encodable + Clone>(ht_map: &HashMap<T, BitVec>) -> HashMap<T, BitVec> {
    let mut sorted = ht_map.into_iter().collect::<Vec<(&T, &BitVec)>>();
    sorted.sort_by(|a, b| a.0.cmp(b.0));

    let max_length = sorted.last().unwrap().1.len();
    let mut bit_length_counts: HashMap<usize, usize> = HashMap::new();
    for (_, s) in ht_map {
        bit_length_counts
            .entry(s.len())
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    bit_length_counts.insert(0, 0);

    dbg!(&bit_length_counts);
    // code = 0;
    // bl_count[0] = 0;
    // for (bits = 1; bits <= MAX_BITS; bits++) {
    //     code = (code + bl_count[bits-1]) << 1;
    //     next_code[bits] = code;
    // }

    let mut code = 0;
    let mut next_code: HashMap<usize, usize> = HashMap::new();
    for bits in 1..=max_length {
        if let Some(bc) = bit_length_counts.get(&(bits - 1)) {
            code = (code + bc) << 1;
            next_code.insert(bits, code);
        } else {
            code <<= 1;
            next_code.insert(bits, code);
        }
    }
    dbg!(&next_code);
    // for (n = 0;  n <= max_code; n++) {
    //     len = tree[n].Len;
    //     if (len != 0) {
    //         tree[n].Code = next_code[len];
    //         next_code[len]++;
    //     }
    // }

    let mut canonical_map = HashMap::new();
    for p in sorted {
        let len = ht_map[p.0].len();
        println!("{}", &len);
        let mut coded = bitvec![0; len];
        coded.store(next_code[&len]);
        canonical_map.insert(p.0.clone(), coded);
        next_code
            .entry(len)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    canonical_map
}
