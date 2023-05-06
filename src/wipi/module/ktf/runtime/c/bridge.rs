use crate::{
    core::arm::{allocator::Allocator, ArmCore},
    wipi::c::{CBridge, CBridgeMethod, CResult},
};

pub struct KtfCBridge<'a> {
    core: &'a mut ArmCore,
}

impl<'a> KtfCBridge<'a> {
    pub fn new(core: &'a mut ArmCore) -> Self {
        Self { core }
    }
}

impl CBridge for KtfCBridge<'_> {
    fn alloc(&mut self, size: u32) -> CResult<u32> {
        Allocator::alloc(self.core, size)
    }

    fn write_raw(&mut self, address: u32, data: &[u8]) -> CResult<()> {
        self.core.write_raw(address, data)
    }

    fn register_function(&mut self, method: CBridgeMethod) -> CResult<u32> {
        self.core.register_function(move |mut core: ArmCore| {
            let mut bridge = KtfCBridge::new(&mut core);
            let result = method(&mut bridge)?;

            Ok::<_, anyhow::Error>(result)
        })
    }
}
