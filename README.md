# BinaryModifier
BinaryModifier is a Rust library designed to simplify working with binary data streams. It provides a set of tools for reading and writing various data types in binary format, handling endianness, seeking within streams, and more. This crate is particularly useful when dealing with tasks such as parsing binary file formats or implementing network protocols.

# Features
- Read and write primitive data types like integers, floats, characters, and more in binary format.
- Read and write length-prefixed strings from and to binary streams.
- Seek within the stream to a specific position.
- Handle endianness swapping during reading and writing processes.
- Support different data types and conversions based on the target architecture.

# Usage 
The Binary Modifier crate is centered around two main structs: BinaryReader and BinaryWriter. Both of these structs interact with an underlying MemoryStream, which abstracts a byte buffer for efficient in-memory manipulation of binary data.

## BinaryReader
The BinaryReader struct is used for reading data from binary streams

Example of reading an integer from the stream:
```rs
use binary_modifier::{BinaryReader, Endian};

let data = vec![0x01, 0x02, 0x03, 0x04];
let mut reader = BinaryReader::new_vec(&mut data, Endian::Big);
let value: i32 = reader.read_i32().unwrap();
println!("Read integer: {}", value);
```

## BinaryWriter
The BinaryWriter struct is used for writing data to binary streams. 

Example of writing a string to the stream:
```rs
use binary_modifier::{BinaryWriter, Endian};

let mut data = Vec::new();
let mut writer = BinaryWriter::new_vec(&mut data, Endian::Little);
let text = "Hello, Binary!";
writer.write_string(text).unwrap();
println!("Data written: {:?}", data);
```

# Usage
To use the Binary Modifier crate in your Rust project, add the following dependency to your Cargo.toml file:
```toml
binary_modifier = "0.2.1"
```
Or use cargo to install it using
```
cargo add binary_modifier
```

# License
This crate is licensed under the MIT License.

______
If you encounter any issues or have suggestions for improvements, please don't hesitate to contribute or report issues on GitHub.

