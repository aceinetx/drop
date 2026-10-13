// Copyright 2022 Alexey Kutepov <reximkut@gmail.com>

// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:

// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

#ifndef ARENA_H_
#define ARENA_H_
namespace drop {
#include <assert.h>
#include <stdarg.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>

#define ARENA_ASSERT assert

typedef struct Region Region;

struct Region {
  Region *next;
  size_t count;
  size_t capacity;
  uintptr_t data[1];
};

typedef struct {
  Region *region;
  size_t count;
} Arena_Mark;

struct Arena {
  Region *begin, *end;

  static Arena init();
  void deinit();

  void *alloc(size_t size_bytes);
  void *realloc(void *oldptr, size_t oldsz, size_t newsz);
  char *strdup(const char *cstr);
  void *memdup(const void *data, size_t size);

  void trim();
  void rewind(Arena_Mark m);
  void reset();
  Arena_Mark snapshot();
};

#ifndef ARENA_REGION_DEFAULT_CAPACITY
#define ARENA_REGION_DEFAULT_CAPACITY (8 * 1024)
#endif // ARENA_REGION_DEFAULT_CAPACITY

// Region* new_region(size_t capacity);
// void free_region(Region* r);

Arena_Mark arena_snapshot(Arena *a);
void arena_reset(Arena *a);
void arena_rewind(Arena *a, Arena_Mark m);
void arena_free(Arena *a);
void arena_trim(Arena *a);

#endif // ARENA_H_

#ifdef ARENA_IMPLEMENTATION

#include <stdlib.h>

// TODO: instead of accepting specific capacity new_region() should accept the
// size of the object we want to fit into the region It should be up to
// new_region() to decide the actual capacity to allocate
static Region *new_region(size_t capacity) {
  size_t size_bytes = sizeof(Region) + sizeof(uintptr_t) * capacity;
  // TODO: it would be nice if we could guarantee that the regions are allocated
  // by ARENA_BACKEND_LIBC_MALLOC are page aligned
  Region *r = (Region *)malloc(size_bytes);
  ARENA_ASSERT(r); // TODO: since ARENA_ASSERT is disableable go through all the
  // places where we use it to check for failed memory
  // allocation and return with NULL there.
  r->next = NULL;
  r->count = 0;
  r->capacity = capacity;
  return r;
}

static void free_region(Region *r) { free(r); }

// TODO: add debug statistic collection mode for arena
// Should collect things like:
// - How many times new_region was called
// - How many times existing region was skipped
// - How many times allocation exceeded ARENA_REGION_DEFAULT_CAPACITY

Arena Arena::init() {
  Arena self;
  memset(&self, 0, sizeof self);
  return self;
}

void Arena::deinit() {
  Region *r = this->begin;
  while (r) {
    Region *r0 = r;
    r = r->next;
    free_region(r0);
  }
  this->begin = NULL;
  this->end = NULL;
}

void *Arena::alloc(size_t size_bytes) {
  size_t size = (size_bytes + sizeof(uintptr_t) - 1) / sizeof(uintptr_t);

  if (this->end == NULL) {
    ARENA_ASSERT(this->begin == NULL);
    size_t capacity = ARENA_REGION_DEFAULT_CAPACITY;
    if (capacity < size)
      capacity = size;
    this->end = new_region(capacity);
    this->begin = this->end;
  }

  while (this->end->count + size > this->end->capacity &&
         this->end->next != NULL) {
    this->end = this->end->next;
  }

  if (this->end->count + size > this->end->capacity) {
    ARENA_ASSERT(this->end->next == NULL);
    size_t capacity = ARENA_REGION_DEFAULT_CAPACITY;
    if (capacity < size)
      capacity = size;
    this->end->next = new_region(capacity);
    this->end = this->end->next;
  }

  void *result = &this->end->data[this->end->count];
  memset(result, 0, size_bytes);
  this->end->count += size;
  return result;
}

void *Arena::realloc(void *oldptr, size_t oldsz, size_t newsz) {
  if (newsz <= oldsz)
    return oldptr;
  void *newptr = this->alloc(newsz);
  char *newptr_char = (char *)newptr;
  char *oldptr_char = (char *)oldptr;
  for (size_t i = 0; i < oldsz; ++i) {
    newptr_char[i] = oldptr_char[i];
  }
  return newptr;
}

char *Arena::strdup(const char *cstr) {
  size_t n = strlen(cstr);
  char *dup = (char *)this->alloc(n + 1);
  memcpy(dup, cstr, n);
  dup[n] = '\0';
  return dup;
}

void *Arena::memdup(const void *data, size_t size) {
  return memcpy(this->alloc(size), data, size);
}

Arena_Mark Arena::snapshot() {
  Arena_Mark m;
  if (this->end == NULL) { // snapshot of uninitialized arena
    ARENA_ASSERT(this->begin == NULL);
    m.region = this->end;
    m.count = 0;
  } else {
    m.region = this->end;
    m.count = this->end->count;
  }

  return m;
}

void Arena::reset() {
  for (Region *r = this->begin; r != NULL; r = r->next) {
    r->count = 0;
  }

  this->end = this->begin;
}

void Arena::rewind(Arena_Mark m) {
  if (m.region == NULL) { // snapshot of uninitialized arena
    this->reset();        // leave allocation
    return;
  }

  m.region->count = m.count;
  for (Region *r = m.region->next; r != NULL; r = r->next) {
    r->count = 0;
  }

  this->end = m.region;
}

void Arena::trim() {
  Region *r = this->end->next;
  while (r) {
    Region *r0 = r;
    r = r->next;
    free_region(r0);
  }
  this->end->next = NULL;
}
}
#endif // ARENA_IMPLEMENTATION
