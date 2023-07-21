use std::cmp::Ordering;
use needletail::*;

#[derive(Debug, Copy, Clone)]
pub struct Mutation(pub usize, pub f64, pub f64, pub f64, pub f64);

impl Mutation {
    pub fn prod(self, r: Mutation) -> Mutation {
        Mutation(self.0, self.1 * r.1, self.2 *r.2, self.3 * r.3, self.4 * r.4)
    }

    pub fn likelihood(self, branch_length: f64, prob_matrix: &na::Matrix4<f64>) -> Mutation {
        
        let x = prob_matrix * na::Vector4::new(self.1, self.2, self.3, self.4);

        Mutation(self.0, x[0], x[1], x[2], x[3])
    }
}

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
pub fn combine_lists(seq1: Option<&Vec<Mutation>>, 
    seq2: Option<&Vec<Mutation>>, 
    branchlengths: (f64, f64), 
    rate_matrix: &na::Matrix4<f64>) -> Vec<Mutation> {

    let mut out: Vec<Mutation> = Vec::new();

    // Probability matrices
    let p1 = na::Matrix::exp(&(rate_matrix * branchlengths.0));
    let p2 = na::Matrix::exp(&(rate_matrix * branchlengths.1));

    let mut s1 = seq1.unwrap().iter();
    let mut s2 = seq2.unwrap().iter();

    let mut mut1 = s1.next();
    let mut mut2 = s2.next();

    while mut1.is_some() | mut2.is_some() {
        if mut1.is_none() {
            // First iterator empty, push second
            out.push(mut2.unwrap().likelihood(branchlengths.1, &p2));
            mut2 = s2.next();
        } else if mut2.is_none() {
            // Second iterator empty, push first
            out.push(mut1.unwrap().likelihood(branchlengths.0, &p1));
            mut1 = s1.next();
        } else {
            // Neither iterator empty, compare indices of mutations and push highest
            // or combine likelihood if mutations at same location
            match mut1.unwrap().0.cmp(&mut2.unwrap().0) {
                Ordering::Equal => {
                    out.push(mut1.unwrap()
                                .likelihood(branchlengths.0, &p1)
                                .prod(mut2.unwrap().likelihood(branchlengths.1, &p2)));
                            mut1 = s1.next();
                            mut2 = s2.next();
                },
                Ordering::Greater => {
                    out.push(mut1.unwrap().likelihood(branchlengths.0, &p1));
                    mut1 = s1.next();
                },
                Ordering::Less => {
                    out.push(mut2.unwrap().likelihood(branchlengths.1, &p2));
                    mut2 = s2.next();
                },
            }
        }

        
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
