#ifndef DA_H
#define DA_H

#include <stddef.h>

#define DECLARE_DYNAMIC_ARRAY(T, Name) \
    typedef struct {                   \
        T *items;                      \
        size_t count;                  \
        size_t capacity;               \
    } Name

#define da_append(xs, x) \
    do { \
        if ((xs)->count >= (xs)->capacity) { \
            if ((xs)->capacity == 0) (xs)->capacity = 256; \
            else (xs)->capacity *= 2; \
            (xs)->items = realloc((xs)->items, (xs)->capacity*sizeof(*(xs)->items)); \
        } \
        (xs)->items[(xs)->count++] = (x); \
    } while (0)\

#define da_free(da) \
    free((da)->items)

#endif
