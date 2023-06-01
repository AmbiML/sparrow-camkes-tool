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

#undef assert

#ifdef NDEBUG
#define assert(x) (void)0
#else
#define assert(x) \
  ((void)((x) || (__assert_fail(#x, __FILE__, __LINE__, __func__), 0)))
#endif

#define static_assert _Static_assert
_Noreturn void __assert_fail(const char *, const char *, int, const char *);
