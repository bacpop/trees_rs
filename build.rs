fn main() {
    cxx_build::bridge("src/newick_to_vec.rs")
        .file("src/phylo2vec.cpp")
        .std("c++17")
        .compile("phylo2vec");

        println!("cargo:rerun-if-changed=src/newick_to_vec.rs");
        println!("cargo:rerun-if-changed=src/phylo2vec.cpp");
        println!("cargo:rerun-if-changed=src/phylo2vec.hpp");
}
