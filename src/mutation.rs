
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
    let mut out: Vec<Mutation> = Vec::new();

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
            Mutation(0.0, 0.0, 0.0, 0.0)},
        _ => panic!("Unrecognised character: {}", e),
    }
}

// Converts a vector of correct length to Mutation
pub fn to_mutation(x1: Vec<f64>) -> Mutation {
    if x1.len().ne(&4) {
        panic!("Length of vector too long to cast to Mutation");
    } else {
        Mutation(*x1.first().unwrap(), *x1.get(1).unwrap(), *x1.get(2).unwrap(), *x1.get(3).unwrap())
    }
}

impl Mutation {

    // Converts a Mutation to a vector
    pub fn to_vector(self) -> Vec<f64> {
        vec![self.0, self.1, self.2, self.3]
    }

    // Adds two Mutations together
    pub fn sum(self, r: Mutation) -> Mutation {
        Mutation(
            self.0 + r.0,
            self.1 + r.1,
            self.2 + r.2,
            self.3 + r.3,
        )
    }

}


