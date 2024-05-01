fn main() {
    cxx_build::bridge("src/build_tree.rs")
        .file("src/phylo2vec.cpp")
        .std("c++17")
        .compile("phylo2vec");

        println!("cargo:rerun-if-changed=src/build_tree.rs");
        println!("cargo:rerun-if-changed=src/phylo2vec.cpp");
        println!("cargo:rerun-if-changed=src/phylo2vec.hpp");
}
