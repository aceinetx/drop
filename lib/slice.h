#pragma once
#include "base.h"
#include <assert.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>

namespace drop {
template <typename T> struct slice {
  T *ptr;
  size_t len;

  T operator[](size_t i) const {
    if (i >= len) {
      fprintf(
          stderr,
          "internal error: out of bounds slice access: (slice of [%zu])[%zu]\n",
          i, len);
      drop_trap();
    }
    return ptr[i];
  }

  T &operator[](size_t i) {
    if (i >= len) {
      fprintf(
          stderr,
          "internal error: out of bounds slice access: (slice of [%zu])[%zu]\n",
          i, len);
      drop_trap();
    }
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

template <typename T> bool slice_compare(slice<T> one, slice<T> other) {
  return !memcmp(one.ptr, other.ptr, drop_min(one.len, other.len));
}

/* Stolen from zig */
template <typename T> inline u32 slice_hash(slice<T> slice) {
  // FNV 32-bit hash
  u32 h = 2166136261;
  u8 *buff = (u8 *)slice.ptr;
  for (usize i = 0; i < sizeof(T) * slice.len; i += 1) {
    h = h ^ ((u8)buff[i]);
    h = h * 16777619;
  }
  return h;
}
} // namespace drop
