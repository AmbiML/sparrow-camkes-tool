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

#ifndef _STDIO_H
#define _STDIO_H

#define __NEED_off_t
#define __NEED_size_t
#define __NEED_FILE
#include <_types.h>

int printf(const char *__restrict, ...);
int fprintf(FILE *__restrict, const char *__restrict, ...);
int snprintf(char *__restrict, size_t, const char *__restrict, ...);
int putchar(int);
int puts(const char *);
int fputs(const char *__restrict, FILE *__restrict);
int fflush(FILE *);
int vsnprintf(char *__restrict, size_t, const char *__restrict,
              __builtin_va_list);
size_t fwrite(const void *restrict ptr, size_t size, size_t nmemb,
              FILE *restrict stream);

#define stdin 0
#define stdout 0
#define stderr 0
#define EOF (-1)

#endif
