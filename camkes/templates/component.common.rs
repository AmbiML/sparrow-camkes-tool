/*#
 * Copyright 2017, Data61, CSIRO (ABN 41 687 119 230)
 *
 * SPDX-License-Identifier: BSD-2-Clause
 #*/

/*#
 *# CantripOS support.
 #*/
use crate::camkes::*;
use crate::camkes::baresema::*;
#[allow(unused_imports)]
use crate::camkes::irq::*;
#[allow(unused_imports)]
use crate::camkes::semaphore::*;
use crate::camkes::startup::*;

const fn round_up(a: usize, b: usize) -> usize { ((a + b - 1) / b) * b }

/*? assert(configuration[me.name].get('cantripos')) ?*/

/*# TODO(sleffler): make more of these conditional? #*/
/*- set self_cnode = alloc_cap('cnode', my_cnode, write=true) -*/
#[no_mangle]
pub static SELF_CNODE: sel4_sys::seL4_CPtr = /*? self_cnode ?*/;
/*- set self_pd = alloc_cap('pd', my_pd, write=true) -*/
#[no_mangle]
pub static SELF_VSPACE_ROOT: sel4_sys::seL4_CPtr = /*? self_pd ?*/;
/*- set recv_cnode_size_bits = 8 -*/
/*- set recv_cnode = alloc('recv_cnode', seL4_CapTableObject, size_bits=recv_cnode_size_bits) -*/
#[no_mangle]
pub static MEMORY_RECV_CNODE: sel4_sys::seL4_CPtr = /*? recv_cnode ?*/;
#[no_mangle]
pub static MEMORY_RECV_CNODE_DEPTH: u8 = /*? recv_cnode_size_bits ?*/;

/*# Hand-off caps to ProcessManager for constructing applications. #*/
/*# Beware the rootserver has the location of these caps builtin. #*/
/*- if configuration[me.name].get('asid_pool', False) -*/
    /*- set asid_pool = alloc('asid_pool', type=seL4_ASID_Pool) -*/
    #[no_mangle]
    pub static ASID_POOL: sel4_sys::seL4_CPtr = /*? asid_pool ?*/;
/*- endif -*/
/*- if configuration[me.name].get('domain_ctrl', False) -*/
    /*- set domain_ctrl = alloc('domain', type=seL4_DomainControl, core=configuration[me.name].get('domain_ctrl')) -*/
    #[no_mangle]
    pub static DOMAIN_CTRL: sel4_sys::seL4_CPtr = /*? domain_ctrl ?*/;
/*- endif -*/
/*- if options.realtime -*/
/*- if configuration[me.name].get('sched_ctrl', False) -*/
    /*- set sched_ctrl = alloc('sched_control', type=seL4_SchedControl, core=configuration[me.name].get('sched_ctrl')) -*/
    #[no_mangle]
    pub static SCHED_CTRL: sel4_sys::seL4_CPtr = /*? sched_ctrl ?*/;
/*- endif -*/
/*- endif -*/

/*# Arrange to receive the BootInfo frame that comes with the UntypedMemory caps. #*/
/*- if configuration[me.name].get('untyped_memory', False) -*/
    /*# Setup a page for the loader to copy the BootInfo into #*/
    /*- set page_size = macros.get_page_size(4096, options.architecture) -*/
    /*- set page_size_bits = int(math.log(page_size, 2)) -*/
    /*- set bootinfo_symbol = 'BOOTINFO_FRAME' -*/
    /*- set bootinfo_struct = bootinfo_symbol.lower() -*/
    #[repr(C, align(/*? page_size ?*/))]
    pub struct /*? bootinfo_struct ?*/ { pub data: [u8; /*? page_size ?*/], }
    #[link_section = "align_/*? page_size_bits ?*/bit"]
    #[no_mangle]
    pub static /*? bootinfo_symbol ?*/: /*? bootinfo_struct ?*/ = /*? bootinfo_struct ?*/ { data: [0u8; /*? page_size ?*/] } ;
    #[no_mangle]
    pub fn get_bootinfo() -> &'static sel4_sys::seL4_BootInfo {
        unsafe { &*(core::ptr::addr_of!(/*? bootinfo_symbol ?*/.data[0]) as *const sel4_sys::seL4_BootInfo) }
    }
    /*? register_shared_variable(bootinfo_symbol, bootinfo_symbol, page_size, frame_size=page_size, perm='R', language='rust') ?*/
    /*- do register_fill_frame(bootinfo_symbol, 'CDL_FrameFill_BootInfo CDL_FrameFill_BootInfo_BootInfo', page_size) -*/
    /*- do my_cnode.__setattr__('has_untyped_memory', True) -*/
    /*- do my_cnode.__setattr__('headroom', my_cnode['headroom'] + 80) -*/
