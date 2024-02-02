#include <assert.h>
#include <pwd.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

/**
 * @brief A function that calls `getpass` and copies the
 * result into \p dst , which has size \p size .
 *
 * @param dst A pointer to the location where to store the
 * read bytes.
 * @param size The size of \p dst , in bytes. Note that this
 * value must be greater than one, else the process will get
 * terminated (unless `NDEBUG` gets defined at compile time,
 * which this program assumes it won't).
 * @return size_t The number of read bytes that have been
 * copied to \p dst . This number will always be at least
 * 1 less than \p size , because it gets terminated by `\0`.
 * @note This function has been written as a wrapper to
 * be called by Rust's FFI bindings.
 */
size_t my_lib_getpass(char *dst, size_t size)
{
    assert(size > 1);
    char *user_input = getpass("");
    size_t n = strlen(user_input);
    if (n + 1 > size)
    {
        memcpy(dst, user_input, size - 1);
        dst[size - 1] = '\0';
        n = size - 1;
    }
    else
    {
        strcpy(dst, user_input);
    }
    while (*user_input != '\0')
    {
        *user_input = '\0';
        user_input += 1;
    }
    return n;
}
