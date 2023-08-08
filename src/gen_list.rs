use needletail::*;
use std::cmp::Ordering;
use crate::Tree;

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
        'B' => Mutation(i, 0.0, 1.0 / 3.0, 1.0 / 3.0, 1.0 / 3.0),
        'D' => Mutation(i, 1.0 / 3.0, 0.0, 1.0 / 3.0, 1.0 / 3.0),
        'V' => Mutation(i, 1.0 / 3.0, 1.0 / 3.0, 1.0 / 3.0, 0.0),
        // This is currently incorrect. Not sure how to handle gaps.
        '-' => Mutation(i, 0.25, 0.25, 0.25, 0.25),
        _ => panic!("Unrecognised character: {}", e),
    }
}

// Combines two vectors of Mutations into a single vector
pub fn combine_lists(
    seq1: Option<&Vec<Mutation>>,
    seq2: Option<&Vec<Mutation>>,
    branchlengths: (f64, f64),
    rate_matrix: &na::Matrix4<f64>,
) -> Vec<Mutation> {
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
            out.push(mut2.unwrap().child_likelihood(&p2));
            mut2 = s2.next();
        } else if mut2.is_none() {
            // Second iterator empty, push first
            out.push(mut1.unwrap().child_likelihood(&p1));
            mut1 = s1.next();
        } else {
            // println!("mut1 = {:?} mut2 = {:?}", mut1.unwrap(), mut2.unwrap());
            // Neither iterator empty, compare indices of mutations and push highest
            // or combine likelihood if mutations at same location
            match mut1.unwrap().0.cmp(&mut2.unwrap().0) {
                Ordering::Equal => {
                    // println!("mut1 == mut2 so pushing {:?}", mut1.unwrap());
                    out.push(
                        mut1.unwrap()
                            .child_likelihood(&p1)
                            .prod(mut2.unwrap().child_likelihood(&p2)),
                    );
                    mut1 = s1.next();
                    mut2 = s2.next();
                }
                Ordering::Greater => {
                    // println!("mut1 > mut2 so pushing {:?}", mut2.unwrap());
                    out.push(mut2.unwrap().child_likelihood(&p2));
                    mut2 = s2.next();
                }
                Ordering::Less => {
                    // println!("mut2 > mut1 so pushing {:?}", mut1.unwrap());
                    out.push(mut1.unwrap().child_likelihood(&p1));
                    mut1 = s1.next();
                }
            }
        }
    }
    out
}

impl Tree {
    pub fn add_genetic_data(&mut self, filename: &str) {
        let mut reader = parse_fastx_file(filename).expect("Error parsing file");

        // For now take first sequence as reference sequence
        let record = reader.next().unwrap().unwrap();
        let seq_vec: Vec<char> = record.seq().iter().map(|l| *l as char).collect();

        while let Some(rec) = reader.next() {
            let newrec: Vec<char> = rec.unwrap().seq().iter().map(|l| *l as char).collect();
            self.mutation_lists.push(create_list(&seq_vec, &newrec));
        }

        let leafn = self.mutation_lists.len() - 1;
        for _ in 0..leafn {
            self.mutation_lists.push(Vec::new());
        }
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