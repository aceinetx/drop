#include <stdio.h>
void test();

int main() {
  printf("\n\n");
  printf("RUNNING TEST ID %d\n", DROPTEST);
  printf("============================\n");
  test();
  printf("TEST ID %d PASSED\n", DROPTEST);
}
