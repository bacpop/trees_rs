use crate::Tree;
use needletail::*;
use rand::Rng;
use std::cmp::Ordering;

#[derive(Debug, Copy, Clone)]
pub struct Mutation(pub f64, pub f64, pub f64, pub f64);

const NEGINF: f64 = -f64::INFINITY;
        // (A, C, G, T)
const AMUT: Mutation = Mutation(0.0, NEGINF, NEGINF, NEGINF);
const CMUT: Mutation = Mutation(NEGINF, 0.0, NEGINF, NEGINF);
const GMUT: Mutation = Mutation(NEGINF, NEGINF, 0.0, NEGINF);
const TMUT: Mutation = Mutation(NEGINF, NEGINF, NEGINF, 0.0);
const YMUT: Mutation = Mutation(NEGINF, 0.0, NEGINF, 0.0);
const WMUT: Mutation = Mutation(0.0, NEGINF, NEGINF, 0.0);
const RMUT: Mutation = Mutation(0.0, NEGINF, 0.0, NEGINF);
const KMUT: Mutation = Mutation(NEGINF, NEGINF, 0.0, 0.0);
const SMUT: Mutation = Mutation(NEGINF, 0.0, 0.0, NEGINF);
const MMUT: Mutation = Mutation(0.0, 0.0, NEGINF, NEGINF);
const BMUT: Mutation = Mutation(NEGINF, 0.0, 0.0, 0.0);
const DMUT: Mutation = Mutation(0.0, NEGINF, 0.0, 0.0);
const VMUT: Mutation = Mutation(0.0, 0.0, 0.0, NEGINF);

pub fn char_to_mutation(e: &char) -> Mutation {
    match e {
        'A' => AMUT,
        'C' => CMUT,
        'G' => GMUT,
        'T' => TMUT,
        'Y' => YMUT,
        'W' => WMUT,
        'R' => RMUT,
        'K' => KMUT,
        'S' => SMUT,
        'M' => MMUT,
        'B' => BMUT,
        'D' => DMUT,
        'V' => VMUT,
        '-' => {
            // This way of coding gives same answer as other tree programs
            Mutation(0.0, 0.0, 0.0, 0.0)},
        _ => panic!("Unrecognised character: {}", e),
    }
}

// Combines two vectors of Mutations into a single vector
// pub fn combine_lists(
//     seq1: Option<&Vec<Mutation>>,
//     seq2: Option<&Vec<Mutation>>,
//     branchlengths: (f64, f64),
//     rate_matrix: &na::Matrix4<f64>,
// ) -> Vec<Mutation> {
//     let mut out: Vec<Mutation> = Vec::new();

//     // Probability matrices
//     let p1 = na::Matrix::exp(&(rate_matrix * branchlengths.0));
//     let p2 = na::Matrix::exp(&(rate_matrix * branchlengths.1));

//     let mut s1 = seq1.unwrap().iter();
//     let mut s2 = seq2.unwrap().iter();

//     let mut mut1 = s1.next();
//     let mut mut2 = s2.next();

//     while mut1.is_some() | mut2.is_some() {
//         if mut1.is_none() {
//             // First iterator empty, push second
//             out.push(mut2.unwrap().child_log_likelihood(&p2));
//             mut2 = s2.next();
//         } else if mut2.is_none() {
//             // Second iterator empty, push first
//             out.push(mut1.unwrap().child_log_likelihood(&p1));
//             mut1 = s1.next();
//         } else {
//             // println!("mut1 = {:?} mut2 = {:?}", mut1.unwrap(), mut2.unwrap());
//             // Neither iterator empty, compare indices of mutations and push highest
//             // or combine likelihood if mutations at same location
//             match mut1.unwrap().0.cmp(&mut2.unwrap().0) {
//                 Ordering::Equal => {
//                     // println!("mut1 == mut2 so pushing {:?}", mut1.unwrap());
//                     out.push(
//                         mut1.unwrap()
//                             .child_log_likelihood(&p1)
//                             .sum(mut2.unwrap().child_log_likelihood(&p2)),
//                     );
//                     mut1 = s1.next();
//                     mut2 = s2.next();
//                 }
//                 Ordering::Greater => {
//                     // println!("mut1 > mut2 so pushing {:?}", mut2.unwrap());
//                     out.push(mut2.unwrap().child_log_likelihood(&p2));
//                     mut2 = s2.next();
//                 }
//                 Ordering::Less => {
//                     // println!("mut2 > mut1 so pushing {:?}", mut1.unwrap());
//                     out.push(mut1.unwrap().child_log_likelihood(&p1));
//                     mut1 = s1.next();
//                 }
//             }
//         }
//     }
//     out
// }

pub fn combine_lists(
    seq1: &[Mutation],
    seq2: &[Mutation],
    branchlengths: (f64, f64),
    rate_matrix: &na::Matrix4<f64>,
) -> Vec<Mutation> {

    // Probability matrices
    let p1 = na::Matrix::exp(&(rate_matrix * branchlengths.0));
    let p2 = na::Matrix::exp(&(rate_matrix * branchlengths.1));

    let out: Vec<Mutation> = seq1.iter()
    .zip(seq2.iter())
    .map(|(b1, b2)| b1.child_log_likelihood(&p1)
                                            .sum(b2.child_log_likelihood(&p2)))
    .collect();

    out
}

impl Tree {
    pub fn add_genetic_data(&mut self, filename: &str) {
        let mut reader = parse_fastx_file(filename).expect("Error parsing file");

        // Add genetic data 
        while let Some(rec) = reader.next() {
            let newrec: Vec<char> = rec.unwrap().seq().iter().map(|l| *l as char).collect();
            self.mutation_lists.push(create_list(&newrec));
        }

        // Add empty lists for internal nodes
        let leafn = self.mutation_lists.len() - 1;
        for _ in 0..leafn {
            self.mutation_lists.push(Vec::new());
        }
    }
}

// Takes a reference sequence and another sequence in SequenceRecord<'_> format
// Returns a vector of Mutations for how the latter sequence differs from the reference
pub fn create_list(seq: &[char]) -> Vec<Mutation> {
    let mut out: Vec<Mutation> = Vec::new();

    for base in seq.iter() {
        out.push(char_to_mutation(base));
    }

    out
}

// pub fn create_dummy_genetic_data(n_leaves: usize, n_mutations: usize, sequence_length: usize) -> Vec<Vec<Mutation>> {
//     let mut output: Vec<Vec<Mutation>> = Vec::new();
//     let mut rng = rand::thread_rng();

//     for i in 0..n_leaves {
//         let mut temp: Vec<Mutation> = Vec::new();
//         for j in 0..n_mutations {
//             let mut mutation = Mutation(rng.gen_range(1..sequence_length), 0.0, 0.0, 0.0, 0.0);
//             match rng.gen_range(1..=4) {
//                 1 => {mutation.1 = 1.0},
//                 2 => {mutation.2 = 1.0},
//                 3 => {mutation.3 = 1.0},
//                 4 => {mutation.4 = 1.0},
//                 _ => {},
//             }
//             temp.push(mutation);
//         }
//         temp.sort_by(|a, b| a.0.cmp(&b.0));
//         temp.dedup_by(|a, b| a.0.eq(&b.0));
//         output.push(temp);
//     }

//     for _ in 0..(n_leaves + 1) {
//         output.push(Vec::new());
//     }

//     output
// }