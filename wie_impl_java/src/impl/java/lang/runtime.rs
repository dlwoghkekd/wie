use alloc::vec;

use crate::{
    base::{JavaClassProto, JavaContext, JavaMethodFlag, JavaMethodProto, JavaResult},
    proxy::JavaObjectProxy,
};

// class java.lang.Runtime
pub struct Runtime {}

impl Runtime {
    pub fn as_proto() -> JavaClassProto {
        JavaClassProto {
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, JavaMethodFlag::NONE),
                JavaMethodProto::new("getRuntime", "()Ljava/lang/Runtime;", Self::get_runtime, JavaMethodFlag::NONE),
                JavaMethodProto::new("totalMemory", "()J", Self::total_memory, JavaMethodFlag::NONE),
                JavaMethodProto::new("freeMemory", "()J", Self::free_memory, JavaMethodFlag::NONE),
                JavaMethodProto::new("gc", "()V", Self::gc, JavaMethodFlag::NONE),
            ],
            fields: vec![],
        }
    }

    async fn init(_: &mut dyn JavaContext, this: JavaObjectProxy<Runtime>) -> JavaResult<()> {
        tracing::warn!("stub java.lang.Runtime::<init>({:#x})", this.ptr_instance);

        Ok(())
    }

    async fn get_runtime(context: &mut dyn JavaContext) -> JavaResult<JavaObjectProxy<Runtime>> {
        tracing::debug!("java.lang.Runtime::get_runtime");

        let instance = context.jvm().instantiate_class("java/lang/Runtime").await?;
        let instance = JavaObjectProxy::new(context.instance_raw(&instance));
        context.call_method(&instance.cast(), "<init>", "()V", &[]).await?;

        Ok(instance)
    }

    async fn total_memory(_: &mut dyn JavaContext, this: JavaObjectProxy<Runtime>) -> JavaResult<i32> {
        tracing::warn!("stub java.lang.Runtime::totalMemory({:#x})", this.ptr_instance);

        Ok(0x100000) // TODO: hardcoded
    }

    async fn free_memory(_: &mut dyn JavaContext, this: JavaObjectProxy<Runtime>) -> JavaResult<i32> {
        tracing::warn!("stub java.lang.Runtime::freeMemory({:#x})", this.ptr_instance);

        Ok(0x100000) // TODO: hardcoded
    }

    async fn gc(_: &mut dyn JavaContext, this: JavaObjectProxy<Runtime>) -> JavaResult<()> {
        tracing::warn!("stub java.lang.Runtime::gc({:#x})", this.ptr_instance);

        Ok(())
    }
}
