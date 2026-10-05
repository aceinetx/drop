#pragma once
#include "slice.h"
#include <cassert>
#include <cstdlib>

namespace drop {
template <typename T> T *create() {
  T *ptr = malloc(sizeof(T));
  assert(ptr && "malloc failed");
  return ptr;
}

template <typename T> slice<T> alloc(size_t len) {
  T *ptr = malloc(sizeof(T) * len);
  assert(ptr && "malloc failed");
  return {ptr, len};
}

void free(void *ptr);
template <typename T> void free(slice<T> slice) { ::free(slice.ptr); }

template <typename T> slice<T> realloc(slice<T> s, size_t len) {
  if (len == 0) {
    if (s.ptr) {
      ::free(s.ptr);
      s.ptr = NULL;
    }
  } else {
    s.ptr = (T *)::realloc(s.ptr, len * sizeof(T));
  }
  s.len = len;
  return s;
}
} // namespace drop
