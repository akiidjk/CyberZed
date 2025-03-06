# CyberZed Extension

CyberZed is an extension designed for the Zed text editor, which provides a number of useful commands for manipulating strings and performing encoding, hashing, UUID generation and XOR operations between strings of bytes.

This extension includes the following commands:

- **Encoding and Decoding**: Base64, Hex, URL
- **Hashing**: SHA256, SHA384, SHA512, MD5
- **UUID Generation**.
- **XOR operations** between two strings of bytes

## Main functionalities

### Encryption Commands

1. **Base64**
   - `encode`: Encodes a string in Base64 format.
   - `decode`: Decodes a Base64 string into readable text.

2. **Hex**
   - `encode`: Encodes a string into hexadecimal format.
   - `decode`: Decodes a hexadecimal string into readable text.

3. **URL**.
   - `encode`: Encodes a string into URL encoding.
   - `decode`: Decodes an encoded URL string.

### Hashing Commands

1. **SHA**
   - `sha256`: Calculates the SHA-256 hash of a string.
   - `sha384`: Calculates the SHA-384 hash of a string.
   - `sha512`: Calculates the SHA-512 hash of a string.

2. **MD5**
   - md5`: Calculates the MD5 hash of a string.

### UUID generation

- `uuid`: Generates a new UUIDv4.

### XOR between two strings of bytes

The `xor_bytes` function performs the XOR operation between two arrays of bytes. If the array lengths are different, the longer array is truncated to the length of the shorter array. The operation is limited to the length of the shorter string.

### Project structure

```plaintext
├── commands
│ ├── encoding.rs # Handles encoding commands (base64, hex, URL)
│ ├── hashing.rs # Handles hashing commands (SHA, MD5)
│ └── mod.rs # Main module that includes all commands
├── cyberzed.rs # Main file recording the CyberZed extension
└── utils
    ├── uuid_gen.rs # UUID Generation
    └── xor.rs # XOR function between two arrays of bytes
```

### Supported commands

1. **Base64**
   - `encode`: Encodes a string in base64.
   - `decode`: Decodes a base64 string into readable text.
2. **Hex**
   - `encode`: Encodes a string into hexadecimal format.
   - `decode`: Decodes a hexadecimal string into readable text.
3. **URL**.
   - `encode`: Encodes a string into URL encoding.
   - `decode`: Decodes an encoded URL string.
4. **SHA**
   - `sha256`, `sha384`, `sha512`: Calculates the corresponding SHA hash.
5. **MD5**
   - `md5`: Calculates the MD5 hash of a string.
6. **UUID**
   - `uuid`: Generates a new UUID.
7. **XOR**
   - `xor`: Performs an XOR operation between two strings of bytes and returns the result in hexadecimal format.

### Example of use

#### XOR between two strings

Suppose we want to perform an XOR between two strings:

```bash
$ xor [‘xor’, ‘encode’, ‘Hello’, ‘World’]
```

This command will return the result of the XOR operation between the strings ‘Hello’ and ‘World’ in hexadecimal format.

#### Base64 Encoding and Decoding

To perform Base64 encoding and decoding of a string:

```bash
$ base64 [‘base64’, ‘encode’, ‘Hello World’]
$ base64 [‘base64’, ‘decode’, ‘SGVsbG8gV29ybGQ=’]
```

#### Calculate an SHA hash

To calculate a SHA256 hash, for example:

```bash
$ sha [‘sha’, ‘256’, ‘Hello World’]
```

This will calculate the SHA256 hash of the string ‘Hello World’.

### Installation

...

### Licence

This project is released under the [Apache License](LICENSE).
