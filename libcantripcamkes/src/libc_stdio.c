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

#include <stdarg.h>
#include <stdio.h>
#include <string.h>

int fflush(FILE *f) {
  /* NOP */
  (void)f;
  return 0;
}

int fprintf(FILE *restrict f, const char *restrict fmt, ...) {
  /* Functionality limited to stdout */
  (void)f;

  int ret;
  va_list ap;
  va_start(ap, fmt);
  ret = printf(fmt, ap);
  va_end(ap);
  return ret;
}

int fputs(const char *restrict s, FILE *restrict f) {
  /* Functionality limited to stdout */
  (void)f;

  return puts(s);
}

size_t fwrite(const void *restrict src, size_t size, size_t nmemb,
              FILE *restrict f) {
  /* NOP. Called by unused code paths, exists to satisfy linker. */
  (void)src;
  (void)size;
  (void)f;
  return 0;
}

int snprintf(char *restrict s, size_t n, const char *restrict fmt, ...) {
  int ret;
  va_list ap;
  va_start(ap, fmt);
  ret = vsnprintf(s, n, fmt, ap);
  va_end(ap);
  return ret;
}
