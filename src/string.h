#include "slice.h"
#include <cstring>

namespace drop {
using string = slice<u8>;

inline string strlit(const char *s) { return string{(u8 *)s, strlen(s)}; }
} // namespace drop
