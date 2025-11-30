use std::collections::{BinaryHeap, HashMap};

#[derive(Debug)]
pub enum HuffmanNode<T> {
    InternalNode(f64, (Box<HuffmanNode<T>>, Box<HuffmanNode<T>>)),
    LeafNode(f64, T),
}

impl<T: Eq + Ord + std::hash::Hash> HuffmanNode<T> {
    pub fn weight(&self) -> f64 {
        match self {
            HuffmanNode::InternalNode(w, _) => *w,
            HuffmanNode::LeafNode(w, _) => *w,
        }
    }
}

impl<T: Eq + Ord + std::hash::Hash> Ord for HuffmanNode<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.weight().total_cmp(&other.weight())
    }
}

impl<T: Eq + Ord + std::hash::Hash> PartialOrd for HuffmanNode<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Eq + Ord + std::hash::Hash> PartialEq for HuffmanNode<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::InternalNode(l0, l1), Self::InternalNode(r0, r1)) => l0 == r0 && l1 == r1,
            (Self::LeafNode(l0, l1), Self::LeafNode(r0, r1)) => l0 == r0 && l1 == r1,
            _ => false,
        }
    }
}

impl<T: Eq + Ord + std::hash::Hash> Eq for HuffmanNode<T> {}

fn huffman_treeify<T: Eq + Ord + std::hash::Hash>(leaves: Vec<HuffmanNode<T>>) -> HuffmanNode<T> {
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

pub fn get_encoding_map<T: Eq + Ord + std::hash::Hash>(
    ht_root: HuffmanNode<T>,
) -> HashMap<T, String> {
    let mut encoding = HashMap::new();
    _get_encoding_map_acc(ht_root, String::new(), &mut encoding);

    encoding
}

fn _get_encoding_map_acc<T: Eq + Ord + std::hash::Hash>(
    ht_node: HuffmanNode<T>,
    path: String,
    encoding: &mut HashMap<T, String>,
) {
    match ht_node {
        HuffmanNode::InternalNode(_, (left, right)) => {
            _get_encoding_map_acc(*left, path.clone() + "0", encoding);
            _get_encoding_map_acc(*right, path + "1", encoding);
        }
        HuffmanNode::LeafNode(_, item) => {
            encoding.insert(item, path);
        }
    }
}

pub fn decode<T: Eq + Ord + std::hash::Hash>(coding: &[char], ht_node: HuffmanNode<T>) -> T {
    if coding.len() > 1 {
        let (l, r) = match ht_node {
            HuffmanNode::InternalNode(_, (l, r)) => (*l, *r),
            _ => panic!("decode failure"),
        };
        match coding[0] {
            '0' => decode(&coding[1..], l),
            '1' => decode(&coding[1..], r),
            _ => unreachable!(),
        }
    } else {
        match ht_node {
            HuffmanNode::LeafNode(_, item) => item,
            HuffmanNode::InternalNode(_, _) => panic!("decode failure"),
        }
    }
}

pub fn canonicalize<T: Eq + Ord + Clone + std::hash::Hash>(
    ht_map: &HashMap<T, String>,
) -> HashMap<T, String> {
    let mut sorted = ht_map.into_iter().collect::<Vec<(&T, &String)>>();
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
        canonical_map.insert(
            p.0.clone(),
            format!("{:01$b}", next_code[&len], len).to_string(),
        );
        next_code
            .entry(len)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    canonical_map
}
