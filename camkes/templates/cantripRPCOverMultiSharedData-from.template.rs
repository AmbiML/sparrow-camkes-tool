/*#
 *#Copyright 2020, Data61, CSIRO (ABN 41 687 119 230)
 *#
 *#SPDX-License-Identifier: BSD-2-Clause
  #*/

/*- set suffix = "_buf" -*/

/*# Assign client ids and badges #*/
/*- set badges = namespace() -*/
/*- if client_ids is not undefined -*/
    /*- set badges.badges = client_ids.badges -*/
/*- else -*/
    /*- from 'rpc-connector.c' import allocate_badges with context -*/
    /*- do allocate_badges(badges) -*/
/*- endif -*/
/*- set client_id = badges.badges[me.parent.from_ends.index(me)] -*/

/*- if suffix is not defined -*/
  /*- set suffix = '' -*/
/*- endif -*/

/*- set shmem_size = configuration[me.instance.name].get("%s_shmem_size" % me.interface.name, 4096) -*/
/*- set shmem_section = '%s' % me.interface.name -*/
/*- set shmem_symbol = '%s_INTERFACE_DATA' % me.interface.name.upper() -*/
/*- set shmem_name = '%s%s' % (me.interface.name, suffix) -*/
/*- set page_size = macros.get_page_size(shmem_size, options.architecture) -*/
/*- if page_size == 0 -*/
  /*? raise(TemplateError('Setting %s.%s_shmem_size does not meet minimum size and alignment requirements. %d must be at least %d and %d aligned' % (me.instance.name, me.interface.name, size, 4096, 4096))) ?*/
/*- endif -*/

/*? macros.shared_buffer_symbol(sym=shmem_symbol, shmem_size=shmem_size, page_size=page_size, language='rust') ?*/
#[no_mangle]
pub fn /*? me.interface.name ?*/_interface_shared_buffer_mut() -> &'static mut [u8] {
    unsafe { &mut /*? shmem_symbol ?*/.data[..] }
}
#[no_mangle]
pub fn /*? me.interface.name ?*/_interface_shared_buffer() -> &'static [u8] {
    unsafe { &/*? shmem_symbol ?*/.data[..] }
}
/*? register_shared_variable('%s_%s_data' % (me.parent.name, client_id), shmem_symbol, shmem_size, frame_size=page_size, language='rust') ?*/

/*- from 'rpc-connector.c' import establish_from_rpc with context -*/

/*- set connector = namespace() -*/
/*- set shmem_size = configuration[me.instance.name].get("%s_shmem_size" % me.interface.name, 4096) -*/

/*- set lock = True -*/
/*? establish_from_rpc(connector, buffer=('((void*)%s%s)' % (me.interface.name, suffix), shmem_size, lock), language='rust') ?*/

/*- set prefix = me.interface.name.upper() -*/
#[no_mangle]
pub static /*? prefix ?*/_INTERFACE_ENDPOINT: sel4_sys::seL4_CPtr = /*? connector.ep ?*/;
