#include "mem.h"

void drop::free(void *ptr) { ::free(ptr); }
