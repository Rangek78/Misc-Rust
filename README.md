# Miscelaneous projects in Rust
## Introduction
This repository contains projects written in Rust covering the following topics:
- Binary Tree (only implemented insert and node count functions)
- LinkedList
- Sorting Algorithms - Bubble, Selection and Insertion Sort

## Binary Tree
In the Binary Tree implementation, a function for inserting a node and a function to count the number of elements in the tree were made, two for each. Those ending with `_iter` were created to execute using iteration, while the ones which don't end in `_iter` use recursion.

## Linked List
This project is quite long to be in a single `main.rs` file, but it can easily be divided into other files by moving the declared modules to different files.
It was designed to have an interactive cli menu (in portuguese) for you to create, read, update or delete a student in the list. The list will only exist as long as the program is running (in RAM), meaning a persistent database was not implemented.

## Sorting Algorithms
Three classic sorting algorithms functions were made: Bubble Sort, Selection Sort and Insertion Sort. The local variables `assignments` and `comparisons` keep track of how many assignment and comparison operations were made in the algorithm for each test vector. These implementations can be used to benchmark the algorithms, but the additional helper variables may influence in the results, so either keep that in mind or comment them out. The results are logged to the console when the algorithm ends.
