#include "phylo2vec.hpp"

// #include <omp.h>

#include <algorithm>
#include <iostream>
#include <random>
#include <regex>
#include <sstream>
#include <stdexcept>

std::vector<int> sample(const int &k) {
    std::vector<int> v;
    std::random_device rd;
    std::mt19937 gen(rd());

    for (std::size_t i = 0; i < k; ++i) {
        std::uniform_int_distribution<> distrib(0, 2 * i);
        v.push_back(distrib(gen));
    }

    return v;
}

void check_v(const std::vector<int> &v) {
    // check that v is valid: 0 <= v[i] <= 2i

    const std::size_t k = v.size();
    for (std::size_t i = 0; i < k; ++i) {
        if (v[i] > 2 * i) {
            std::ostringstream oss;
            oss << "Invalid value at index " << i << ": v[i] should be less than 2i, found " << v[i]
                << ".";
            throw std::out_of_range(oss.str());
        }
    }
}

template <typename T>
void flipRows(T &vec) {
    std::reverse(vec.begin(), vec.end());
}

template <typename T>
void flipColumns(T &vec) {
    for (auto &arr : vec) {
        std::reverse(arr.begin(), arr.end());
    }
}

template <typename T>
void flip(T &vec, int axis) {
    if (axis == -1) {
        flipRows(vec);
        flipColumns(vec);
    } else if (axis == 0) {
        flipRows(vec);
    } else if (axis == 1) {
        flipColumns(vec);
    } else {
        std::ostringstream oss;
        oss << "axis can be -1, 0 or 1 for now. Found axis = " << axis;
        throw std::out_of_range(oss.str());
    }
}

std::vector<std::vector<int>> initViewMatrix(const int &k) {
    std::vector<std::vector<int>> labels(k, std::vector<int>(k + 1, 0));
    for (std::size_t i = 0; i < k; ++i) {
        for (std::size_t j = 0; j <= i; ++j) {
            labels[i][j] = j;
        }
    }

    return labels;
}

std::vector<std::array<int, 3>> getAncestry(const std::vector<int> &v) {
    const std::size_t k = v.size();

    // init "view" matrix
    std::vector<std::vector<int>> labels = initViewMatrix(k);

    std::vector<int> labels_last_row(k + 1);
    std::iota(labels_last_row.begin(), labels_last_row.end(), 0);

    std::vector<bool> not_processed(k, true);
    std::vector<std::array<int, 3>> M(k, {{0, 0, 0}});

    for (std::size_t step = 0; step < k; ++step) {
        std::vector<int> row_maxes(k);

        int n = -1;

        for (int row = 0; row < k; ++row) {
            int row_max = 0;
            for (std::size_t col = 0; col <= row; ++col) {
                row_max = std::max(row_max, labels[row][col]);
            }
            row_maxes[row] = row_max;

            if (v[row] <= row_max && not_processed[row]) {
                n = row;
            }
        }

        if (n == -1) {
            throw std::out_of_range("n should be a positive index.");
        }

        // find m
        int m = -1;
        for (int i = 0; i < k; ++i) {
            if (labels[n][i] == v[n]) {
                m = i;
                break;
            }
        }

        if (m == -1) {
            throw std::out_of_range("m should be a positive index.");
        }

        // Update the ancestry matrix
        M[step][0] = labels_last_row[m];
        M[step][1] = labels_last_row[n + 1];

        // Update the view matrix
        for (std::size_t row = n; row < k; ++row) {
            labels[row][m] = row_maxes[row] + 1;
        }

        labels_last_row[m] = *std::max_element(labels_last_row.begin(), labels_last_row.end()) + 1;

        M[step][2] = labels_last_row[m];
        not_processed[n] = false;
    }

    // flip rows and columns so that we get:
    // 1st column: parent
    // 2nd and 3rd columns: children
    flip(M);
    return M;
}

