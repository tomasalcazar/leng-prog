#include <stdio.h>

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Utility helpers
static void print_bytes(const char *label, const unsigned char *buf, size_t n) {
    printf("%s", label);
    for (size_t i = 0; i < n; ++i) printf("%02X ", buf[i]);
    putchar('\n');
}

static void good1(void) {
    char buf[32];
    const char *s = "AAAAAAAAAAAAAAAA";
    strcpy(buf, s);
    puts(buf);
}

static void good2(void) {
    char *p = malloc(8);
    if (!p) { perror("malloc"); return; }

    for (int i = 0; i < 8; ++i)
        p[i] = 'A';

    free(p);
}

static void good3(void) {
    char s[5] = "abc";
    s[3] = 'x';
    s[4] = '\0';
    puts(s);
}

static void good4(void) {
    size_t n = SIZE_MAX / 2 + 1;
    if (n > SIZE_MAX / (2 * sizeof(int))) { puts("malloc failed"); return;}
    int *a = malloc(n * 2 * sizeof *a);
    if (!a) { perror("malloc"); return; }
    a[0] = 1;
    free(a);
}

static void good5(void) {
    int len = 5;
    char *p = malloc((size_t)len);
    if (!p) { perror("malloc"); return; }
    free(p);
}

static void good6(void) {
    int *p = (int*)malloc(sizeof *p);
    if (!p) { perror("malloc"); return; }
    *p = 7;
    free(p);
    p = NULL;
}

static void good7(void) {
    int *p = malloc(sizeof *p);
    if (!p) { perror("malloc"); return; }
    free(p);
    p = NULL;
}

static void good8(void) {
    char *p = malloc(100);
    if (!p) { perror("malloc"); return; }
    free(p);
    p = malloc(200);
    if (!p) { perror("malloc"); return; }
    free(p);
}

static void good9(void) {
    char *p = malloc(10);
    if (!p) { perror("malloc"); return; }
    char *q = p + 2;
    free(p);
}

static int* returns_static_good(void) { // La hago static para que no muera en el stack
    // Un problema podría ser que el static tiene poca memoria
    static int x = 7;
    return &x;
}

static void good10(void) {
    int *p = returns_static_good();
    printf("value: %d\n", *p);
}

static void good11(void) {
    int x = 0;
    if (x == 123) puts("equal");
}

static void good12(void) {
    size_t huge = (size_t)1 << 62;
    char *p = malloc(huge);
    if (!p) { perror("malloc"); return; }
    p[0] = 'x';
    free(p);
}

static void good13(void) {
    char *p = malloc(10);
    if (!p) { perror("malloc"); return; }
    char *tmp = realloc(p, SIZE_MAX / 4);
    if (!tmp) {
        puts("realloc failed");
        free(p);
        return;
    }
    p = tmp;
    free(p);
}

static void good14(void) {
    char buf[16] = "abcdefgh";
    memmove(buf + 2, buf, 8);
    puts(buf);
}

static void good15(void) {
    char buf[16];
    const char *a = "0123456789";
    const char *b = "ABCDEFGHIJ";
    snprintf(buf, sizeof buf, "%s %s", a, b);
    puts(buf);
}

static void rec_iter(int n) {
    while (n) n--;
}

static void good16(void) {
    rec_iter(1000000000);
}

static int static_x;

static void good18(void) {
    int local;
    (void)static_x;
    (void)local;  
}

static void good20(void) {
    char *p = malloc(10);
    if (!p) { perror("malloc"); return; }
    char *alias = p;
    alias[0] = 'x';
    free(p);
    p = NULL;
    alias = NULL;
}

int main(int argc, char **argv) {
    //good1();
    //good2();
    //good3();
    //good4();
    //good5();
    //good6();
    //good7();
    //good8();
    //good9();
    //good10();
    //good11();
    //good12();
    //good13();
    //good14();
    good15();
    //good16();
    //good18();
    //good20();
    return 0;
}