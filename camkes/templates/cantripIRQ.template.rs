/*
 * Copyright 2017, Data61, CSIRO (ABN 41 687 119 230)
 *
 * SPDX-License-Identifier: BSD-2-Clause
 */

/*# NB: only simple & arm types are "tested" #*/

/*- macro check_setting(v, attr, expect) -*/
    /*- if v is none -*/
        /*? raise(TemplateError('Setting %s.%s is not defined; expecting %s' % (from_instance.name, attr, expect))) ?*/
    /*- endif -*/
    /*- if not isinstance(v, numbers.Integral) -*/
        /*? raise(TemplateError('Setting %s.%s is not an integer; expecting %s' % (from_instance.name, attr, expect))) ?*/
    /*- endif -*/
/*- endmacro -*/

/*- macro register_irq(configuration, from_end, irq_prefix, ntfn) -*/
    /*- set from_instance = from_end.instance -*/
    /*- set from_interface = from_end.interface -*/

    /*- set type_attr = '%s_irq_type' % from_interface.name -*/
    /*- set type = configuration[from_instance.name].get(type_attr, 'simple') -*/
    /*- if type == 'simple' -*/
        /*- set attr = '%s_irq_number' % from_interface.name -*/
        /*- set _irq = configuration[from_instance.name].get(attr) -*/
        /*- do check_setting(_irq, attr, 'IRQ number') -*/

        /*- set spi_attr = '%s_spi_number' % from_interface.name -*/
        /*- set _irq_spi = configuration[from_instance.name].get(spi_attr) -*/
        /*- if (isinstance(_irq_spi, numbers.Integral)) and (_irq_spi == 0) -*/
            /*- set _irq = _irq + 32 -*/
        /*- endif -*/

        /*- set irq = alloc(irq_prefix.lower(), seL4_IRQHandler, number=_irq, notification=ntfn) -*/
const /*? irq_prefix ?*/_NUMBER: usize = /*? _irq ?*/;
const /*? irq_prefix ?*/_HANDLER: sel4_sys::seL4_CPtr = /*? irq ?*/;
    /*- elif type in ['arm'] -*/
        /*- set attr = '%s_irq_trigger' % from_interface.name -*/
        /*- set trigger = configuration[from_instance.name].get(attr, "level") -*/
        /*- if trigger == "level" -*/
            /*- set trigger = seL4_ARM_IRQ_LEVEL -*/
            /*- set sel4_trigger_param = 0 -*/
        /*- elif trigger == "edge" -*/
            /*- set trigger = seL4_ARM_IRQ_EDGE -*/
            /*- set sel4_trigger_param = 1 -*/
        /*- else -*/
            /*? raise(TemplateError('Setting %s.%s that should specify an IRQ trigger mode can only be "edge" or "level" but is set to: %s' % (from_instance.name, attr, trigger))) ?*/
        /*- endif -*/

        /*- set attr = '%s_irq_number' % from_interface.name -*/
        /*- set _irq = configuration[from_instance.name].get(attr) -*/
        /*- do check_setting(_irq, attr, 'IRQ number') -*/

        /*- set attr = '%s_irq_target' % from_interface.name -*/
        /*- set target = configuration[from_instance.name].get(attr, 0) -*/
        /*- do check_setting(target, attr, 'target core') -*/

        /*- set irq = alloc(irq_prefix.lower(), seL4_IRQHandler, number=_irq, trigger=trigger, target=target, notification=ntfn) -*/
const /*? irq_prefix ?*/_TRIGGER: usize = /*? trigger ?*/;
const /*? irq_prefix ?*/_CPU_IDX: usize = /*? cpu_idx ?*/;
const /*? irq_prefix ?*/_NUMBER: usize = /*? _irq ?*/;
const /*? irq_prefix ?*/_HANDLER: sel4_sys::seL4_CPtr = /*? irq ?*/;
    /*- elif type in ['ioapic','isa','pci'] -*/
        /*- if type == 'isa' -*/
            /*- set level = 0 -*/
            /*- set polarity = 0 -*/
        /*- elif type == 'pci' -*/
            /*- set level = 1 -*/
            /*- set polarity = 1 -*/
        /*- else -*/
            /*- set attr = '%s_irq_level' % from_interface.name -*/
            /*- set level = configuration[from_instance.name].get(attr) -*/
            /*- do check_setting(level, attr, 'IOAPIC interrupt level') -*/

            /*- set attr = '%s_irq_polarity' % from_interface.name -*/
            /*- set polarity = configuration[from_instance.name].get(attr) -*/
            /*- do check_setting(polarity, attr, 'IOAPIC interrupt polarity') -*/
        /*- endif -*/

        /*- set attr = '%s_irq_ioapic' % from_interface.name -*/
        /*- set ioapic = configuration[from_instance.name].get(attr) -*/
        /*- do check_setting(ioapic, attr, 'IOAPIC controller number') -*/

        /*- set attr = '%s_irq_ioapic_pin' % from_interface.name -*/
        /*- set ioapic_pin = configuration[from_instance.name].get(attr) -*/
        /*- do check_setting(ioapic_pin, attr, 'IOAPIC pin number') -*/

        /*- set attr = '%s_irq_vector' % from_interface.name -*/
        /*- set vector = configuration[from_instance.name].get(attr) -*/
        /*- do check_setting(vector, attr, 'IRQ vector') -*/

        /*- set irq = alloc(irq_prefix.lower(), seL4_IRQHandler, vector=vector, ioapic=ioapic, ioapic_pin=ioapic_pin, level=level, polarity=polarity, notification=ntfn) -*/
