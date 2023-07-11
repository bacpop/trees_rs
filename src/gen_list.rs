
#[derive(Debug)]
pub enum MutationType {
    A,
    C,
    G,
    T,
    M, // A or C
    R, // A or G
    W, // A or T
    S, // C or G
    Y, // C or T
    K, // G or T
    V, // A or C or G
    H, // A or C or T
    D, // A or G or T
    B, // C or G or T
    N, // Any
    Rem, // Removed
}

pub fn mutation_char_to_enum(e: char) -> MutationType {
    match e {
        'A' => MutationType::A,
        'C' => MutationType::C,
        'G' => MutationType::G,
        'T' => MutationType::T,
        'M' => MutationType::M,
        'R' => MutationType::R,
        'W' => MutationType::W,
        'S' => MutationType::S,
        'Y' => MutationType::Y,
        'K' => MutationType::K,
        'V' => MutationType::V,
        'H' => MutationType::H,
        'D' => MutationType::D,
        'B' => MutationType::B,
        'N' => MutationType::N,
        '-' => MutationType::Rem,
        _ => panic!("Unrecognised character in sequence"),
    }
}

pub fn mutation_to_likelihood(i: usize, e: &char) -> (usize, f64, f64, f64, f64) {
    match e {
        // (A, C, G, T)
        'A' => (i, 1.0, 0.0, 0.0, 0.0),
        'C' => (i, 0.0, 1.0, 0.0, 0.0),
        'G' => (i, 0.0, 0.0, 1.0, 0.0),
        'T' => (i, 0.0, 0.0, 0.0, 1.0),
        'Y' => (i, 0.0, 0.5, 0.0, 0.5),
        'W' => (i, 0.5, 0.0, 0.0, 0.5),
        'R' => (i, 0.5, 0.0, 0.5, 0.0),
        'K' => (i, 0.0, 0.0, 0.5, 0.5),
        'S' => (i, 0.0, 0.5, 0.5, 0.0),
        '-' => (i, 0.25, 0.25, 0.25, 0.25),
        _ => panic!("Unrecognised character"),
     }
}

pub fn complement(e: MutationType) -> MutationType {
    match e {
        MutationType::A => MutationType::T,
        MutationType::C => MutationType::G,
        MutationType::G => MutationType::C,
        MutationType::T => MutationType::A,
        MutationType::M => MutationType::K,
        MutationType::R => MutationType::Y,
        MutationType::Y => MutationType::R,
        MutationType::K => MutationType::M,
        MutationType::V => MutationType::B,
        MutationType::H => MutationType::D,
        MutationType::D => MutationType::H,
        MutationType::B => MutationType::V,
        _ => e,
    }
}

#[derive(Debug)]
pub struct Sample {
    pub list: Vec<Entry>,
}

#[derive(Debug)]
pub struct Entry {
    pub element: (usize, f64, f64, f64, f64),
}

pub fn combine_lists(
    seq1: &mut Vec<(usize, f64, f64, f64, f64)>, 
    seq2: &mut Vec<(usize, f64, f64, f64, f64)>) -> Vec<(usize, f64, f64, f64, f64)> {

    let mut out: Vec<(usize, f64, f64, f64, f64)> = Vec::new();

    seq1.reverse();
    seq2.reverse();
    
    let mut j = seq2.pop();
    let mut i = seq1.pop();

    while i.is_some() | j.is_some() {
        
        if j.is_none() {
            out.push(i.unwrap());
            i = seq1.pop();
        } else if i.is_none() {
            out.push(j.unwrap());
            j = seq2.pop();
        } else {
            let i0 = i.unwrap().0;
            let j0 = j.unwrap().0;
            if i0 == j0 {
                out.push((i0,  5.0, 5.0, 5.0, 5.0));
                i = seq1.pop();
                j = seq2.pop();
            } else if i0 < j0 {
                out.push(i.unwrap());
                i = seq1.pop();
            } else {
                out.push(j.unwrap());
                j = seq2.pop();
            }
        }
    }

    out
}

// impl Sample {
//     pub fn add(&mut self, x: Entry) {
//         self.list.push(x);
//     }
// }
    

// impl Entry {
//     pub fn new(mutation: char, i: i64, j: Option<i64>) -> Entry {
//         let converted_mut: MutationType = mutation_char_to_enum(mutation);

//         Entry {
//             element: (converted_mut, i , j),
//         }
//     }

//     pub fn start(&self) -> i64 {
//         self.element.1
//     }

//     pub fn end(&self) -> Option<i64> {
//         self.element.2
//     }

// }