/*- endif -*/

/*- set copy_region_caps = [] -*/
/*- for c in me.type.copyregions -*/
    /*# Setup any per-component copy regions. The pre_init (or similar) routine
      * is expected to unmap the pages using copy_region_caps. This leaves each
      * COPY_REGION as a "hole" in the component's VSpace where it can map page
      * frames. Users are required to implement any necessary synchronization. #*/
    /*- set copy_region_size = c.size -*/
    /*- set page_size = macros.get_page_size(copy_region_size, options.architecture) -*/
    /*- if page_size == 0 -*/
      /*? raise(TemplateError('Invalid %s copy_region size %d: must be a multiple of %d' % (c.name, copy_region_size, 4096))) ?*/
    /*- endif -*/
    /*- set copy_region_symbol = c.name -*/
    /*- set copy_region_struct = copy_region_symbol.lower() -*/
    #[repr(C, align(/*? page_size ?*/))]
    pub struct /*? copy_region_struct ?*/ { pub data: [u8; /*? page_size ?*/], }
    #[link_section = "align_12bit"]
    #[no_mangle]
    pub static mut /*? copy_region_symbol ?*/: /*? copy_region_struct ?*/ = /*? copy_region_struct ?*/ {
      data: [0u8; round_up(/*? copy_region_size ?*/, /*? page_size ?*/)],
    };
    #[no_mangle]
    pub fn get_/*? copy_region_struct ?*/_mut() -> &'static mut [u8] {
        unsafe { &mut /*? copy_region_symbol ?*/.data[..] }
    }
    #[no_mangle]
    pub fn get_/*? copy_region_struct ?*/() -> &'static [u8] {
        unsafe { &/*? copy_region_symbol ?*/.data[..] }
    }
    /*- do register_copyregion_symbol(copy_region_symbol, copy_region_size) -*/
/*- endfor -*/

/*- set cnode_size = configuration[me.address_space].get('cnode_size_bits') -*/
/*- if cnode_size -*/
        /*- if isinstance(cnode_size, six.string_types) -*/
            /*- set size = int(cnode_size, 0) -*/
        /*- else -*/
            /*- set size = cnode_size -*/
        /*- endif -*/
    /*- do my_cnode.__setattr__('size_bits', size) -*/
/*- endif -*/

// seL4-based mutex's
/*- for m in me.type.mutexes -*/
    /*- set notification = alloc(m.name, seL4_NotificationObject, read=True, write=True) -*/
pub static /*? m.name.upper() ?*/: sel4_sys::seL4_CPtr = /*? notification ?*/;
/*- endfor -*/

// seL4-based semaphores
/*- for s in me.type.semaphores -*/
    /*# XXX remove any gratuitous _semaphore added elsewhere #*/
    /*- set name = s.name.replace('semaphore_semaphore', 'semaphore') -*/
    /*- set endpoint = alloc(name, seL4_EndpointObject, read=True, write=True) -*/
const /*? name.upper() ?*/_ENDPOINT: sel4_sys::seL4_CPtr = /*? endpoint ?*/;
static_semaphore!(/*? name ?*/);
/*- endfor -*/

