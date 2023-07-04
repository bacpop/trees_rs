
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
    pub element: (MutationType, i64, Option<i64>),
}

impl Sample {
    pub fn add(&mut self, x: Entry) {
        self.list.push(x);
    }
}
    

impl Entry {
    pub fn new(mutation: char, i: i64, j: Option<i64>) -> Entry {
        let converted_mut: MutationType = mutation_char_to_enum(mutation);

        Entry {
            element: (converted_mut, i , j),
        }
    }

    pub fn start(&self) -> i64 {
        self.element.1
    }

    pub fn end(&self) -> Option<i64> {
        self.element.2
    }

}
