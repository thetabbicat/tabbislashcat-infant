# tabbislashcat-infant — technical specification

## abstract

infant is an ultra-minimal binary token serialization format. it encodes primitive values and composite structures with zero overhead: no length prefixes, no type descriptors beyond a single byte, no framing. the consumer either understands the format or it does not. there is no negotiation.

## data model

### primitives

| type byte | name   | encoding              | notes |
|-----------|--------|----------------------|-------|
| 0x00      | null   | (none)               | absence of value |
| 0x01      | bool   | 0x00 / 0x01          | false / true |
| 0x02      | u8     | 8 bits, unsigned      | 0-255 |
| 0x03      | i8     | 8 bits, signed        | -128 to 127, two's complement |
| 0x04      | u16    | 16 bits, big-endian   | 0-65535 |
| 0x05      | i16    | 16 bits, big-endian   | -32768 to 32767 |
| 0x06      | u32    | 32 bits, big-endian   | 0-4294967295 |
| 0x07      | i32    | 32 bits, big-endian   | -2147483648 to 2147483647 |
| 0x08      | u64    | 64 bits, big-endian   | 0-18446744073709551615 |
| 0x09      | i64    | 64 bits, big-endian   | -9223372036854775808 to 9223372036854775807 |
| 0x0A      | f32    | 32 bits, IEEE 754     | single-precision float |
| 0x0B      | f64    | 64 bits, IEEE 754     | double-precision float |

### variable-length primitives

| type byte | name | encoding | notes |
|-----------|------|-----------|-------|
| 0x0C      | str  | UTF-8 bytes + null terminator | maximum length: unlimited |
| 0x0D      | bin  | u16 length + raw bytes | length-prefixed binary data |

### composite types

| type byte | name | encoding | notes |
|-----------|------|-----------|-------|
| 0x0E      | arr  | u16 length + type byte + payload... | homogeneous array |
| 0x0F      | obj  | u16 count + (key, value)... | object with string keys |

## encoding examples

### boolean true
01 01

### u8 value 42
02 2A

### string "hello"
0C 68 65 6C 6C 6F 00

### array of u8 [1, 2, 3]
0E 00 03 02 01 02 02 03

### object {"a": 1, "b": true}
0F 00 02 0C 61 00 02 01 0C 62 00 01 01

## decoding algorithm

1. read type byte
2. switch on type byte
3. read payload according to type
4. if error: stop immediately, return error

## encoding algorithm

1. determine value type
2. emit type byte
3. emit payload according to type

## error handling

infant has no error recovery. detect invalid data and stop immediately.

## security considerations

Implementations must validate input lengths, check for overflows, and limit recursion depth.

## interoperability

- big-endian byte order for all multi-byte integers
- IEEE 754 for floating point
- UTF-8 for strings
- two's complement for signed integers

## versioning

infant has no version field. the format is immutable.

## reference implementations

see the main README for examples.