#pragma once
#include <cassert>
#include <cstddef>

namespace drop {
template <typename T> struct slice {
  T *ptr;
  size_t len;

  T &operator[](size_t i) {
    assert(i < len);
    return ptr[i];
  }
};
} // namespace drop
