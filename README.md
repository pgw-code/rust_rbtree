

# Red-Black Tree in Rust

This project implements a **Red-Black Tree** in Rust, a self-balancing binary search tree that ensures that the tree remains balanced by enforcing certain properties related to node colors during insertions.

## Features

- **Red-Black Tree Implementation**: The tree automatically balances itself during insertions, ensuring that search operations remain efficient with a time complexity of O(log n).
- **Insertion with Balancing**: The tree performs rotations (left and right) and recoloring as needed to maintain the Red-Black properties.
- **In-Order Traversal**: The tree supports in-order traversal, which visits nodes in sorted order.
- **Debug Output**: The insertion and rotation processes include debug prints to show the changes in the tree structure.

## Red-Black Tree Properties

A Red-Black Tree has the following properties:

1. Every node is either red or black.
2. The root is always black.
3. Every leaf (NIL node) is black.
4. If a red node has children then, the children are black.
5. Every path from a node to its descendant NIL nodes has the same number of black nodes.

These properties ensure that the tree remains balanced with a height of O(log n), guaranteeing efficient operations.

## Usage

### Prerequisites

- **Rust**: Make sure you have [Rust](https://www.rust-lang.org/learn/get-started) installed on your machine. You can install it using `rustup`.

### Running the Project

To run the program:

1. Clone the repository:

   

2. Run the program with:

   ```bash
   cargo run
   ```

This will insert a set of integers into the Red-Black Tree and perform an in-order traversal, printing the tree nodes in sorted order.

### Example Output

```
Inserting root: 20
Inserting 15 as left child of 20
Inserting 25 as right child of 20
Inserting 10 as left child of 15
Inserting 5 as left child of 10
Inserting 1 as left child of 5
Inserting 30 as right child of 25
Inserting 18 as right child of 15

In-order traversal of the Red-Black Tree:
1 (Black)
5 (Red)
10 (Black)
15 (Black)
18 (Red)
20 (Black)
25 (Black)
30 (Black)
```

### Code Explanation

- **`Node<T>`**: This struct represents a single node in the tree. Each node contains:
  - `data`: The value stored in the node.
  - `color`: The color of the node (`Red` or `Black`).
  - `left`, `right`, and `parent`: Pointers to the child and parent nodes.
  
- **`RedBlackTree<T>`**: This struct represents the Red-Black Tree. It provides methods for:
  - **`new`**: Creates a new empty Red-Black Tree.
  - **`insert`**: Inserts a new node into the tree while maintaining the Red-Black properties.
  - **`fix_insert`**: Fixes any violations of the Red-Black properties after insertion.
  - **`left_rotate` and `right_rotate`**: Perform rotations to maintain tree balance.
  - **`inorder`**: Performs an in-order traversal and prints the node data in sorted order.


## Contributing

Contributions to this project are welcome! Feel free to open an issue or create a pull request with improvements or fixes.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
