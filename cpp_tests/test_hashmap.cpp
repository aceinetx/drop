#include "hash_map.h"
#include "str.h"
#include "testrt.h"

using namespace drop;

void test() {
  auto hashmap = HashMap<string, s32, slice_hash, slice_compare>();
  hashmap.init(128);

  hashmap.put(strlit("abc"), 69);
  hashmap.put(strlit("def"), 420);
  test_assert(hashmap.get(strlit("def")) == 420);
  test_assert(hashmap.get(strlit("abc")) == 69);

  hashmap.deinit();
}
