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

#include <_arch_defs.h>

#if defined(__NEED_size_t) && !defined(__DEFINED_size_t)
typedef unsigned _addr size_t;
#define __DEFINED_size_t
#endif

#if defined(__NEED_ssize_t) && !defined(__DEFINED_ssize_t)
typedef _addr ssize_t;
#define __DEFINED_ssize_t
#endif

#if defined(__NEED_int32_t) && !defined(__DEFINED_int32_t)
typedef int int32_t;
#define __DEFINED_int32_t
#endif

#if defined(__NEED_off_t) && !defined(__DEFINED_off_t)
typedef _int64 off_t;
#define __DEFINED_off_t
#endif

#if defined(__NEED_uint8_t) && !defined(__DEFINED_uint8_t)
typedef unsigned char uint8_t;
#define __DEFINED_uint8_t
#endif

#if defined(__NEED_uint16_t) && !defined(__DEFINED_uint16_t)
typedef unsigned short uint16_t;
#define __DEFINED_uint16_t
#endif

#if defined(__NEED_uint32_t) && !defined(__DEFINED_uint32_t)
typedef unsigned int uint32_t;
#define __DEFINED_uint32_t
#endif

#if defined(__NEED_uint64_t) && !defined(__DEFINED_uint64_t)
typedef unsigned _int64 uint64_t;
#define __DEFINED_uint64_t
#endif

#if defined(__NEED_uintptr_t) && !defined(__DEFINED_uintptr_t)
typedef unsigned _addr uintptr_t;
#define __DEFINED_uintptr_t
#endif

#if defined(__NEED_va_list) && !defined(__DEFINED_va_list)
typedef __builtin_va_list va_list;
#define __DEFINED_va_list
#endif

#if defined(__NEED_FILE) && !defined(__DEFINED_FILE)
typedef void* FILE;
#define __DEFINED_FILE
#endif

#if defined(__NEED_clockid_t) && !defined(__DEFINED_clockid_t)
typedef int clockid_t;
#define __DEFINED_clockid_t
#endif

#if defined(__NEED_time_t) && !defined(__DEFINED_time_t)
typedef _time time_t;
#define __DEFINED_time_t
#endif

#if defined(__NEED_struct_timespec) && !defined(__DEFINED_struct_timespec)
struct timespec {
  time_t tv_sec;
  long tv_nsec;
};
#define __DEFINED_struct_timespec
#endif

#if defined(__NEED_pid_t) && !defined(__DEFINED_pid_t)
typedef int pid_t;
#define __DEFINED_pid_t
#endif

#if defined(__NEED_ptrdiff_t) && !defined(__DEFINED_ptrdiff_t)
typedef _addr ptrdiff_t;
#define __DEFINED_ptrdiff_t
#endif
