# split-gif
Split a GIF file into frames and save them as images. Allows different file type ouputs, currently jpg and png.

## Setup
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

## Examples
```
# split into pngs
split-gif -i ./images/example.gif -o png
# split into 10 jpg frames
split-gif -i ./images/example.gif -o jpg -m 10
```

## Help
```
Usage: split-gif [OPTIONS] --input-path <INPUT_PATH> --output-type <OUTPUT_TYPE>

Options:
  -i, --input-path <INPUT_PATH>
          Input GIF path

  -o, --output-type <OUTPUT_TYPE>
          Output file type of the frames

          [possible values: png, jpg]

  -m, --max-frames <MAX_FRAMES>
          Optional argument to shorten the output to a certain amount of frames.
          For example; setting a 20 frame gif to 10 frames will save every 2nd frame

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
