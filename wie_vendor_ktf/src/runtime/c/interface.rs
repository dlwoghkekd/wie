use alloc::vec::Vec;
use core::mem::size_of;

use bytemuck::{Pod, Zeroable};

use wie_backend::Backend;
use wie_base::util::write_generic;
use wie_core_arm::ArmCore;
use wie_wipi_c::{get_database_method_table, get_graphics_method_table, get_kernel_method_table, get_media_method_table, CContext, CMethodBody};

use crate::runtime::c::context::KtfCContext;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct WIPICInterface {
    interface_0: u32,
    interface_1: u32,
    interface_2: u32,
    interface_3: u32,
    interface_4: u32,
    interface_5: u32,
    interface_6: u32,
    interface_7: u32,
    interface_8: u32,
    interface_9: u32,
    interface_10: u32,
    interface_11: u32,
    interface_12: u32,
}

fn write_methods(context: &mut dyn CContext, methods: Vec<CMethodBody>) -> anyhow::Result<u32> {
    let address = context.alloc_raw((methods.len() * 4) as u32)?;

    let mut cursor = address;
    for method in methods {
        let address = context.register_function(method)?;

        write_generic(context, cursor, address)?;
        cursor += 4;
    }

    Ok(address)
}

pub fn get_wipic_knl_interface(core: &mut ArmCore, backend: &mut Backend) -> anyhow::Result<u32> {
    let kernel_methods = get_kernel_method_table(get_wipic_interfaces);

    let mut context = KtfCContext::new(core, backend);
    let address = write_methods(&mut context, kernel_methods)?;

    Ok(address)
}

async fn get_wipic_interfaces(context: &mut dyn CContext) -> anyhow::Result<u32> {
    tracing::trace!("get_wipic_interfaces");

    let graphics_methods = get_graphics_method_table();
    let interface_2 = write_methods(context, graphics_methods)?;

    let database_methods = get_database_method_table();
    let interface_6 = write_methods(context, database_methods)?;

    let media_methods = get_media_method_table();
    let interface_9 = write_methods(context, media_methods)?;

    let interface = WIPICInterface {
        interface_0: 0,
        interface_1: 0,
        interface_2,
        interface_3: 0,
        interface_4: 0,
        interface_5: 0,
        interface_6,
        interface_7: 0,
        interface_8: 0,
        interface_9,
        interface_10: 0,
        interface_11: 0,
        interface_12: 0,
    };

    let address = context.alloc_raw(size_of::<WIPICInterface>() as u32)?;

    write_generic(context, address, interface)?;

    Ok(address)
}