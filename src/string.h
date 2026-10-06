#include "base.h"
#include "mem.h"
#include "slice.h"

namespace drop {
template <> struct slice<u8> {
  u8 *ptr;
  size_t len;

  u8 &operator[](size_t i) {
    assert(i < len);
    return ptr[i];
  }

  slice<u8> dupe() {
    slice<u8> newp = alloc<u8>(len);
    memcpy(newp.ptr, ptr, len);
    return newp;
  }
};

using string = slice<u8>;

inline string strlit(const char *s) { return string{(u8 *)s, strlen(s)}; }
} // namespace drop
