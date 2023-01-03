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

#include <arch_stdio.h>
#include <stddef.h>
#include <utils/page.h>

write_buf_fn sel4muslcsys_register_stdio_write_fn(write_buf_fn write_fn) {
  /* NOP */
  (void)write_fn;
}

char __attribute__((aligned(PAGE_SIZE_4K))) morecore_area[0];
size_t morecore_size = 0;

void camkes_install_syscalls(void) { /* NOP */
}
