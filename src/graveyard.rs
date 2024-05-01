

// Return a mutable reference to the parent of a given node
// pub fn mut_parent(&mut self, index: usize) -> Option<&mut Node> {
//     match self.nodes.get(index).unwrap().parent {
//         Some(i) => self.mut_node(i),
//         None => None,
//     }
// }

// pub fn path_length(&self, index1: usize, index2: usize) -> usize {
//     let mut temp: HashSet<usize> = HashSet::new();

//     let x: Vec<usize> = self.iter(self.get_node(index1))
//                             .chain(self.iter(self.get_node(index2)))
//                             .map(|n| n.index)
//                             .collect();

//     for i in &x {
//         match temp.get(i) {
//             Some(_) => temp.remove(i),
//             None => temp.insert(*i),
//         };
//     };

//     temp.iter().len()
// }

// #[derive(Debug)]
// pub struct RootIter<'a> {
//     current_node: Option<&'a Node>,
//     next_node: Option<&'a Node>,
//     tree: &'a Tree,
//     end_flag: bool,
// }

// // Traverses from a specified node up to the root of the tree
// impl<'a> Iterator for RootIter<'a> {
//     type Item = &'a Node;

//     fn next(&mut self) -> Option<Self::Item> {
//         let output: Option<Self::Item>;

//         if self.end_flag {
//             return None;
//         };

//         match self.current_node.unwrap().parent {
//             None => {
//                 output = self.tree.get_root();
//                 self.end_flag = true;
//             }
//             Some(i) => {
//                 output = self.current_node;
//                 self.next_node = self.tree.get_node(i);
//             }
//         };

//         self.current_node = self.next_node;

//         output
//     }
// }

// impl<'a> Tree {
//     // Iterates from a specified node upwards to the root of the tree
// pub fn iter(&'a self, node: Option<&'a Node>) -> RootIter {
//     RootIter {
//         current_node: node,
//         next_node: node,
//         tree: self,
//         end_flag: false,
//     }
// }

// // Rootwards iterator that ignores leaves
// pub fn iter_notips(&'a self, node: Option<&'a Node>) -> impl Iterator<Item = &'a Node> {
//     self.iter(node).filter(|node| !node.tip)
// }
// }

// #[derive(Debug)]
// pub struct Preorder<'a> {
//     current_node: Option<&'a Node>,
//     next_node: Option<&'a Node>,
//     return_nodes: Vec<Option<&'a Node>>,
//     tree: &'a Tree,
//     // pub newick: String,
// }

// // Traverses tree in preorder starting from specified node
// impl<'a> Iterator for Preorder<'a> {
//     type Item = &'a Node;

//     fn next(&mut self) -> Option<Self::Item> {
//         let output: Option<&'a Node> = self.current_node;

//         if self.current_node.is_none() {
//             return output;
//         }
//         match self.current_node.unwrap().children {
//             (Some(a), None) => {
//                 self.next_node = self.tree.get_node(a);
//             }
//             (Some(a), Some(b)) => {
//                 self.next_node = self.tree.get_node(a);
//                 self.return_nodes.push(self.tree.get_node(b));
//             }
//             (None, None) => {
//                 self.next_node = match self.return_nodes.pop() {
//                     None => None,
//                     Some(node) => node,
//                 };
//             }
//             _ => {
//                 panic!("Iterator has found a node with only a right child")
//             }
//         };

//         self.current_node = self.next_node;

//         output
//     }
// }



// Traverses up to the root, updating likelihood as it goes
// pub fn update_likelihood_rootward(&'a self,
//     node: Option<&'a Node>,
//     genetic_data: &mut GeneticData,
//     rate_matrix: &na::Matrix4<f64>) {

//     for elem in self.iter_notips(node) {
//         let branchlengths = (self.get_branchlength(elem.children.0.unwrap()),
//         self.get_branchlength(elem.children.1.unwrap()));

//     let seq1 = genetic_data.likelihood_lists.get(elem.children.0.unwrap());
//     let seq2 = genetic_data.likelihood_lists.get(elem.children.1.unwrap());

//     genetic_data.likelihood_lists[elem.index] = combine_lists(seq1, seq2, branchlengths, rate_matrix);
//     }

// }

// Returns vector of nodes in tree that are tips
// pub fn get_tips(&self) -> Vec<&Node> {
//     self.nodes.iter().filter(|n| n.tip).collect()
// }


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