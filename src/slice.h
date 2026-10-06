#pragma once
#include "base.h"
#include <cassert>
#include <cstddef>
#include <cstdlib>

namespace drop {
template <typename T> struct slice {
  T *ptr;
  size_t len;

  T &operator[](size_t i) {
    assert(i < len);
    return ptr[i];
  }

  bool compare(slice<T> other) {
    return !memcmp(ptr, other.ptr, drop_min(len, other.len));
  }

  slice<T> dupe() {
    size_t bytes = sizeof(T) * len;
    slice<T> new_slice = *this;
    new_slice.ptr = (T *)malloc(bytes);
    memcpy(new_slice.ptr, ptr, bytes);
    return new_slice;
  }
};
} // namespace drop
