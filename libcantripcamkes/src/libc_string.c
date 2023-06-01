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

#include <string.h>

int strcmp(const char *s1, const char *s2) {
  for (; *s1 == *s2 && *s1 != '\0'; s1++, s2++)
    ;
  return *(unsigned char *)s1 - *(unsigned char *)s2;
}

char *strcpy(char *restrict dest, const char *restrict src) {
  const unsigned char *s = src;
  unsigned char *d = dest;
  while ((*d++ = *s++) != '\0')
    ;
  return dest;
}

size_t strlen(const char *s) {
  size_t res = 0;
  while (*(s++) != '\0') {
    res++;
  }
  return res;
}

size_t strnlen(const char *s, size_t n) {
  size_t res;
  for (res = 0; res < n; res++, s++) {
    if (*s == '\0') {
      break;
    }
  }
  return res;
}