const /*? irq_prefix ?*/_IOAPIC: usize = /*? ioapic ?*/;
const /*? irq_prefix ?*/_PIN: usize = /*? ioapic_pin ?*/;
const /*? irq_prefix ?*/_LEVEL: usize = /*? level ?*/;
const /*? irq_prefix ?*/_POLARITY: usize = /*? polarity ?*/;
const /*? irq_prefix ?*/_VECTOR: usize = /*? vector ?*/;
const /*? irq_prefix ?*/_HANDLER: sel4_sys::seL4_CPtr = /*? irq ?*/;
    /*- elif type == 'msi' -*/
        /*- set attr = '%s_irq_handle' % from_interface.name -*/
        /*- set handle = configuration[from_instance.name].get(attr) -*/
        /*- do check_setting(handle, attr, 'MSI handle') -*/

        /*- set attr = '%s_irq_pci_bus' % from_interface.name -*/
        /*- set pci_bus = configuration[from_instance.name].get(attr) -*/
        /*- do check_setting(pci_bus, attr, 'PCI bus') -*/

        /*- set attr = '%s_irq_pci_dev' % from_interface.name -*/
        /*- set pci_dev = configuration[from_instance.name].get(attr) -*/
        /*- do check_setting(pci_dev, attr, 'PCI device') -*/

        /*- set attr = '%s_irq_pci_fun' % from_interface.name -*/
        /*- set pci_fun = configuration[from_instance.name].get(attr) -*/
        /*- do check_setting(pci_fun, attr, 'PCI function') -*/

        /*- set attr = '%s_irq_vector' % from_interface.name -*/
        /*- set vector = configuration[from_instance.name].get(attr) -*/
        /*- do check_setting(vector, attr, 'IRQ vector') -*/

        /*- set irq = alloc(irq_prefix.lower(), seL4_IRQHandler, vector=vector, handle=handle, pci_bus=pci_bus, pci_dev=pci_dev, pci_fun=pci_fun, notification=ntfn) -*/
const /*? irq_prefix ?*/_BUS: usize = /*? pci_bus ?*/;
const /*? irq_prefix ?*/_DEV: usize = /*? pci_dev ?*/;
const /*? irq_prefix ?*/_FUNC: usize = {/*? pci_func ?*/;
const /*? irq_prefix ?*/_HANDLE: usize = /*? handle ?*/;
const /*? irq_prefix ?*/_VECTOR: usize = /*? vector ?*/;
const /*? irq_prefix ?*/_HANDLER: sel4_sys::seL4_CPtr = /*? irq ?*/;
    /*- else -*/
        /*? raise(TemplateError('Unknown irq type specified by %s.%s' % (from_instance.name, type_attr))) ?*/
    /*- endif -*/
/*- endmacro -*/

/*- set ntfn_prefix = me.interface.name.upper() -*/

/*- set ntfn_obj = alloc_obj('ntfn', seL4_NotificationObject) -*/
/*- set root_ntfn = alloc_cap('ntfn', ntfn_obj, read=True) -*/
const /*? ntfn_prefix ?*/_NOTIFICATION: sel4_sys::seL4_CPtr = /*? root_ntfn ?*/;

/*- for (i, end) in enumerate(me.parent.from_ends) -*/
    /*- set irq_prefix = end.interface.name.upper() -*/
    /*- set irq_badge = pow(2, i) -*/
    /*- set irq_ntfn = Cap(ntfn_obj, read=True, badge=irq_badge) -*/
    /*? register_irq(configuration, end, irq_prefix, irq_ntfn) ?*/
static_irq!(/*? irq_prefix.lower() ?*/, /*? irq_badge ?*/, /*? ntfn_prefix ?*/_NOTIFICATION);
/*- endfor -*/