// Dataports
#[allow(dead_code)]
pub type Buf = [u8; 4096];
/*- for dataport in me.type.dataports -*/
  /*- set include_code = macros.include_code(composition, me, dataport) -*/
  /*- if include_code -*/
    include!("/*? include_code ?*/.rs"); // /*? dataport.name ?*/
  /*- endif -*/
/*- endfor -*/

// IRQ's
/*- for irq in me.type.consumes -*/
  /*- set include_code = macros.include_code(composition, me, irq) -*/
  /*- if include_code -*/
    include!("/*? include_code ?*/.rs"); // /*? irq.name ?*/
  /*- endif -*/
/*- endfor -*/

/* seL4-based binary semaphores */
/*- for b in me.type.binary_semaphores -*/
    /*- set notification = alloc(b.name, seL4_NotificationObject, read=True, write=True) -*/
    /*- set initial = configuration[me.name].get('%s_value' % b.name, 0) -*/
    /*? assert(initial in (0, 1), "Expected 0 or 1 as initial value for binary semaphore \"%s\". Got %d." % (b.name, initial)) ?*/
#[no_mangle]
pub static /*? b.name.upper() ?*/: sel4_sys::seL4_CPtr = /*? notification ?*/;
#[no_mangle]
pub static /*? "%s_INITIAL" % b,name.upper() ?*/: usize = /*? initial ?*/;
/*- endfor -*/

// RPC connections
/*- for p in me.type.provides -*/
  /*- set include_code = macros.include_code(composition, me, p) -*/
  /*- if include_code -*/
    include!("/*? include_code ?*/.rs"); // PROVIDES /*? p.name ?*/
  /*- endif -*/
/*- endfor -*/
/*- for p in me.type.uses -*/
  /*- set include_code = macros.include_code(composition, me, p) -*/
  /*- if include_code -*/
    include!("/*? include_code ?*/.rs"); // USES /*? p.name ?*/
  /*- endif -*/
/*- endfor -*/
/*- for p in me.type.emits -*/
  /*- set include_code = macros.include_code(composition, me, p) -*/
  /*- if include_code -*/
    include!("/*? include_code ?*/.rs"); // EMITS /*? p.name ?*/
  /*- endif -*/
/*- endfor -*/

/*- set heap_size = configuration[me.name].get('heap_size', 'CONFIG_CAMKES_DEFAULT_HEAP_SIZE') -*/

/*- set threads = macros.threads(composition, me, configuration[me.name], options) -*/

/*- if options.debug_fault_handlers -*/
  /*- set fault_ep = alloc_obj('fault_ep', seL4_EndpointObject) -*/
/*- endif -*/

/*- if options.realtime -*/
    /*# Dict mapping thread prefixes to tcb caps #*/
    /*- set passive_tcbs = {} -*/

    /*# Set of Thread objects corresponding to passive threads #*/
    /*- set passive_threads = set() -*/
/*- endif -*/

