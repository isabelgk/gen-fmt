#ifndef GEN_FMT_H
#define GEN_FMT_H

#ifdef __cplusplus
extern "C" {
#endif

/**
 * Format a GenExpr string.
 *
 * Returns a newly-allocated C string on success, or NULL on error.
 * The caller must free the result with gen_fmt_free().
 *
 * skip_idempotence: non-zero to skip idempotence check
 * tolerate_parsing_errors: non-zero to continue despite parse errors
 */
char *gen_fmt_format(const char *input, int skip_idempotence, int tolerate_parsing_errors);

/**
 * Free a string returned by gen_fmt_format().
 */
void gen_fmt_free(char *s);

#ifdef __cplusplus
}
#endif

#endif /* GEN_FMT_H */
