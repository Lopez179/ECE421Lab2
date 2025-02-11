# ECE 421 Lab 2
## Question 1
Go to question 1 directory
```cargo run```
## Question 2
Go to question 2 directory
Fixed in main.rs

Rust requires knowing the size of all types at compile time, because unlike most other languages, rust doesn't allow implicit heap alocation (this is necessary as part of having syntax dependent on memory safety). 

When struct contain borrowed references, lifetimes need to be explicitly specified. Other languages don't need this because they use garbage collection or manual memory management. The lifetime specifier tells the compiler that data needs to live aslong as TreeNode.
## Question 3
Go to question 3 directory
```cargo run```

## Question 4
Go to question 4 directory
```cargo run```

Options are essentially enums with variants Some and None. Empty behaves much like the none in options. That's why this code does not
options like in question 3.

The struct based approach is probably better as it's use has clearer intentions, and is more memory efficient. Enums came with overhead due to being more flexible with the types of data they can use. For this same reason, it's harder to read code that use enum data type abstractions.