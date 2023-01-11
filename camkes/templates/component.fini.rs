/*#
 * Copyright 2017, Data61, CSIRO (ABN 41 687 119 230)
 *
 * SPDX-License-Identifier: BSD-2-Clause
 #*/

/*? assert(configuration[me.name].get('cantripos')) ?*/

/*- macro next_pow2(val) -*/
    /*? pow(2, val.bit_length()) ?*/
/*- endmacro -*/

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
/*- set holding_slot = alloc_cap('temporary_cantripos_slot', None) -*/
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
#[no_mangle]
pub static SELF_CNODE_FIRST_SLOT: sel4_sys::seL4_CPtr = /*? holding_slot ?*/;
#[no_mangle]
pub static SELF_CNODE_LAST_SLOT: sel4_sys::seL4_CPtr = /*? cnodesize ?*/;