/*- set thread_names = dict() -*/
/*- for t in threads -*/
    /*? macros.thread_stack(t.stack_symbol, t.stack_size, language='rust') ?*/
    /*- do register_stack_symbol(t.stack_symbol, t.stack_size) -*/

    /*? macros.ipc_buffer(t.ipc_symbol, language='rust') ?*/
    /*- set ipc_frame = alloc_obj('frame_%s' % (t.ipc_symbol), seL4_FrameObject) -*/
    /*- do register_ipc_symbol(t.ipc_symbol, ipc_frame) -*/

    /*- set _tcb = alloc_obj("%s_tcb" % t.name, seL4_TCBObject) -*/
    /*- set tcb = alloc_cap("%s_tcb" % t.name, _tcb) -*/
    /*- do _tcb.__setitem__('ipc_buffer_slot', Cap(ipc_frame, read=True, write=True)) -*/
    /*- do _tcb.__setitem__('vspace', Cap(my_pd)) -*/
    /*- set cnode_cap = Cap(my_cnode) -*/
    /*- do my_cnode.update_guard_size_caps.append(cnode_cap) -*/
    /*- do _tcb.__setitem__('cspace', cnode_cap) -*/

    /*- do _tcb.__setattr__('ip', "get_vaddr(\'%s\')" % ("camkes %s _camkes_start" % me.name) ) -*/
    /*- do _tcb.__setattr__('sp', t.sp) -*/
    /*- do _tcb.__setattr__('addr', t.addr) -*/
    /*- do _tcb.init.append(tcb) -*/

    /*- if options.realtime -*/
        /*- if not t.interface or not configuration[me.name].get("%s_passive" % t.interface.name, False) -*/
            /*- set _sc = alloc_obj("%s_sc" % t.name, seL4_SchedContextObject) -*/
            /*- set sc = alloc_cap("%s_sc" % t.name, _sc) -*/
            /*- do _tcb.__setitem__('sc_slot', Cap(_sc)) -*/
            /*- if loop.first -*/
                /*- do macros.set_sc_properties(_sc, options, configuration[me.name], "_") -*/
            /*- elif options.debug_fault_handlers and loop.last -*/
                /*- do macros.set_sc_properties(_sc, options, configuration[me.name], "fault") -*/
            /*- else -*/
                /*- do macros.set_sc_properties(_sc, options, configuration[me.name], "%s_" % t.interface.name) -*/
            /*- endif -*/
        /*- else -*/
            /*# This branch is for interface threads that are passive #*/
            /*- do passive_tcbs.__setitem__(t.name, tcb) -*/
            /*- do passive_threads.add(t) -*/
        /*- endif -*/
    /*- endif -*/

    /*- if options.debug_fault_handlers and not loop.last -*/
        /*- if not options.realtime -*/
            /*- set fault_ep_cap = alloc_cap('fault_ep_%s' % t.name, fault_ep, read=True, write=True, grantreply=True, badge=tcb) -*/
            /*- do setattr(_tcb, 'fault_ep_slot', fault_ep_cap) -*/
        /*- endif -*/

        /*- if options.realtime -*/
            /*- do _tcb.set_fault_ep_slot(fault_ep=fault_ep.name, badge=tcb) -*/
        /*- endif -*/

    /*- endif -*/

    /*- if loop.first -*/
        /*- do macros.set_tcb_properties(_tcb, options, configuration[me.name], "_") -*/
        /*- do thread_names.__setitem__(tcb, "control") -*/
    /*- elif options.debug_fault_handlers and loop.last -*/
        /*- do thread_names.__setitem__(tcb, "fault_handler") -*/
        /*- do _tcb.__setattr__('prio', 255) -*/
        /*- do _tcb.__setattr__('affinity', options.default_affinity) -*/
        /*- do _tcb.__setattr__('max_prio', options.default_max_priority) -*/

    /*- else -*/
        /*- do thread_names.__setitem__(tcb, t.interface.name) -*/
        /*- do macros.set_tcb_properties(_tcb, options, configuration[me.name], "%s_" % t.interface.name) -*/
    /*- endif -*/


/*- endfor -*/

/*- if options.realtime and passive_threads -*/
    /*# SC to use to initialise all passive interfaces of this instance #*/
    /*- set sc_passive_init = alloc('%d_0_control_%d_passive_init_sc' % (len(me.name), len('0_control')), seL4_SchedContextObject) -*/

    /*# Ntfn to use in passive init protocol #*/
    /*- set ntfn_passive_init = alloc('%d_0_control_%d_passive_init_ntfn' % (len(me.name), len('0_control')), seL4_NotificationObject, read=True, write=True) -*/
/*- endif -*/

/*- for tcb, name in thread_names.items() -*/
const SELF_TCB_/*? name.upper() ?*/ : sel4_sys::seL4_CPtr = /*? tcb ?*/;
#[no_mangle]
pub static TCB_/*? name.upper() ?*/ : sel4_sys::seL4_CPtr = /*? tcb ?*/;
/*- endfor -*/

