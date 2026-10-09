#pragma once
#include <stdio.h>

#define test_assert(...)                                                       \
  do {                                                                         \
    if (!(__VA_ARGS__)) {                                                      \
      printf("TEST ASSERTION FAILED: " #__VA_ARGS__ "\n");                     \
      *(volatile int *)0 = 0;                                                  \
    }                                                                          \
  } while (0)
