#include "list.h"

int main() {
  auto list = drop::List<int>::init();
  list.append(123);

  list.deinit();
}
