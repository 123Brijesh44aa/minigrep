# An I/O project : Building a Command Line Program to Learn Rust File System.


Let's break this down into simpler terms:

### **Creating a Project**

First, we are creating a Rust project called `minigrep`, which is a simplified version of the well-known `grep` tool used for searching text in files. 

- You use the command `cargo new minigrep` to create the project, then move into that directory with `cd minigrep`.

### **Accepting Command Line Arguments**

We want our program to accept two inputs (called *command-line arguments*):
1. A string to search for in a file (e.g., "test").
2. The file path to search in (e.g., "sample.txt").

You’ll run the program like this:
```
$ cargo run -- searchstring example-filename.txt
```
The two arguments are `searchstring` (the text you want to search for) and `example-filename.txt` (the file to search in).

### **Reading the Command Line Arguments**

To read these arguments, we use `std::env::args`. This function provides an *iterator*, which is a series of values that you can loop through. We convert it into a list (called a *vector* in Rust) using the `collect` method.

Here’s the code that reads the arguments:

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);
}
```

This code does the following:
- `env::args()` gets the command-line arguments.
- `collect()` turns them into a vector of strings.
- `dbg!(args)` prints out the arguments for debugging purposes.

If you run the program:
```bash
$ cargo run
```
You’ll see that the first argument is always the program’s name, followed by any arguments you provided. For example:
```bash
$ cargo run -- needle haystack
```
Will show:
```
["target/debug/minigrep", "needle", "haystack"]
```

Here, `"target/debug/minigrep"` is the program name, `"needle"` is the string you're searching for, and `"haystack"` is the file name.

### **Handling Invalid Unicode**

The function `std::env::args()` can crash if an argument contains invalid Unicode. For programs that need to handle such cases, you can use `std::env::args_os()` instead, but it’s more complex, so we're sticking with `args()` for now.

### **Saving the Argument Values**

Next, we store the two important arguments (the search string and the file name) in variables so we can use them later:

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {query}");
    println!("In file {file_path}");
}
```

- `args[0]` is the program name, which we don’t care about.
- `args[1]` is the search string (e.g., "test").
- `args[2]` is the file path (e.g., "sample.txt").

We use `&args[1]` and `&args[2]` to borrow references to those values, then print them out.

If you run the program:
```bash
$ cargo run -- test sample.txt
```
It will output:
```
Searching for test
In file sample.txt
```

Now, your program is reading and using the command-line arguments correctly! Up next, you’ll add file-reading functionality and error handling to deal with situations like missing arguments.



