# Rust Insert Read Example

This is an example Rust program that demonstrates how to insert values 1-100 into a Redis list and then retrieve them and print them in reverse order.

## Prerequisites

To run this program, you will need to have Rust installed on your system. If you do not have Rust installed, you can download and install it from the [official Rust website](https://www.rust-lang.org/tools/install).

You will also need to have a Redis server running on your local machine or network. You can download and install Redis from the [official Redis website](https://redis.io/download).

## Running the Program

To run the program, follow these steps:

1. Clone this repository to your local machine:
```
git clone https://github.com/boriguen/rust-insert-read.git
```

2. Navigate to the project directory:
```
cd rust-insert-read
```

3. Install the Redis crate:
```
cargo update
```

4. Build the project:
```
cargo build --release
```

5. Run the program:
```
./target/release/rust-insert-read
```

If everything is working correctly, you should see the values 100-1 printed to the console.

## Configuring the Redis Server

Currently, the program has hardcoded Redis database params. If your Redis server is running on a different host or port, you can modify the connection string in the `main()` function of the `main.rs` file.

These connection details will be moved into ENV variables at some point.

## License

This program is licensed under the MIT License. See the LICENSE file for details.