std::string buildNewick(std::vector<std::array<int, 3>> M) {
    std::vector<std::string> parent_nodes;

    std::vector<std::string> sub_newicks;

    int k = M.size();

    for (int i = k - 1; i >= 0; --i) {
        std::string parent = std::to_string(M[i][0]);
        std::string child1 = std::to_string(M[i][1]);
        std::string child2 = std::to_string(M[i][2]);

        auto it1 = std::find(parent_nodes.begin(), parent_nodes.end(), child1);
        auto it2 = std::find(parent_nodes.begin(), parent_nodes.end(), child2);

        int idx1 = it1 != parent_nodes.end() ? std::distance(parent_nodes.begin(), it1) : -1;
        int idx2 = it2 != parent_nodes.end() ? std::distance(parent_nodes.begin(), it2) : -1;

        if (idx1 >= 0 && idx2 >= 0) {
            // Case 1: Both children are parent nodes, so we have sub-newicks for them

            // Merge the sub-newicks and add the parent node
            sub_newicks[idx1] = "(" + sub_newicks[idx1] + "," + sub_newicks[idx2] + ")" + parent;

            // Update the parent node for the 1st children
            parent_nodes[idx1] = parent;

            // Discard info on 2nd children as merged with the 1st children
            sub_newicks.erase(sub_newicks.begin() + idx2);

            parent_nodes.erase(parent_nodes.begin() + idx2);
        } else if (idx1 >= 0) {
            // Case 2: only the first child is a parent node

            // Update its sub-Newick:
            // (sub_child1.1, sub_child1.2)child_1 -->
            // ((sub_child1.1,sub_child1.2)child_1,child_2)parent
            sub_newicks[idx1].insert(sub_newicks[idx1].find(child1) + child1.size(),
                                     "," + child2 + ")" + parent);

            sub_newicks[idx1].insert(0, "(");

            // Update the parent node (child 1 is now just an internal node)
            parent_nodes[idx1] = parent;
        } else if (idx2 >= 0) {
            // Case 3: only the second child is a parent node (similar to Case 2)

            // (sub_child2.1, sub_child2.2)child_2 -->
            // ((sub_child2.1,sub_child2.2)child_2, child_1)parent
            sub_newicks[idx2].insert(sub_newicks[idx2].find(child2) + child2.size(),
                                     "," + child1 + ")" + parent);

            sub_newicks[idx2].insert(0, "(");

            // Update the parent node (child 2 is now just an internal node)
            parent_nodes[idx2] = parent;
        } else {
            // Case 4: the children nodes have not been added yet

            // Add a new sub-Newick for this triplet
            sub_newicks.push_back("(" + child1 + "," + child2 + ")" + parent);

            parent_nodes.push_back(parent);
        }
    }
    // If everything went well, only one "sub-newick" should be left, with only
    // one parent: the root node

    std::string newick = sub_newicks[0] + ";";
    return newick;
}

std::string toNewick(const std::vector<int> &v) { return buildNewick(getAncestry(v)); }

void removeBranchLengthAnnotations(std::string &newick) {
    std::regex pattern(":\\d+(\\.\\d+)?");
    newick = std::regex_replace(newick, pattern, "");
}

void removeParentAnnotations(std::string &newick) {
    std::regex pattern("\\)([^,;\\(\\)]+?)([\\(,;\\)])");
    while (std::regex_search(newick, pattern)) {
        newick = std::regex_replace(newick, pattern, ")$2");
    }
}

std::map<std::string, std::string> integerizeChildNodes(std::string &newick) {
    std::map<std::string, std::string> mapping;
    int idx = 0;

    char left_start = '(', left_end = ',', right_start = ',', right_end = ')', newick_end = ';';

    for (std::size_t i = 0; i < newick.length(); ++i) {
        if (newick[i] == left_start || right_start) {
            size_t j = i + 1;
            while (j < newick.length() &&
                   (newick[j] != left_start && newick[j] != left_end && newick[j] != right_start &&
                    newick[j] != right_end && newick[j] != newick_end)) {
                ++j;
            }

            if (j != i + 1) {
                std::string node = newick.substr(i + 1, j - i - 1);

                if (mapping.find(node) == mapping.end()) {
                    mapping[std::to_string(idx)] = node;
                    newick.replace(i + 1, j - i - 1, std::to_string(idx));
                    ++idx;
                }
            }
        }
    }

    return mapping;
}

