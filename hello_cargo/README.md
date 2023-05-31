- We can create a project using

```sh
cargo new my_new_project
```

- We can build a project using

```sh
cargo build
```

- We can build and run a project in one step using

```sh
cargo run
```

- We can build a project without producing a binary to check for errors using

```sh
cargo check
```

- Instead of saving the result of the build in the same directory as our code, Cargo stores it in the target/debug directory.

- When your project is finally ready for release, you can use cargo build --release to compile it with optimizations.This command will create an executable in target/release instead of target/debug. The optimizations make your Rust code run faster, but turning them on lengthens the time it takes for your program to compile. This also necessary to do before benchmarking your program.

```sh
cargo build --release
```
