// Copyright 2022 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#include <stdio.h>
#include <stdlib.h>

static char line_str[3 * sizeof(int) + 1];
static char *itoa(unsigned i) {
  char *p = line_str + 3 * sizeof(int);
  *--p = '\0';
  do {
    *--p = '0' + i % 10;
    i /= 10;
  } while (i);
  return p;
}

_Noreturn void __assert_fail(const char *expr, const char *file, int line,
                             const char *func) {
  /* Avoid using string formating here. */
  puts("Assertion failed: ");
  puts(expr);
  puts(" (");
  puts(file);
  puts(":");
  puts(itoa(line));
  puts(" ");
  puts(func);
  puts(")\n");
  abort();
}