int getNumLeavesFromNewick(const std::string &newick) {
    // Adapted from https://www.geeksforgeeks.org/extract-maximum-numeric-value-given-string/
    int num = 0, min_val = 0, max_val = 0;

    for (std::size_t i = 0; i < newick.length(); i++) {
        if (newick[i] >= '0' && newick[i] <= '9') {
            // Convert and adjust if consecutive
            num = num * 10 + (newick[i] - '0');
        } else {
            // Update max
            max_val = std::max(max_val, num);
            min_val = std::min(min_val, num);

            // Reset the number
            num = 0;
        }
    }

    return std::max(max_val, num) - std::min(min_val, num) + 1;  // TODO
}

// Copyright Contributors to the Pystring project.
// SPDX-License-Identifier: BSD-3-Clause
// https://github.com/imageworks/pystring/blob/master/LICENSE
void partition(const std::string &str, const std::string &sep, std::vector<std::string> &result) {
    result.resize(3);
    size_t index = str.find(sep);
    if (index == std::string::npos) {
        result[0] = str;
        result[1] = "";
        result[2] = "";
    } else {
        result[0] = str.substr(0, index);
        result[1] = sep;
        result[2] = str.substr(index + sep.size(), str.size());
    }
}

// Copyright Contributors to the Pystring project.
// SPDX-License-Identifier: BSD-3-Clause
// https://github.com/imageworks/pystring/blob/master/LICENSE
void rpartition(const std::string &str, const std::string &sep, std::vector<std::string> &result) {
    result.resize(3);
    size_t index = str.rfind(sep);
    if (index == std::string::npos) {
        result[0] = "";
        result[1] = "";
        result[2] = str;
    } else {
        result[0] = str.substr(0, index);
        result[1] = sep;
        result[2] = str.substr(index + sep.size(), str.size());
    }
}

std::pair<int, int> findLeftLeaf(std::string newick, const std::vector<int> &labels,
                                 const std::vector<bool> &processed, int num_leaves) {
    std::string left_leaf;
    int index_i = -1;
    for (int i = 0; i < num_leaves; ++i) {
        index_i = i;
        if (!processed[num_leaves - i - 1]) {
            // Find whether the node with the current label has a sister node
            std::string label = std::to_string(labels[num_leaves - i - 1]);

            if (newick.find("(" + label + ",") != std::string::npos) {
                // Is label on the left of a newick pair?
                std::string left_sep = "(" + label + ",";
                std::string right_sep = ")";
                left_leaf = rpartition(newick, left_sep)[2];
                left_leaf = partition(left_leaf, right_sep)[0];
            } else if (newick.find("," + label + ")") != std::string::npos) {
                // Is label on the right of a newick pair?
                std::string left_sep = "(";
                std::string right_sep = "," + label + ")";

                left_leaf = partition(newick, right_sep)[0];
                left_leaf = rpartition(left_leaf, left_sep)[2];
            } else {
                // Otherwise --> it has no sister node No sister node --> we can skip it
                continue;
            }

            if (!left_leaf.empty() && std::all_of(left_leaf.begin(), left_leaf.end(), ::isdigit)) {
                // If the sister substring is an actual digit, we can stop
                break;
            }

            // Reset the left_leaf if it wasn't a digit
            left_leaf = "";
        }
    }

    return std::make_pair(std::stoi(left_leaf), index_i);
}

void updateVmin(std::vector<int> &vmin, int right_leaf, int num_leaves,
                const std::vector<bool> &processed) {
    for (int n = right_leaf + 1; n < num_leaves; ++n) {
        if (!processed[n]) {
            if (vmin[n] == 0) {
                vmin[n] = n;
            } else {
                vmin[n]++;
            }
        }
    }
}

