#pragma once
#include <stdint.h>
#include <string.h>

#if defined(DROPTEST)
#include "testrt.h"
#endif

namespace drop {

// types
using u8 = uint8_t;
using u16 = uint16_t;
using u32 = uint32_t;
using u64 = uint64_t;
using s8 = int8_t;
using s16 = int16_t;
using s32 = int32_t;
using s64 = int64_t;
using usize = size_t;

#define drop_min(x, y) ((x) < (y) ? (x) : (y))

// trap
#if defined(DROPTEST)
#define drop_trap() test_assert(0 && "trap signal")
#elif defined(_MSC_VER)
#include <intrin.h>
#define drop_trap() __debugbreak()

#elif defined(__clang__)
#if __has_builtin(__builtin_debugtrap)
#define drop_trap() __builtin_debugtrap()
#elif __has_builtin(__builtin_trap)
#define drop_trap() __builtin_trap()
#else
#include <signal.h>
#define drop_trap() raise(SIGTRAP)
#endif

#elif defined(__GNUC__) || defined(__GNUG__)
#if defined(__i386__) || defined(__x86_64__)
#define drop_trap() __asm__ volatile("int $3")
#else
#define drop_trap() __builtin_trap()
#endif

#else
#include <signal.h>
#define drop_trap() raise(SIGTRAP)
#endif

#define drop_bool_fmt(v) (v ? "true" : "false")
} // namespace drop
