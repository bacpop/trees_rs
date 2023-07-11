#[derive(Debug, Copy, Clone)]
pub struct Mutation(usize, f64, f64, f64, f64);

pub fn char_to_mutation(i: usize, e: &char) -> Mutation {
    match e {
        // (A, C, G, T)
        'A' => Mutation(i, 1.0, 0.0, 0.0, 0.0),
        'C' => Mutation(i, 0.0, 1.0, 0.0, 0.0),
        'G' => Mutation(i, 0.0, 0.0, 1.0, 0.0),
        'T' => Mutation(i, 0.0, 0.0, 0.0, 1.0),
        'Y' => Mutation(i, 0.0, 0.5, 0.0, 0.5),
        'W' => Mutation(i, 0.5, 0.0, 0.0, 0.5),
        'R' => Mutation(i, 0.5, 0.0, 0.5, 0.0),
        'K' => Mutation(i, 0.0, 0.0, 0.5, 0.5),
        'S' => Mutation(i, 0.0, 0.5, 0.5, 0.0),
        '-' => Mutation(i, 0.25, 0.25, 0.25, 0.25),
        _ => panic!("Unrecognised character"),
     }
}

// Takes a reference sequence and another sequence in SequenceRecord<'_> format
// Returns a vector of Mutations for how the latter sequence differs from the reference
pub fn create_list(refseq: &Vec<char>, seq: &Vec<char>) -> Vec<Mutation> {

    let mut out: Vec<Mutation> = Vec::new();

    for (i, (s1, s2)) in refseq.iter().zip(seq.iter()).enumerate() {
        if s1 != s2 {out.push(char_to_mutation(i, s2));}
    }

    out
}

// Combines two vectors of Mutations into a single vector
pub fn combine_lists(
    seq1: &mut Vec<Mutation>, 
    seq2: &mut Vec<Mutation>) -> Vec<Mutation> {

    let mut out: Vec<Mutation> = Vec::new();

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
                out.push(Mutation(i0,  5.0, 5.0, 5.0, 5.0));
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


// pub fn complement(e: MutationType) -> MutationType {
//     match e {
//         MutationType::A => MutationType::T,
//         MutationType::C => MutationType::G,
//         MutationType::G => MutationType::C,
//         MutationType::T => MutationType::A,
//         MutationType::M => MutationType::K,
//         MutationType::R => MutationType::Y,
//         MutationType::Y => MutationType::R,
//         MutationType::K => MutationType::M,
//         MutationType::V => MutationType::B,
//         MutationType::H => MutationType::D,
//         MutationType::D => MutationType::H,
//         MutationType::B => MutationType::V,
//         _ => e,
//     }
// }

// #[derive(Debug)]
// pub enum MutationType {
//     A,
//     C,
//     G,
//     T,
//     M, // A or C
//     R, // A or G
//     W, // A or T
//     S, // C or G
//     Y, // C or T
//     K, // G or T
//     V, // A or C or G
//     H, // A or C or T
//     D, // A or G or T
//     B, // C or G or T
//     N, // Any
//     Rem, // Removed
// }

// pub fn char_to_mutationtype(e: char) -> MutationType {
//     match e {
//         'A' => MutationType::A,
//         'C' => MutationType::C,
//         'G' => MutationType::G,
//         'T' => MutationType::T,
//         'M' => MutationType::M,
//         'R' => MutationType::R,
//         'W' => MutationType::W,
//         'S' => MutationType::S,
//         'Y' => MutationType::Y,
//         'K' => MutationType::K,
//         'V' => MutationType::V,
//         'H' => MutationType::H,
//         'D' => MutationType::D,
//         'B' => MutationType::B,
//         'N' => MutationType::N,
//         '-' => MutationType::Rem,
//         _ => panic!("Unrecognised character in sequence"),
//     }
// }