void updateNewick(std::string &newick, int left_leaf_ind, int left_leaf, int right_leaf,
                  const std::vector<int> &labels) {
    std::string old_pattern =
        "(" + std::to_string(left_leaf) + "," + std::to_string(labels[right_leaf]) + ")";

    std::string new_pattern = std::to_string(labels[left_leaf_ind]);

    size_t pos = newick.find(old_pattern);
    if (pos != std::string::npos) {
        newick.replace(pos, old_pattern.length(), new_pattern);
    } else {
        old_pattern =
            "(" + std::to_string(labels[right_leaf]) + "," + std::to_string(left_leaf) + ")";
        // Replace the old pattern with the new pattern
        pos = newick.find(old_pattern);
        if (pos != std::string::npos) {
            newick.replace(pos, old_pattern.length(), new_pattern);
        }
    }
}

std::vector<int> toVector(std::string newick, int num_leaves) {
    std::vector<int> v(num_leaves, 0);
    std::vector<bool> processed(num_leaves, false);
    std::vector<int> vmin(num_leaves, 0);

    std::vector<int> labels(num_leaves);
    std::iota(labels.begin(), labels.end(), 0);

    try {
        for (int i = 0; i < num_leaves - 1; ++i) {
            std::pair<int, int> tmp = findLeftLeaf(newick, labels, processed, num_leaves);
            int left_leaf = tmp.first;
            int idx = tmp.second;

            int left_leaf_ind =
                std::distance(labels.begin(), std::find(labels.begin(), labels.end(), left_leaf));

            int right_leaf = num_leaves - idx - 1;

            updateVmin(vmin, right_leaf, num_leaves, processed);

            labels[left_leaf_ind] = *std::max_element(labels.begin(), labels.end()) + 1;

            v[right_leaf] = vmin[right_leaf] == 0 ? left_leaf_ind : vmin[right_leaf];

            // Update the processed vector
            processed[right_leaf] = true;

            updateNewick(newick, left_leaf_ind, left_leaf, right_leaf, labels);
        }
    } catch (const std::out_of_range &e) {
        throw std::out_of_range(
            "Have you tried reroot=True? "
            "Are the Newick nodes integers (and not taxa)? "
            "If the error still persists, your tree might be "
            "unrooted or non-binary.");
    }

    return v;
}

void processNewick(std::string &newick) {
    removeBranchLengthAnnotations(newick);
    removeParentAnnotations(newick);
}

Newick2VResult newick2v(std::string &newick, int num_leaves) {
    processNewick(newick);

    if (num_leaves == -1) {
        num_leaves = getNumLeavesFromNewick(newick);
    }

    std::vector<int> v = toVector(newick, num_leaves);

    Newick2VResult res = {v, num_leaves};

    return res;
}

Newick2VResult newick2vWithMapping(std::string &newick, int num_leaves) {
    // Newick2VResult res;
    processNewick(newick);

    if (num_leaves == -1) {
        num_leaves = getNumLeavesFromNewick(newick);
    }

    std::map<std::string, std::string> mapping = integerizeChildNodes(newick);

    std::vector<int> v = toVector(newick, num_leaves);

    Newick2VResult res = {v, num_leaves, mapping};

    return res;
}


std::map<int, std::string> convertMapping(std::map<std::string, std::string> mapping) {
    std::map<int, std::string> converted_mapping;
    for (const auto& pair : mapping) {
        int key = std::stoi(pair.first);
        converted_mapping[key] = pair.second;
    }

    return converted_mapping;
}

// This is the exported function
std::vector<int> doToVector(std::string& newick, int num_leaves, bool with_mapping) {
    std::vector<int> converted_v;

    if (with_mapping) {
        Newick2VResult tmp = newick2vWithMapping(newick, num_leaves);
        converted_v = tmp.v;

        std::map<int, std::string> converted_mapping = convertMapping(tmp.mapping);

        std::cout << "Number of leaves: " << tmp.num_leaves << std::endl;

        std::cout << "Mapping:" << std::endl;
        for (const auto& elem : converted_mapping) {
            std::cout << elem.first << "->" << elem.second << std::endl;
        }
    } else {
        converted_v = newick2v(newick, num_leaves).v;
    }

  return converted_v;
}
