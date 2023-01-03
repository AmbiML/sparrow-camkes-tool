# Copyright 2022 Google LLC
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     https://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

cmake_minimum_required(VERSION 3.7.2)

project(libcantripcamkes C)

file(GLOB deps
	src/*.c
	../../util_libs/libutils/src/zf_log.c
	../../seL4_libs/libsel4debug/src/register_dump.c
)

# From libsel4simple/CMakeLists.txt
if(KernelArchRiscV)
    set(kernel_arch_simple arm)
else()
    set(kernel_arch_simple ${KernelArch})
endif()

if(Kernel64)
    if(${KernelArch} STREQUAL "arm")
        set(libc_arch "aarch64")
    endif()
    if(${KernelArch} STREQUAL "riscv")
        set(libc_arch "riscv")
    endif()
else()
    if(${KernelArch} STREQUAL "arm")
        set(libc_arch "arm")
    endif()
    if(${KernelArch} STREQUAL "riscv")
        set(libc_arch "riscv")
    endif()
endif()

add_library(cantripcamkes STATIC EXCLUDE_FROM_ALL ${deps})

target_include_directories(cantripcamkes
	PUBLIC
	"include"
	"arch_include/${libc_arch}"
	"../../seL4_libs/libsel4debug/include"
	"../../seL4_libs/libsel4debug/arch_include/${KernelArch}"
	"../../seL4_libs/libsel4debug/sel4_arch_include/${KernelSel4Arch}"
	"../../seL4_libs/libsel4muslcsys/include"
	"../../seL4_libs/libsel4platsupport/include"
	"../../seL4_libs/libsel4simple/include"
	"../../seL4_libs/libsel4simple/arch_include/${kernel_arch_simple}"
	"../../seL4_libs/libsel4sync/include"
	"../../seL4_libs/libsel4utils/include"
	"../../seL4_libs/libsel4vka/arch_include/${KernelArch}"
	"../../seL4_libs/libsel4vka/include"
	"../../seL4_libs/libsel4vka/sel4_arch_include/${KernelSel4Arch}"
	"../../seL4_libs/libsel4vspace/include"
	"../../seL4_libs/libsel4vspace/arch_include/${KernelArch}"
	"../../util_libs/libplatsupport/include"
	"../../util_libs/libplatsupport/arch_include/${KernelArch}"
	"../../util_libs/libplatsupport/plat_include/${KernelPlatform}"
	"../../util_libs/libutils/include"
	"../../util_libs/libutils/arch_include/${KernelArch}"
)

target_link_libraries(
	cantripcamkes
	sel4
	sel4muslcsys_Config
	sel4runtime
	sel4runtime_Config
	sel4utils_Config
	sel4vka_Config
	utils_Config
)
