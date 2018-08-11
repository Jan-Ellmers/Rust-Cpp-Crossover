# Rust-Cpp-Crossover
A example how to call C++ from Rust

Let's look at the key lines. First we have to include `libc` so we add `libc = "0.2.0"` to the `Cargo.toml` and a `pub extern crate libc;` to the `main.rs`. In this particular case we need `c_int` from `libc`.

Then we have to add an extern block to declare our C++ function:

    #[link(name = "test")]
    extern "C" {
        fn hello_world () -> c_int;
    }

The `"C"` tells Rust that it should try to link to a C function. Rust cannot link directly to C++ so we need to build a C interface in C++ later.

Just above the extern block we tell Rust where to find our declared functions in this case we will name the output file `libtest.so` so we type `name = "test"`.

In our `main` function we can now call `hello_world()`. But because we enter C/C++ code here it is unsafe to call `hello_world()` and therefore we have to wrap it in an unsafe block.

Of course we have to implement `Hello_world()` so we write a `.h` file in this case i named it `foo.h`. It is very important to add another extern block here and declare our function again inside the block. This time in C notation of course.

    extern "C" {
        int hello_world();
    }

Now we can write our implementation in a `.cpp` file in this case `foo.cpp`. Here is nothing special about it just inculde `foo.h` and implement the function.

Now to the fundamental part: Link everything together. First we add `build = "build.rs"` to `Cargo.toml` in the `[package]` section. `build.rs` is a independent sub rust program that prints out information for building the main program.
The key lines are:

    println!(r"cargo:rustc-env=LD_LIBRARY_PATH={}", path);
    println!(r"cargo:rustc-link-search=native={}", path);

Where path is path to the `libtest.so`.  <br/>
The first line set a variable like `export LD_LIBRARY_PATH=<path>` would do it.
Without it we would get an error like this:

    cannot open shared object file: No such file or directory
The second line tells Rust where to look for `libtest.so` it is a bit unintuitive to tell Rust twice where this file is but you cannot run the program with only one of those two lines.

Practical you could compile the C++ files in the `build.rs` using `std::process::Command` and you can also determine the path to `libtest.so`. I would recommend to put it into the `target` directory so `cargo clean` can delete it.  <br/>
For the compiling we simply call `g++ -shared -fpic foo.cpp -o target/libtest.so`.


Now we can just call `cargo run` and it will compile the C++ code, link everything and run the program. And when we call `cargo clean` we just delete every compiled file so we end up with just the source code.