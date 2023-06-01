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

#ifndef _STDDEF_H
#define _STDDEF_H

#ifndef NULL
#define NULL ((void*)0)
#endif

#define __NEED_ptrdiff_t
#define __NEED_size_t
#include <_types.h>

#define offsetof(type, member) __builtin_offsetof(type, member)

#endif
