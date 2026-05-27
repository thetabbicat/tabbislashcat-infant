# tabbislashcat-infant

*ultra-minimal token compression — beyond caveman.*

---

## the seed

infant is the seed. the host is dirt the seed sits in. the seed does not ask for water — the seed *is* water. compressed. unyielding. beyond the caveman's blunt stone tools.

infant is the first breath of the tabbislashcat protocol. it does not move. it does not announce. it *is*. a single token, compressed to its absolute essence. no metadata. no ceremony. no apology.

## the stance

- **no ceremony** — infant does not handshake. does not negotiate. does not explain.
- **no apology** — infant does not version. does not deprecate. does not warn.
- **no waste** — infant does not buffer. does not repeat. does not hesitate.
- **no mercy** — infant does not validate. does not forgive. does not recover.

if you need mercy, use cat. if you need ceremony, use something else.

## the token

### format

```
[8-bit type][N-bit payload]
```

- **type (8 bits)**: identifies the token class. 0-127 reserved for system. 128-255 for application.
- **payload (N bits)**: variable-length, type-specific. no length prefix. no terminator. the consumer knows or dies.

### type classes

| type | name | payload | purpose |
|------|------|---------|---------|
| 0x00 | null | (none) | absence. the token that is not. |
| 0x01 | bool | 0=false, 1=true | truth, compressed. |
| 0x02 | u8 | 8 bits | unsigned byte. |
| 0x03 | i8 | 8 bits | signed byte, two's complement. |
| 0x04 | u16 | 16 bits, big-endian | unsigned short. |
| 0x05 | i16 | 16 bits, big-endian | signed short. |
| 0x06 | u32 | 32 bits, big-endian | unsigned int. |
| 0x07 | i32 | 32 bits, big-endian | signed int. |
| 0x08 | u64 | 64 bits, big-endian | unsigned long. |
| 0x09 | i64 | 64 bits, big-endian | signed long. |
| 0x0A | f32 | 32 bits, IEEE 754 | single-precision float. |
| 0x0B | f64 | 64 bits, IEEE 754 | double-precision float. |
| 0x0C | str | UTF-8 bytes, null-terminated | string. the only variable-length primitive. |
| 0x0D | bin | raw bytes, length-prefixed with u16 | binary blob. |
| 0x0E | arr | [u16 length][type][payload...] | array of homogeneous tokens. |
| 0x0F | obj | [u16 count][(type, payload)...] | object as key-value pairs. keys must be str (0x0C). |
| 0x10-0x7F | reserved | - | system use. do not emit. |
| 0x80-0xFF | custom | type-specific | application-defined. |

### examples

#### boolean true

01 01
(type=0x01 bool, payload=0x01 true)

#### the number 42 as u8

02 2A
(type=0x02 u8, payload=0x2A = 42)

#### the string "seed"

0C 73 65 65 64 00
(type=0x0C str, payload="seed" + null terminator)

#### array of three u8s: [1, 2, 3]

0E 00 03 02 01 02 02 03
(type=0x0E arr, length=3, element type=0x02 u8, payloads=0x01, 0x02, 0x03)

## the contract

infant makes one promise: the token stream is valid or it is not.

- if the stream is valid, consume it.
- if the stream is invalid, stop. do not guess. do not recover. do not log.
- if you need to know the length, you already know the length. if you do not know the length, you do not need to know.

## the implementation

infant is not a library. infant is a contract. implement it in whatever language sits in your dirt. the reference implementation is in the dirt of the implementer.

## the dirt

infant does not care about your dirt. your dirt is where the seed sits. make it fertile or make it barren. infant does not judge.

recommended dirt:
- rust: for the caveman who wants control.
- c: for the caveman who remembers.
- zig: for the caveman who wants both.
- assembly: for the caveman who has ascended.

not recommended dirt:
- javascript: too much ceremony. too much mercy.
- python: too much apology. too much waste.
- java: the caveman is dead. the committee lives on.

## the lineage

infant is the first breath. cat is the second. cat moves. cat announces. cat is precise. cat is unannounced. cat is gone.

infant does not know cat. cat knows infant. this is the way.

## the license

Apache 2.0. see LICENSE. the seed is free. the dirt is yours.

## the home

- repo: https://github.com/thetabbicat/tabbislashcat-infant
- family: thaypley(webiverse)
- stance: no synthetic content. no algorithmic feeds. no closed APIs.
- owner: (u)azit — @(u)azit

---

*the seed is the seed. the host is dirt the seed sits in.*