#include <stdio.h>
void test();

int main() {
  printf("\n\n");
  printf("RUNNING TEST ID %x\n", DROPTEST);
  printf("============================\n");
  test();
  printf("TEST ID %x PASSED\n", DROPTEST);
}
