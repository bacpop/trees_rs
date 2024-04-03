#ifndef PHYLO2VEC_HPP
#define PHYLO2VEC_HPP

#include <array>
#include <map>
#include <string>
#include <utility>
#include <vector>

/**
 * @brief Result of a Newick2V operation
 * v: the output Phylo2Vec vector
 * num_leaves: number of leaves
 * mapping: the integer (display as a string for now) to taxon mapping
 * // TODO mapping should be std::map<int, std::string>
 */
struct Newick2VResult {
    std::vector<int> v;
    int num_leaves;
    std::map<std::string, std::string> mapping;
};

/**
 * @brief Sample a random Phylo2Vec v for n_leaves = k + 1
 *
 * @param k number of leaves - 1
 */
std::vector<int> sample(const int &k);

/**
 * @brief check that Phylo2Vec v is correct
 * i.e. that each 0 <= v[i] <= 2i
 */

void check_v(const std::vector<int> &v);

/**
 * @brief flip the rows of a 2D matrix
 *
 * @param vec 2D matrix
 */
template <typename T>
void flipRows(T &vec);

/**
 * @brief flip the columns of a 2D matrix
 *
 * @param vec 2D matrix
 */
template <typename T>
void flipColumns(T &vec);

/**
 * @brief flip rows and/or columns of a 2D matrix
 * Equivalent of np.flip
 * @tparam T
 * @param vec 2D matrix
 * @param axis Axis or axes along which to flip over
 * By default: flip all axes
 */
template <typename T>
void flip(T &vec, int axis = -1);

/**
 * @brief initialise the view matrix for getAncestry
 * In Python: np.tril([np.arange(k+1)]*(k+1))
 *
 * @param k number of leaves - 1
 * @return std::vector<std::vector<int>> matrix of size k, k+1
 * The last row is kept apart to clarify the implementation of getAncestry
 * Output:
 * 0 0 0 0 0 ...
 * 0 1 0 0 0 ...
 * 0 1 2 0 0 ...
 * 0 1 2 3 0 ...
 * 0 1 2 3 4 ...
 */
std::vector<std::vector<int>> initViewMatrix(const int &k);

/**
 * @brief Get ancestry for each node given a v-representation.
 *
 * @param v Phylo2Vec vector
 * @return std::vector<std::array<int, 3>>
 * 1st column: parent
 * 2nd and 3rd columns: children
 */
std::vector<std::array<int, 3>> getAncestry(const std::vector<int> &v);

/**
 * @brief Build a Newick string from an "ancestry" array to describe a tree
 * M is processed such that we iteratively write a Newick string
 * to describe the tree.
 * @param M cf. getAncestry
 * @return std::string Newick-format representation of a tree
 */
std::string buildNewick(std::vector<std::array<int, 3>> M);

/**
 * @brief Wrapper of getAncestry and toNewick
 *
 * @param v Phylo2Vec vector
 */
std::string toNewick(const std::vector<int> &v);

/**
 * @brief remove parent nodes from a Newick string
 * Example: "(((2,1)4,0)5,3)6;" --> "(((2,1),0),3);"
 */
void removeParentAnnotations(std::string &newick);

/**
 * @brief remove branch lengths annotations from a Newick string
 * Example: "(((2:0.02,1:0.01),0:0.041),3:1.42);" --> "(((2,1),0),3);"
 * @param newick Newick representation of a tree
 */
void removeBranchLengthAnnotations(std::string &newick);

std::map<std::string, std::string> integerizeChildNodes(std::string &newick);

/**
 * @brief Calculate the number of leaves in a tree from its Newick
 *
 * @param newick Newick representation of a tree
 */
int getNumLeavesFromNewick(const std::string &newick);

/**
 * @brief Split the string around first occurrence of sep
 * This function is part of the Pystring project.
 * Copyright Contributors to the Pystring project.
 * SPDX-License-Identifier: BSD-3-Clause
 * For the full copyright and license information, please view:
 * https://github.com/imageworks/pystring/blob/master/LICENSE
 * @param result a 3-tuple containing the part before the separator, the separator itself, and the
 * part after the separator
 * If the separator is not found, return a 3-tuple containing the string itself, followed by two
 * empty strings
 */
void partition(const std::string &str, const std::string &sep, std::vector<std::string> &result);
inline std::vector<std::string> partition(const std::string &str, const std::string &sep) {
    std::vector<std::string> result;
    partition(str, sep, result);
    return result;
}

/**
 * @brief Split the string at the last occurrence of sep
 * This function is part of the Pystring project.
 * Copyright Contributors to the Pystring project.
 * SPDX-License-Identifier: BSD-3-Clause
 * For the full copyright and license information, please view:
 * https://github.com/imageworks/pystring/blob/master/LICENSE
 * @param result a 3-tuple containing the part before the separator, the separator itself, and the
 * part after the separator If the separator is not found, return a 3-tuple containing two empty
 * strings, followed by the string itself
 */
void rpartition(const std::string &str, const std::string &sep, std::vector<std::string> &result);
inline std::vector<std::string> rpartition(const std::string &str, const std::string &sep) {
    std::vector<std::string> result;
    rpartition(str, sep, result);
    return result;
}

/**
 * @brief Find the most inner left leaf
 *
 * @param newick Newick representation of a tree
 * @param labels @todo
 * @param processed Whether each leaf node has been processed or not
 * @param num_leaves Number of leaves
 * @return std::pair<int, int> the left leaf + at which iteration it was found
 */
std::pair<int, int> findLeftLeaf(std::string newick, const std::vector<int> &labels,
                                 const std::vector<bool> &processed, int num_leaves);

/**
 * @brief TODO
 *
 * @param vmin
 * @param right_leaf
 * @param num_leaves
 * @param processed
 */
void updateVmin(std::vector<int> &vmin, int right_leaf, int num_leaves,
                const std::vector<bool> &processed);

/**
 * @brief Update the newick string by fusing the left leaf and the right leaf
 *
 * @param newick  Newick representation of a tree
 * @param left_leaf_ind @todo
 * @param left_leaf @todo
 * @param right_leaf @todo
 * @param labels @todo
 */
void updateNewick(std::string &newick, int left_leaf_ind, int left_leaf, int right_leaf,
                  const std::vector<int> &labels);

/**
 * @brief remove annotations related to parent nodes and branch lengths of a Newick string
 *
 * @param newick Newick representation of a tree
 */
void processNewick(std::string &newick);

/**
 * @brief Convert a newick-format tree to its v representation
 *
 * @param newick Newick representation of a tree
 * @param num_leaves Number of leaves (saves some computation time if fed in advance)
 * @return std::vector<int> Phylo2Vec representation of newick
 */
std::vector<int> toVector(std::string newick, int num_leaves);

/**
 * @brief Wrapper of processNewick + getNumLeavesFromNewick (if num_leaves == -1) + toVector
 *
 * @param newick Newick representation of a tree
 * @param num_leaves Number of leaves
 * Saves some computation time if fed in advance
 * @return Newick2VResult: v and num_leaves
 */
Newick2VResult newick2v(std::string &newick, int num_leaves = -1);

/**
 * @brief Wrapper of processNewick + getNumLeavesFromNewick (if num_leaves == -1) +
 * integerizeChildNodes + toVector This is the newick2v that is used when the child nodes are not
 * integers but "real" taxa (or any string)
 * @param newick Newick representation of a tree
 * @param num_leaves Number of leaves (saves some computation time if fed in advance)
 * @return Newick2VResult: v, num_leaves, and mapping
 */
Newick2VResult newick2vWithMapping(std::string &newick, int num_leaves);

#endif  // PHYLO2VEC_HPP