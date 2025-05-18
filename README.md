# Toy PNG Decoder (Rust)

学習目的のPNGデコーダー

## 使用例

```
$ cargo run --example ex1 examples/test1.png

...

IHDR: len:13, crc: 0x20DB1B2
IDAT: len:64, crc: 0xC8824552
IEND: len:0, crc: 0xAE426082
----------------
IHDR
    width: 5
    height: 5
    bit_depth: 8
    colour_type: 2 (Truecolour (8, 16) bit)
    compression_method: 0 (deflate/inflate compression with a sliding window of at most 32768 bytes)
    filter_method: 0 (adaptive filtering with five basic filter types)
    interlace_method: 0 (no interlace)
----------------
IDAT
    chunks: 1
    length: 64

```