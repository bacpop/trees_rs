fn main() {
    cc::Build::new()
        .cpp(true)
        .flag("-std=c++17")
        .file("src/phylo2vec.cpp")
        .compile("phylo2vec");
}