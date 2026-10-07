#include "slice.h"
#include <ctype.h>
#include <string.h>

namespace drop {
using string = slice<u8>;

inline string strlit(const char *s) { return string{(u8 *)s, strlen(s)}; }

inline bool isident(u8 ch) { return isalnum(ch) || ch == '_'; }
} // namespace drop
