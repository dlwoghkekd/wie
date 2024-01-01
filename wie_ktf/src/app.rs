use alloc::string::{String, ToString};

use anyhow::Context;

use wie_backend::{App, SystemHandle};
use wie_core_arm::{Allocator, ArmCore};
use wie_impl_java::r#impl::org::kwis::msp::lcdui::Jlet;

use crate::runtime::KtfJavaContext;

const IMAGE_BASE: u32 = 0x100000;

pub struct KtfApp {
    core: ArmCore,
    system: SystemHandle,
    bss_size: u32,
    main_class_name: String,
}

impl KtfApp {
    pub fn new(main_class_name: &str, system: &SystemHandle) -> anyhow::Result<Self> {
        let mut core = ArmCore::new(system.clone())?;

        Allocator::init(&mut core)?;

        let resource = system.resource();
        let filename = resource.files().find(|x| x.starts_with("client.bin")).context("Invalid archive")?;
        let data = resource.data(resource.id(filename).context("Resource not found")?);

        let bss_size = Self::load(&mut core, data, filename)?;

        Ok(Self {
            core,
            system: system.clone(),
            bss_size,
            main_class_name: main_class_name.to_string(),
        })
    }

    #[tracing::instrument(name = "start", skip_all)]
    async fn do_start(core: &mut ArmCore, system: &mut SystemHandle, bss_size: u32, main_class_name: String) -> anyhow::Result<()> {
        let wipi_exe = crate::runtime::start(core, IMAGE_BASE, bss_size).await?;
        tracing::debug!("Got wipi_exe {:#x}", wipi_exe);

        let fn_init = crate::runtime::init(core, wipi_exe).await?;
        tracing::debug!("Call wipi init at {:#x}", fn_init);

        let result = core.run_function::<u32>(fn_init, &[]).await?;
        anyhow::ensure!(result == 0, "wipi init failed with code {:#x}", result);

        let mut java_context = KtfJavaContext::new(core, system);

        Jlet::start(&mut java_context, &main_class_name).await?;

        Ok(())
    }

    fn load(core: &mut ArmCore, data: &[u8], filename: &str) -> anyhow::Result<u32> {
        let bss_start = filename.find("client.bin").context("Incorrect filename")? + 10;
        let bss_size = filename[bss_start..].parse::<u32>()?;

        core.load(data, IMAGE_BASE, data.len() + bss_size as usize)?;

        tracing::debug!("Loaded at {:#x}, size {:#x}, bss {:#x}", IMAGE_BASE, data.len(), bss_size);

        Ok(bss_size)
    }
}

impl App for KtfApp {
    fn start(&mut self) -> anyhow::Result<()> {
        let mut core = self.core.clone();
        let mut system_clone = self.system.clone();

        let bss_size = self.bss_size;
        let main_class_name = self.main_class_name.clone();

        self.core
            .spawn(move || async move { Self::do_start(&mut core, &mut system_clone, bss_size, main_class_name).await });

        Ok(())
    }

    fn crash_dump(&self) -> String {
        self.core.dump_reg_stack(IMAGE_BASE)
    }
}
