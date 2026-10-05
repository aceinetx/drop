#pragma once
#include "mem.h"
#include "slice.h"

namespace drop {
template <typename T> struct List {
  slice<T> items;
  size_t capacity;

  static List<T> init() {
    List<T> list;
    list.items = {0, 0};
    list.capacity = 0;
    return list;
  }

  void deinit() {
    if (this->items.ptr)
      drop::free(this->items);
    *this = init();
  }

  void append(T item) {
    if (this->items.len >= this->capacity) {
      size_t new_capacity = this->capacity * 1.5;
      if (this->capacity == 0)
        new_capacity = 64;

      this->capacity = new_capacity;

      size_t len = this->items.len;
      this->items = realloc(this->items, new_capacity);
      this->items.len = len;
    }

    this->items[this->items.len++] = item;
  }

  T pop() {
    assert(this->items.len > 0);
    T item = this->items[--this->items.len];
    return item;
  }

  void clearRetainingCapacity() { this->items.len = 0; }
};
} // namespace drop
