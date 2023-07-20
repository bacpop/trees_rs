use std::cmp::Ordering;
use needletail::*;

#[derive(Debug, Copy, Clone)]
pub struct Mutation(pub usize, pub f64, pub f64, pub f64, pub f64);

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
        'M' => Mutation(i, 0.5, 0.5, 0.0, 0.0),
        'B' => Mutation(i, 0.0, 1.0/3.0, 1.0/3.0, 1.0/3.0),
        'D' => Mutation(i, 1.0/3.0, 0.0, 1.0/3.0, 1.0/3.0),
        'V' => Mutation(i, 1.0/3.0, 1.0/3.0, 1.0/3.0, 0.0),
        // This is currently incorrect. Not sure how to handle gaps.
        '-' => Mutation(i, 0.25, 0.25, 0.25, 0.25),
        _ => panic!("Unrecognised character: {}", e),
    }
}

// Takes a reference sequence and another sequence in SequenceRecord<'_> format
// Returns a vector of Mutations for how the latter sequence differs from the reference
pub fn create_list(refseq: &[char], seq: &[char]) -> Vec<Mutation> {
    let mut out: Vec<Mutation> = Vec::new();

    for (i, (s1, s2)) in refseq.iter().zip(seq.iter()).enumerate() {
        if s1 != s2 {
            out.push(char_to_mutation(i, s2));
        }
    }

    out
}

// Combines two vectors of Mutations into a single vector
pub fn combine_lists(seq1: Option<&Vec<Mutation>>, seq2: Option<&Vec<Mutation>>) -> Vec<Mutation> {
    let mut out: Vec<Mutation> = Vec::new();
    let seq1 = seq1.unwrap();
    let seq2 = seq2.unwrap();

    // Index in each vector of mutations
    let mut s1_i = seq1.len() - 1;
    let mut s2_i = seq2.len() - 1;

    // Mutation at each index
    let mut s1_node = seq1.get(s1_i);
    let mut s2_node = seq2.get(s2_i);

    // Location of mutation in sequence
    let mut s1_loc = s1_node.unwrap().0;
    let mut s2_loc = s2_node.unwrap().0;

    while (s1_i > 0) | (s2_i > 0) {
        match s1_loc.cmp(&s2_loc) {
            Ordering::Equal => {
                // There should be a step here to calculate combined likelihoods
                out.push(Mutation(s1_loc, 5.0, 5.0, 5.0, 5.0));
                
                s1_i -= 1;
                s1_node = seq1.get(s1_i);
                s1_loc = s1_node.unwrap().0;

                s2_i -= 1;
                s2_node = seq2.get(s2_i);
                s2_loc = s2_node.unwrap().0;
            },
            Ordering::Greater => {
                out.push(*s1_node.unwrap());
                s1_i -= 1;
                s1_node = seq1.get(s1_i);
                s1_loc = s1_node.unwrap().0;
            },
            Ordering::Less => {
                out.push(*s2_node.unwrap());
                s2_i -= 1;
                s2_node = seq2.get(s2_i);
                s2_loc = s2_node.unwrap().0;
            }
        }
    }

    // Push last entries
    match s1_loc.cmp(&s2_loc) {
        Ordering::Equal => {
            out.push(Mutation(s1_loc, 5.0, 5.0, 5.0, 5.0));},
        Ordering::Greater => {
            out.push(*s1_node.unwrap());
            out.push(*s2_node.unwrap());},
        Ordering::Less => {
            out.push(*s2_node.unwrap());
            out.push(*s1_node.unwrap());
        },
    }

    out.reverse();

    out
}

#[derive(Debug)]
pub struct GeneticData {
    pub likelihood_lists: Vec<Vec<Mutation>>,
}

pub fn create_genetic_data(filename: &str) -> GeneticData {
    let mut reader = parse_fastx_file(filename).expect("File error");

    // For now take first sequence as reference sequence
    let record = reader.next().unwrap().unwrap();
    let seq_vec:Vec<char> = record.seq().iter().map(|l| *l as char).collect();

    let mut ll: GeneticData = GeneticData {likelihood_lists: Vec::new()};

    while let Some(rec) = reader.next() {
        let newrec: Vec<char> = rec.unwrap().seq().iter().map(|l| *l as char).collect();
        ll.likelihood_lists.push(create_list(&seq_vec, &newrec));
    }

    ll
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
