

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

// pub fn theta_change(pivec: &Vec<f64>, delta: &Vec<f64>, plus: bool) -> Vec<usize> {

    //     let zip = pivec.iter().zip(delta.iter());
        
    //     match plus {
    //         true => {
    //             zip
    //             .map(|(x, y)| (x + (y / 2.0)).round() as usize)
    //             .collect()
    //         },
    //         false => {
    //             zip
    //             .map(|(x, y)| (x - (y / 2.0)).round() as usize)
    //             .collect()
    //         }
    //     }
    // }
// pub fn phi(v: &[f64]) -> Vec<f64> {
//     v.iter().enumerate().map(|(i, value)| {
//         if i == 0 || value.lt(&0.0) {
//             0.0
//         } else if value.gt(&((2 * (i - 1)) as f64)) {
//             ((2 * (i - 1)) as f64) - 0.000001
//         } else {
//             *value
//         }
//     }).collect()
// }

// pub fn piv(v: &[f64]) -> Vec<f64> {
//     let mut pivec: Vec<f64> = phi(v).iter().map(|el| el.floor() + 0.5).collect();
//     pivec[0] = 0.0;
//     pivec
// }

// pub fn peturbation_vec(n: usize) -> Vec<f64> {
//     let rng = rand::thread_rng();
//     let distr = rand::distributions::Bernoulli::new(0.5).unwrap();
//     let mut delta: Vec<f64> = rng.sample_iter(distr).take(n).map(|el| match el {
//         true => 1.0,
//         false => -1.0,
//     }).collect();
//     delta[0] = 0.0;
//     delta
// }

    // pub fn optimise(&mut self, q: &na::Matrix4<f64>, iterations: usize) {

    //     // Update likelihood if not done already
    //     // if self.get_tree_likelihood().eq(&0.0) {
    //     //     self.update_likelihood(&q);
    //     // }

    //     // Convert tree vector to Vec<f64>
    //     let mut theta: Vec<f64> = self.tree_vec.iter().map(|x| *x as f64).collect();
    //     println!("Current tree vector is: {:?}", self.tree_vec);
    //     println!("Current likelihood is: {}", self.get_tree_likelihood());
    //     let n: usize = theta.len();

    //     // Tuning parameters for optimisation, will
    //     // eventually have defaults or be passed in
    //     let a: f64 = 1.5;
    //     let cap_a: f64 = 1000.0;
    //     let alpha: f64 = 0.51;

    //     // Pre-allocate vectors
    //     let mut delta: Vec<f64> = Vec::with_capacity(n);
    //     let mut pivec: Vec<f64> = Vec::with_capacity(n);
    //     let mut thetaplus: Vec<usize> = Vec::with_capacity(n);
    //     let mut thetaminus: Vec<usize> = Vec::with_capacity(n);
    //     let mut ghat: Vec<f64> = Vec::with_capacity(n);
    //     let mut new_tree_vec: Vec<usize> = Vec::with_capacity(n);

    //     // Optimisation loop
    //     for k in 0..=iterations {
    //         println!("Optimisation step {} out of {}", k, iterations);
    //         println!("Negative tree log likelihood: {}", -self.get_tree_likelihood());
    //         // Generate peturbation vector
    //         delta = peturbation_vec(n);
    //         // println!("Peturbation vector: {:?}", delta);

    //         // Generate pi vector
    //         pivec = piv(&theta);
    //         // println!("Pi vector: {:?}", pivec);

    //         // Calculate theta+ and theta-,
    //         // New tree vectors based on peturbation
    //         thetaplus = theta_change(&pivec, &delta, true);
    //         thetaminus = theta_change(&pivec, &delta, false);
    //         // println!("theta+: {:?}", thetaplus);
    //         // println!("theta-: {:?}", thetaminus);

    //         // Update tree and calculate likelihoods
    //         self.update_quad(thetaplus);
    //         self.update_likelihood(&q);
    //         let lplus: f64 = -self.get_tree_likelihood();

    //         self.update_quad(thetaminus);
    //         self.update_likelihood(&q);
    //         let lminus: f64 = -self.get_tree_likelihood();

    //         // Update theta based on likelihoods of theta+/-
    //         let ldiff = lplus - lminus;

    //         println!("ll+ is {} and ll- is {}, ldiff is {}", lplus, lminus, ldiff);

    //         ghat = delta.iter().map(|delta| ldiff * (1.0 / delta)).collect();
    //         ghat[0] = 0.0;

    //         // println!("ghat is {:?}", ghat);

    //         let ak: f64 = a / (1.0 + cap_a + k as f64).powf(alpha);

    //         println!("ak is {}", ak);

    //         // Set new theta
    //         theta = theta.iter().zip(ghat.iter())
    //         .map(|(theta, g)| *theta - (ak * g)).collect();

    //         new_tree_vec = phi(&theta).iter().map(|x| x.round() as usize).collect();
    //         self.update_quad(new_tree_vec);
    //         self.update_likelihood(&q)
    //         // println!("New theta is: {:?}", theta);
    //     }

    //     // Update final tree after finishing optimisation
    //     // let new_tree_vec: Vec<usize> = phi(&theta).iter().map(|x| x.round() as usize).collect();
    //     // println!("New tree vector is: {:?}", new_tree_vec);
    //     // self.update_quad(new_tree_vec);
    //     // self.update_likelihood(&q);
    //     println!("New tree likelihood is {}", self.get_tree_likelihood());
    // }