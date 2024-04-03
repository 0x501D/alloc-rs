#include <stdio.h>
#include <string.h>

typedef void *(*alloc_cb)(unsigned, unsigned);
typedef void (*free_cb)(unsigned char *, unsigned, unsigned);

typedef struct {
    alloc_cb a_cb;
    free_cb f_cb;
} demo_t;

void demo(demo_t *demo)
{
    unsigned char *p = demo->a_cb(42, 64);
    memcpy(p, "trololo", 8);
    printf("data:\'%s\'\n", p);
    demo->f_cb(p, 42, 64);
}