/*- for t in threads -*/
/*- if loop.first -*/
static_control_thread!(
    /*name=*/ /*? me.name ?*/,
    /*tcb=*/ SELF_TCB_CONTROL,
    /*ipc_buffer=*/ core::ptr::addr_of!(/*? macros.ipc_buffer_address(t.ipc_symbol, language='rust') ?*/),
    &CAMKES
);
/*- elif options.debug_fault_handlers and loop.last -*/
static_fault_handler_thread!(
    /*name=*/ /*? me.name ?*/,
    /*tcb=*/ SELF_TCB_FAULT_HANDLER,
    /*ipc_buffer=*/ core::ptr::addr_of!(/*? macros.ipc_buffer_address(t.ipc_symbol, language='rust') ?*/),
    &CAMKES
);
/*- else -*/
/*- if macros.interface_is_irq(t.interface, me) -*/
static_irq_thread!(
    /*name=*/ /*? t.interface.name ?*/,
    /*tcb=*/ SELF_TCB_/*? t.interface.name.upper() ?*/,
    /*ipc_buffer=*/ core::ptr::addr_of!(/*? macros.ipc_buffer_address(t.ipc_symbol, language='rust') ?*/),
    &CAMKES
);
/*- else -*/
/*# TODO: passive interface support #*/
static_interface_thread!(
    /*name=*/ /*? t.interface.name ?*/,
    /*tcb=*/ SELF_TCB_/*? t.interface.name.upper() ?*/,
    /*ipc_buffer=*/ core::ptr::addr_of!(/*? macros.ipc_buffer_address(t.ipc_symbol, language='rust') ?*/),
    &CAMKES
);
/*- endif -*/
/*- endif -*/
/*- endfor -*/

// Locks for synchronising startup. These are used during thread startup.
/*- set pre_init_ep = alloc('pre_init_ep', seL4_EndpointObject, read=True, write=True) -*/
const PRE_INIT_ENDPOINT: sel4_sys::seL4_CPtr = /*? pre_init_ep ?*/;
static_bare_sema!(PRE_INIT);
/*- set interface_init_ep = alloc('interface_init_ep', seL4_EndpointObject, read=True, write=True) -*/
const INTERFACE_INIT_ENDPOINT: sel4_sys::seL4_CPtr = /*? interface_init_ep ?*/;
static_bare_sema!(INTERFACE_INIT);
/*- set post_init_ep = alloc('post_init_ep', seL4_EndpointObject, read=True, write=True) -*/
const POST_INIT_ENDPOINT: sel4_sys::seL4_CPtr = /*? post_init_ep ?*/;
static_bare_sema!(POST_INIT);

#[no_mangle]
pub static CAMKES: Camkes = Camkes::new(
    "/*? me.name ?*/",
    &PRE_INIT,
    &POST_INIT,
    &INTERFACE_INIT,
    &[
        &/*? me.name.upper() ?*/_THREAD,
/*- for t in threads[1:] -*/
/*- if options.debug_fault_handlers and loop.last -*/
        &FAULT_HANDLER_THREAD,
/*- else -*/
        &/*? t.interface.name.upper() ?*/_THREAD,
/*- endif -*/
/*- endfor -*/
    ],
);

// Start thread using thread id. Called from crt0:_camkes_start.
#[no_mangle]
pub fn _camkes_start_rust(thread_id: sel4_sys::seL4_CPtr) -> ! {
    let thread = CAMKES.thread(thread_id).unwrap();
    match thread_id {
/*- for t in threads -*/
/*- if loop.first -*/
        SELF_TCB_CONTROL => crate::/*? me.name.title().replace('_', '') ?*/ControlThread::start(thread),
/*- elif options.debug_fault_handlers and loop.last -*/
        SELF_TCB_FAULT_HANDLER => crate::/*? me.name.title().replace('_', '') ?*/FaultHandlerThread::start(thread),
/*- else -*/
        SELF_TCB_/*? t.interface.name.upper() ?*/ => crate::/*? t.interface.name.title().replace('_', '') ?*/InterfaceThread::start(thread),
/*- endif -*/
/*- endfor -*/
        _ => unreachable!(),
    }
    // ?unreachable!();
}

include!("camkes.fini.rs");
