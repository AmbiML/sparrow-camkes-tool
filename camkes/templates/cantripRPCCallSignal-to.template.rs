/*#
 *#Copyright 2019, Data61, CSIRO (ABN 41 687 119 230)
 *#
 *#SPDX-License-Identifier: BSD-2-Clause
  #*/

/*# Assign client ids and badges #*/
/*- from 'rpc-connector.c' import allocate_badges with context -*/
/*- from 'global-endpoint.template.c' import allocate_cap with context -*/
/*- set client_ids = namespace() -*/
/*- do allocate_badges(client_ids) -*/

/*- set badges = client_ids.badges -*/

#[no_mangle]
pub fn /*? me.interface.name ?*/_emit(
    badge: sel4_sys::seL4_Word,
) {
    match badge {
    /*- for (i, c) in enumerate(me.parent.from_ends) -*/
        /*- do allocate_cap(c, is_reader=False) -*/
        /*- set notification = pop('notification') -*/
        /*? badges[i] ?*/ => unsafe { sel4_sys::seL4_Signal(/*? notification ?*/) },
    /*- endfor -*/
        _ => unreachable!(),
    }
}

/*- from 'rpc-connector.c' import establish_recv_rpc with context -*/

/*- set connector = namespace() -*/

/*? establish_recv_rpc(connector, me.interface.name, language='rust') ?*/

/*- set prefix = me.interface.name.upper() -*/
#[no_mangle]
pub static /*? prefix ?*/_INTERFACE_ENDPOINT: sel4_sys::seL4_CPtr = /*? connector.ep ?*/;
#[no_mangle]
pub static /*? prefix ?*/_INTERFACE_REPLY: sel4_sys::seL4_CPtr = /*? connector.reply_cap_slot ?*/;

