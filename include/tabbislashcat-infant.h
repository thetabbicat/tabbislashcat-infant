/*
 * tabbislashcat-infant — C header
 * ultra-minimal token compression — beyond caveman.
 * 
 * the seed is the seed. the host is dirt the seed sits in.
 */

#ifndef TABBISLASHCAT_INFANT_H
#define TABBISLASHCAT_INFANT_H

#include <stdint.h>
#include <stdbool.h>

/* token types */
#define INFANT_NULL  0x00
#define INFANT_BOOL  0x01
#define INFANT_U8    0x02
#define INFANT_I8    0x03
#define INFANT_U16   0x04
#define INFANT_I16   0x05
#define INFANT_U32   0x06
#define INFANT_I32   0x07
#define INFANT_U64   0x08
#define INFANT_I64   0x09
#define INFANT_F32   0x0A
#define INFANT_F64   0x0B
#define INFANT_STR   0x0C
#define INFANT_BIN   0x0D
#define INFANT_ARR   0x0E
#define INFANT_OBJ   0x0F

/* forward declaration */
typedef struct infant_token infant_token_t;

/* token union */
typedef union {
    bool b;
    uint8_t u8;
    int8_t i8;
    uint16_t u16;
    int16_t i16;
    uint32_t u32;
    int32_t i32;
    uint64_t u64;
    int64_t i64;
    float f32;
    double f64;
    char *str;
    struct {
        uint8_t *data;
        uint16_t len;
    } bin;
    struct {
        infant_token_t *elements;
        uint16_t len;
    } arr;
    struct {
        char **keys;
        infant_token_t *values;
        uint16_t count;
    } obj;
} infant_value_t;

/* token structure */
struct infant_token {
    uint8_t type;
    infant_value_t value;
};

/* decode a token from buffer */
/* returns: number of bytes consumed, or 0 on error */
size_t infant_decode(const uint8_t *buf, size_t len, infant_token_t *token);

/* encode a token to buffer */
/* returns: number of bytes written, or 0 on error */
size_t infant_encode(const infant_token_t *token, uint8_t *buf, size_t len);

/* free token memory */
void infant_free(infant_token_t *token);

/* the dirt is yours */

#endif /* TABBISLASHCAT_INFANT_H */
