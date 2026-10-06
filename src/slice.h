#pragma once
#include <cassert>
#include <cstddef>
#include <cstdint>
#include <cstdlib>
#include <cstring>

namespace drop {
template <typename T> struct slice {
  T *ptr;
  size_t len;

  T &operator[](size_t i) {
    assert(i < len);
    return ptr[i];
  }
};

template <> struct slice<uint8_t> {
  uint8_t *ptr;
  size_t len;

  uint8_t &operator[](size_t i) {
    assert(i < len);
    return ptr[i];
  }

  slice<uint8_t> dupe() {
    void *newp = malloc(len);
    memcpy(newp, ptr, len);
    return {(uint8_t *)newp, len};
  }
};
} // namespace drop
