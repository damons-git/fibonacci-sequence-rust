# Fibonacci Sequence

Fibonacci sequence generator written in Rust.

## Testing
To execute the test suite for this fibonacci sequence generator run the command:
```
$ cargo test
```

## Building
To build the application run:
```
$ cargo build
```

## Excuting the program
To execute the program, ensure the application has been built, and run the following command:
```
$ ./target/debug/fibonacci-rust
```

By default this program will generate and display the fibonacci sequence to a depth of 25. Using the 'depth' command line argument this can be increased or lowered.

```
$ ./target/debug/fibonacci-rust --depth 30
```
```
$ ./target/debug/fibonacci-rust -d 30

```

## Note
This program was written as a learning tool to become more familiar with the syntax of Rust. For this reason signed 64-bit integers have been used to express values within the fibonacci sequence. At a depth of greater than 93 the largest value in this sequence exceeds that in which can be expressed.