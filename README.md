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









## Reading a File
Now we’ll add functionality to read the file specified in the file_path argument. First we need a sample file to test it with: we’ll use a file with a small amount of text over multiple lines with some repeated words. Listing 12-3 has an Emily Dickinson poem that will work well! Create a file called poem.txt at the root level of your project, and enter the poem “I’m Nobody! Who are you?”

`Filename: poem.txt`
``` 
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog! 

```
Listing 12-3: A poem by Emily Dickinson makes a good test case.
With the text in place, edit src/main.rs and add code to read the file, as shown in Listing 12-4.

``` rust
Filename: src/main.rs
use std::env;
use std::fs;

fn main() {
    // --snip--
    println!("In file {file_path}");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}
```
Listing 12-4: Reading the contents of the file specified by the second argument
First we bring in a relevant part of the standard library with a use statement: we need std::fs to handle files.

In main, the new statement **fs::read_to_string** takes the file_path, opens that file, and returns a value of type **std::io::Result<String>** that contains the file’s contents.

After that, we again add a temporary println! statement that prints the value of contents after the file is read, so we can check that the program is working so far.

Let’s run this code with any string as the first command line argument (because we haven’t implemented the searching part yet) and the poem.txt file as the second argument:

``` 
$ cargo run -- the poem.txt
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.0s
     Running `target/debug/minigrep the poem.txt`
Searching for the
In file poem.txt
With text:
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!

```


Great! The code read and then printed the contents of the file. But the code has a few flaws. At the moment, the main function has multiple responsibilities: generally, functions are clearer and easier to maintain if each function is responsible for only one idea. The other problem is that we’re not handling errors as well as we could. The program is still small, so these flaws aren’t a big problem, but as the program grows, it will be harder to fix them cleanly. It’s a good practice to begin refactoring early on when developing a program because it’s much easier to refactor smaller amounts of code. We’ll do that next.

