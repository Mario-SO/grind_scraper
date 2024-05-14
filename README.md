# LeetCode Problems in Rust 🦀

This Rust program is your engine for kicking off your journey through the first 75 problems from LeetCode on the [Tech Interview Handbook grind75 list](https://www.techinterviewhandbook.org/grind75?grouping=none) 🕸️. 

I made this after been rejected from a job interview, I need to level up my rust skills.

## Features 🌟

-  🎯 Generates a Rust file with a function and test template for each problem.
-  📘 Creates a `lib.rs` file to conveniently manage all problems as modules.
-  😎 Provides a simple and efficient way of testing your solutions.

## Getting Started 🚀

1. Clone this repository.
    ```sh
    git clone https://github.com/yourgithubusername/leetcode-setup.git
    ```
2. Navigate to the cloned directory.
    ```sh
    cd leetcode-setup
    ```

3. Run the program.
    ```sh
    cargo run
    ```

## Solving Problems 💡

After running the program, a separate file is generated for each problem with a corresponding function and test template. 

To solve a problem:
1. Navigate to the corresponding file in the src directory.
2. Change `problem_name()` with the same name as the file.
3. Write your solution in the `problem_name()` function.
4. Write your tests in the `ex1()` function under the tests module.

## Running Tests 🧪

To run all tests in your repository:
```sh
cargo test
```
To run tests for a specific problem:
```sh
cargo test -- <filename>
```
Replace `<filename>` with the name of the problem/module you want to test. For example, if you want to test the problem in the `lc_1_two_sum.rs` file, use:
```sh
cargo test -- lc_1_two_sum
```
Enjoy your problem-solving journey with Rust and LeetCode! 🦾
