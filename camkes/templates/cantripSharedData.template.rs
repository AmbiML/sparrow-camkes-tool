/*
 * Copyright 2017, Data61, CSIRO (ABN 41 687 119 230)
 *
 * SPDX-License-Identifier: BSD-2-Clause
 */

/*- if me in me.parent.from_ends -*/
  /*- set index = me.parent.from_ends.index(me) -*/
  /*- set end = 'from' -*/
/*- elif me in me.parent.to_ends -*/
  /*- set index = me.parent.to_ends.index(me) -*/
  /*- set end = 'to' -*/
/*- endif -*/

/*- set dataport_symbol_name = "%s" % me.interface.name.upper() -*/
/*- set type_size = macros.dataport_size(me.interface.type, language='rust') -*/
/*- if type_size.startswith("core::") -*/
   /*- set size = configuration[me.parent.name].get('size', 4096) -*/
   /*- set page_size = macros.get_page_size(size, options.architecture) -*/
   /*- if page_size == 0 -*/
     /*? raise(TemplateError('Setting %s.size does not meet minimum size requirements. %d must be at least %d and %d aligned' % (me.parent.name, int(size), 4096, 4096))) ?*/
   /*- endif -*/
/*- else -*/
   /*- set size = type_size -*/
   /*- set page_size = macros.get_page_size(size, options.architecture) -*/
   /*- if page_size == 0 -*/
     /*? raise(TemplateError('Setting Buf(%d) does not meet minimum size requirements. %d must be at least %d and %d aligned' % (int(size), int(size), 4096, 4096))) ?*/
   /*- endif -*/
/*- endif -*/

/*- set shmem_symbol_size = str(size) -*/
/*? macros.shared_buffer_symbol(sym=dataport_symbol_name, shmem_size=shmem_symbol_size, page_size=page_size, language='rust') ?*/
#[no_mangle]
pub fn get_/*? me.interface.name.lower() ?*/_mut() -> &'static mut [u8] {
    unsafe { &mut /*? dataport_symbol_name ?*/.data[..] }
}
#[no_mangle]
pub fn get_/*? me.interface.name.lower() ?*/() -> &'static [u8] {
    unsafe { &/*? dataport_symbol_name ?*/.data[..] }
}
/*- set perm = macros.get_perm(configuration, me.instance.name, me.interface.name) -*/
/*? register_shared_variable(me.parent.name, dataport_symbol_name, size, frame_size=page_size, perm=perm, language='rust') ?*/
