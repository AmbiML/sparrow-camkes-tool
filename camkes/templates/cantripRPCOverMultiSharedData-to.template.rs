/*#
 *#Copyright 2020, Data61, CSIRO (ABN 41 687 119 230)
 *#
 *#SPDX-License-Identifier: BSD-2-Clause
  #*/

/*# Assign client ids and badges #*/
/*- set badges = namespace() -*/
/*- if client_ids is not undefined -*/
    /*- set badges.badges = client_ids.badges -*/
/*- else -*/
    /*- from 'rpc-connector.c' import allocate_badges with context -*/
    /*- do allocate_badges(badges) -*/
/*- endif -*/
/*- set badges = badges.badges -*/

/*# Enumerate all the incoming interfaces #*/
/*- set shmems = [] -*/
/*- set client_ids = set() -*/
/*- for c in me.parent.from_ends -*/

    /*- set client_id = badges[loop.index0] -*/

    /*- if client_id not in client_ids -*/
        /*- do client_ids.add(client_id) -*/

        /*- set shmem_size = configuration[c.instance.name].get("%s_shmem_size" % c.interface.name, 4096) -*/
        /*- set shmem_section = '%s_%s' % (me.interface.name.upper(), client_id) -*/
        /*- set shmem_symbol = '%s_%s_DATA' % (me.interface.name.upper(), client_id) -*/
        /*- set shmem_name = "%s_buf_%s" % (me.interface.name, client_id) -*/
        /*- set page_size = macros.get_page_size(shmem_size, options.architecture) -*/
        /*- if page_size == 0 -*/
          /*? raise(TemplateError('Setting %s.%s_shmem_size does not meet minimum size and alignment requirements. %d must be at least %d and %d aligned' % (c.instance.name, c.interface.name, size, 4096, 4096))) ?*/
        /*- endif -*/
        /*- set page_size_bits = int(math.log(page_size, 2)) -*/

        /*? macros.shared_buffer_symbol(sym=shmem_symbol, shmem_size=shmem_size, page_size=page_size, language='rust') ?*/
        /*? register_shared_variable('%s_%s_data' % (me.parent.name, client_id), shmem_symbol, shmem_size, frame_size=page_size, language='rust') ?*/

        /*- do shmems.append((shmem_symbol, client_id, shmem_size)) -*/
    /*- else -*/
        /* skipping /*? client_id ?*/ */
    /*- endif -*/

/*- endfor -*/


pub fn /*? me.interface.name ?*/_interface_recv_buffer(
    badge: sel4_sys::seL4_Word,
) -> &'static mut [u8] {
    match badge {
    /*- for symbol, id, _ in shmems -*/
        /*? id ?*/ => unsafe { &mut /*? symbol.upper() ?*/.data[..] },
    /*- endfor -*/
        _ => unreachable!(),
    }
}


/*- from 'rpc-connector.c' import establish_recv_rpc with context -*/

/*- set connector = namespace() -*/

/*? establish_recv_rpc(connector, me.interface.name, buffer=('(%s_buf(%s_get_sender_id()))' % (me.interface.name,me.interface.name) , '(%s_buf_size(%s_get_sender_id()))' % (me.interface.name,me.interface.name)), language='rust') ?*/

/*- set prefix = me.interface.name.upper() -*/
#[no_mangle]
pub static /*? prefix ?*/_INTERFACE_ENDPOINT: sel4_sys::seL4_CPtr = /*? connector.ep ?*/;
#[no_mangle]
pub static /*? prefix ?*/_INTERFACE_REPLY: sel4_sys::seL4_CPtr = /*? connector.reply_cap_slot ?*/;
