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

#ifndef __ARCH_DEFS_H
#define __ARCH_DEFS_H

#define _addr int
#define _int64 long long
#define _time long long
#define UINTPTR_MAX UINT32_MAX
#define SIZE_MAX UINT32_MAX
#define UINT64_C(c) c##ULL
#define __PRI64 "ll"
#define __PRIPTR ""

#endif
