/*
 * Copyright 2017, Data61, CSIRO (ABN 41 687 119 230)
 *
 * SPDX-License-Identifier: BSD-2-Clause
 */

/*- set index = me.parent.from_ends.index(me) -*/

/*- set paddr = configuration[me.parent.to_instance.name].get('%s_paddr' % me.parent.to_interface.name) -*/
/*- if paddr is none -*/
  /*? raise(TemplateError('Setting %s.%s_paddr that should specify the physical address of an MMIO device is not set' % (me.parent.to_instance.name, me.parent.to_interface.name))) ?*/
/*- endif -*/
/*- if not isinstance(paddr, numbers.Integral) or paddr < 0 -*/
  /*? raise(TemplateError('Setting %s.%s_paddr that should specify the physical address of an MMIO device does not appear to be a valid address' % (me.parent.to_instance.name, me.parent.to_interface.name))) ?*/
/*- endif -*/

/*- set size = configuration[me.parent.to_instance.name].get('%s_size' % me.parent.to_interface.name) -*/
/*- if size is none -*/
  /*? raise(TemplateError('Setting %s.%s_size that should specify the size of an MMIO device is not set' % (me.parent.to_instance.name, me.parent.to_interface.name))) ?*/
/*- endif -*/
/*- if not isinstance(size, numbers.Integral) or size <= 0 -*/
  /*? raise(TemplateError('Setting %s.%s_size that should specify the size of an MMIO device does not appear to be a valid size' % (me.parent.to_instance.name, me.parent.to_interface.name))) ?*/
/*- endif -*/
/*- set size = max(size, 4096) -*/
/*- set page_size = macros.get_page_size(size, options.architecture) -*/
/*- if page_size == 0 -*/
  /*? raise(TemplateError('Setting %s.%s_size must be page-aligned' % (me.parent.to_instance.name, me.parent.to_interface.name))) ?*/
/*- endif -*/
/*- set page_size_bits = int(math.log(page_size, 2)) -*/

/*- set cached = configuration[me.parent.to_instance.name].get('%s_hardware_cached' % me.parent.to_interface.name, False) -*/

/*- set dataport_symbol_name = "%s" % me.interface.name.upper() -*/
/*- set dataport_symbol_size = "%s_SIZE" % me.interface.name.upper() -*/
const /*? dataport_symbol_size ?*/: usize = round_up(/*? macros.dataport_size(me.interface.type, language='rust') ?*/, /*? page_size ?*/);
#[allow(clippy::upper_case_acronyms)]
#[repr(C, align(/*? page_size ?*/))]
pub struct /*? dataport_symbol_name ?*/ {
    pub data: [u8; /*? dataport_symbol_size ?*/],
}
#[link_section = "align_/*? page_size_bits ?*/bit"]
#[no_mangle]
pub static mut /*? dataport_symbol_name ?*/: /*? dataport_symbol_name ?*/ = /*? dataport_symbol_name ?*/ { data: [0u8; /*? dataport_symbol_size?*/] } ;
#[no_mangle]
pub fn get_/*? me.interface.name.lower() ?*/_mut() -> &'static mut [u8] {
    unsafe { &mut /*? dataport_symbol_name ?*/.data[..] }
}
#[no_mangle]
pub fn get_/*? me.interface.name.lower() ?*/() -> &'static [u8] {
    unsafe { &/*? dataport_symbol_name ?*/.data[..] }
}

/*- set frame_caps = [] -*/
/*? register_shared_variable('%s_DATA' % me.parent.name, dataport_symbol_name, size, frame_size=page_size, perm='RW', paddr=paddr, cached=cached, with_mapping_caps=frame_caps, language='rust') ?*/
// Frame caps for operations on underlying pages.
pub const /*? dataport_symbol_name ?*/_CAPS: [sel4_sys::seL4_CPtr; /*? len(frame_caps) ?*/] = [
/*- for cap in frame_caps -*/
   /*? cap ?*/ as sel4_sys::seL4_CPtr,
/*- endfor -*/
];
#[no_mangle]
pub fn get_/*? me.interface.name.lower() ?*/_caps() -> &'static [sel4_sys::seL4_CPtr] {
    &/*? dataport_symbol_name ?*/_CAPS[..]
}
