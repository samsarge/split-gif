# split-gif

Split a GIF file into frames and save them as images. Allows different file type ouputs, currently jpg and png.

## Usage

Clone and run with cargo:
```
cargo run --release -- -i my/gif/path.gif -o png
```

Or you can compile the executable to `/target/release/split-gif` with:
```
cargo build --release
# if you want
cp ./target/release/split-gif /usr/local/bin/split-gif
# then just
split-gif -i example.gif -o jpg
```


# Help

```
Usage: split-gif --input-path <INPUT_PATH> --output-type <OUTPUT_TYPE>

Options:
  -i, --input-path <INPUT_PATH>
          Input GIF path

  -o, --output-type <OUTPUT_TYPE>
          Output file type of the frames

          [possible values: png, jpg]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
