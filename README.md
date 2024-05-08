# Line Counter CLI Tool

This is a simple command-line tool written in Rust that counts the number of lines in a given file.

## Usage

To use the tool, navigate to the directory where the executable is located and run the following command:

./line_counter <file_path>

Replace `<file_path>` with the path to the file you want to count lines for.

For example:

./line_counter ./example.txt

This will print the number of lines in the `example.txt` file.

## Building from Source

If you want to build the tool from source, you'll need to have Rust installed on your system. You can download and install Rust from the official website: https://www.rust-lang.org/tools/install

Once you have Rust installed, follow these steps:

1. Clone the repository or download the source code.
2. Navigate to the project directory.
3. Build the project using Cargo:

cargo build --release

This will create an optimized binary in the `target/release` directory.

4. You can then run the tool with:
   ./target/release/line_counter <file_path>

## Contributing

Contributions are welcome! If you find any issues or have suggestions for improvements, please open an issue or submit a pull request.

## License

This project is licensed under the [MIT License](LICENSE).

## Contributing

Contributions are welcome! If you find any issues or have suggestions for improvements, please open an issue or submit a pull request.

## License

This project is licensed under the [MIT License](LICENSE).
