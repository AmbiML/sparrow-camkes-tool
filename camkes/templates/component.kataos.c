/*#
 *# Copyright 2017, Data61
 *# Commonwealth Scientific and Industrial Research Organisation (CSIRO)
 *# ABN 41 687 119 230.
 *#
 *# This software may be distributed and modified according to the terms of
 *# the BSD 2-Clause license. Note that NO WARRANTY is provided.
 *# See "LICENSE_BSD2.txt" for details.
 *#
 *# @TAG(DATA61_BSD)
 #*/

/*#
 *# KataOS support.
 #*/

#include <autoconf.h>
#include <sel4camkes/gen_config.h>
#include <assert.h>
#include <sel4/types.h>
#include <sel4/sel4.h>
#include <sel4utils/mapping.h>

/*? assert(configuration[me.name].get('kataos')) ?*/

/*- macro next_pow2(val) -*/
    /*? pow(2, val.bit_length()) ?*/
/*- endmacro -*/

/*# TODO(sleffler): make more of these conditional? #*/
/*- set self_cnode = alloc_cap('cnode', my_cnode, write=true) -*/
const seL4_CPtr SELF_CNODE = /*? self_cnode ?*/;
/*- set self_pd = alloc_cap('pd', my_pd, write=true) -*/
const seL4_CPtr SELF_VSPACE_ROOT = /*? self_pd ?*/;
/*- set recv_cnode_size_bits = 8 -*/
/*- set recv_cnode = alloc('recv_cnode', seL4_CapTableObject, size_bits=recv_cnode_size_bits) -*/
const seL4_CPtr MEMORY_RECV_CNODE = /*? recv_cnode ?*/;
const unsigned char MEMORY_RECV_CNODE_DEPTH = /*? recv_cnode_size_bits ?*/;

 /*- set threads = macros.threads(composition, me, configuration[me.name], options) -*/
 /*- for t in threads -*/
     /*- set tcb = alloc('%s_tcb' % t.name, seL4_TCBObject) -*/
     const seL4_CPtr SELF_TCB_/*? t.name.upper() ?*/ = /*? tcb ?*/;
 /*- endfor -*/


/*# Hand-off caps to ProcessManager for constructing applications. #*/
/*# Beware the rootserver has the location of these caps builtin. #*/
/*- if configuration[me.name].get('asid_pool', False) -*/
    /*- set asid_pool = alloc('asid_pool', type=seL4_ASID_Pool) -*/
    const seL4_CPtr ASID_POOL = /*? asid_pool ?*/;
/*- endif -*/
/*- if configuration[me.name].get('domain_ctrl', False) -*/
    /*- set domain_ctrl = alloc('domain', type=seL4_DomainControl, core=configuration[me.name].get('domain_ctrl')) -*/
    const seL4_CPtr DOMAIN_CTRL = /*? domain_ctrl ?*/;
/*- endif -*/
/*- if options.realtime -*/
/*- if configuration[me.name].get('sched_ctrl', False) -*/
    /*- set sched_ctrl = alloc('sched_control', type=seL4_SchedControl, core=configuration[me.name].get('sched_ctrl')) -*/
    const seL4_CPtr SCHED_CTRL = /*? sched_ctrl ?*/;
/*- endif -*/
/*- endif -*/

/*# Arrange to receive the BootInfo frame that comes with the UntypedMemory caps. #*/
/*- if configuration[me.name].get('untyped_memory', False) -*/
    /*# Setup a page for the loader to copy the BootInfo into #*/
    char bootinfo_frame[/*? 4096 ?*/]
    ALIGN(PAGE_SIZE_4K) SECTION("align_12bit");
    /*- do register_fill_frame('bootinfo_frame', 'CDL_FrameFill_BootInfo CDL_FrameFill_BootInfo_BootInfo', 4096) -*/
    /*- do my_cnode.__setattr__('has_untyped_memory', True) -*/
    /*- do my_cnode.__setattr__('headroom', my_cnode['headroom'] + 80) -*/
/*- endif -*/

/*- set cnode_headroom = configuration[me.address_space].get('cnode_headroom') -*/
/*- if cnode_headroom -*/
        /*- if isinstance(cnode_headroom, six.string_types) -*/
            /*- set headroom = int(cnode_headroom, 0) -*/
        /*- else -*/
            /*- set headroom = cnode_headroom -*/
        /*- endif -*/
    /*- do my_cnode.__setattr__('headroom', my_cnode['headroom'] + headroom) -*/
/*- endif -*/

/*# No cap allocation from here on! We assume all caps exist so we can guess our cnode size from the
 * holding slot #*/
/*- set holding_slot = alloc_cap('temporary_kataos_slot', None) -*/
const seL4_CPtr SELF_CNODE_FIRST_SLOT = /*? holding_slot ?*/;
/*- if cap_space.cnode.size_bits == 'auto' -*/
    /*- set size_bits = configuration[me.name].get('cnode_size_bits') -*/
    /*- if size_bits is not none -*/
        /*- set cnodesize = pow(2, size_bits) -*/
    /*- else -*/
        /*- set cnodesize = next_pow2(holding_slot + my_cnode['headroom']) -*/
    /*- endif -*/
/*- else -*/
    /*- set cnodesize = pow2(cap_space.cnode.size_bits) -*/
/*- endif -*/
const seL4_CPtr SELF_CNODE_LAST_SLOT = /*? cnodesize ?*/;
