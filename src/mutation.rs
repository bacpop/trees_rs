use std::hash;

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

// Takes a reference sequence and another sequence in SequenceRecord<'_> format
// Returns a vector of Mutations for how the latter sequence differs from the reference
pub fn create_list(seq: &[char]) -> Vec<Mutation> {
    let mut out: Vec<Mutation> = Vec::with_capacity(seq.len());

    for base in seq.iter() {
        out.push(char_to_mutation(base));
    }

    out
}

// Turns characters into Mutations
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
            Mutation::default()
        }
        _ => panic!("Unrecognised character: {}", e),
    }
}

// Converts a vector of correct length to Mutation
// pub fn to_mutation(x1: Vec<f64>) -> Mutation {
//     if x1.len().ne(&4) {
//         panic!("Length of vector too long to cast to Mutation");
//     } else {
//         Mutation(x1[0], x1[1], x1[2], x1[3])
//     }
// }

impl Mutation {
    // Converts a Mutation to a vector
    // pub fn to_vector(self) -> Vec<f64> {
    //     vec![self.0, self.1, self.2, self.3]
    // }

    // Adds two Mutations together
    pub fn add(self, r: Mutation) -> Mutation {
        Mutation(self.0 + r.0, self.1 + r.1, self.2 + r.2, self.3 + r.3)
    }

    pub fn get(self, i: usize) -> Option<f64> {
        match i {
            0 => Some(self.0),
            1 => Some(self.1),
            2 => Some(self.2),
            3 => Some(self.3),
            _ => None,
        }
    }

    pub fn get_mut(&mut self, i: usize) -> Option<&mut f64> {
        match i {
            0 => Some(&mut self.0),
            1 => Some(&mut self.1),
            2 => Some(&mut self.2),
            3 => Some(&mut self.3),
            _ => None,
        }
    }

    pub fn iter(self) -> MutationIter {
        MutationIter { ind: 0, muta: self }
    }
}

#[derive(Debug)]
pub struct MutationIter {
    ind: usize,
    muta: Mutation,
}

impl Iterator for MutationIter {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        let out: Option<f64> = self.muta.get(self.ind);
        self.ind += 1;
        out
    }
}

impl Default for Mutation {
    fn default() -> Self {
        Mutation(0.0, 0.0, 0.0, 0.0)
    }
}

impl PartialEq for Mutation {
    fn eq(&self, other: &Mutation) -> bool {
        self.iter().zip(other.iter()).all(|(a, b)| a.eq(&b))
    }
}

impl Eq for Mutation {}

#[derive(Debug, Copy, Clone)]
pub struct MutationKey(pub Mutation);

impl MutationKey {
    fn key(&self) -> u64 {
        let x: f64 = (self.0 .0 + 2.0 * self.0 .1 + 3.0 * self.0 .2 + 4.0 * self.0 .3);
        x.to_bits()
    }
}

impl hash::Hash for MutationKey {
    fn hash<H>(&self, state: &mut H)
    where
        H: hash::Hasher,
    {
        self.key().hash(state)
    }
}

impl PartialEq for MutationKey {
    fn eq(&self, other: &MutationKey) -> bool {
        self.key() == other.key()
    }
}

impl Eq for MutationKey {}
