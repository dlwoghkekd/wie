use alloc::vec;

use crate::{
    base::{JavaClassProto, JavaContext, JavaMethodFlag, JavaMethodProto, JavaResult},
    proxy::JavaObjectProxy,
};

// class org.kwis.msp.lcdui.Font
pub struct Font {}

impl Font {
    pub fn as_proto() -> JavaClassProto {
        JavaClassProto {
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, JavaMethodFlag::NONE),
                JavaMethodProto::new("getHeight", "()I", Self::get_height, JavaMethodFlag::NONE),
                JavaMethodProto::new(
                    "getDefaultFont",
                    "()Lorg/kwis/msp/lcdui/Font;",
                    Self::get_default_font,
                    JavaMethodFlag::STATIC,
                ),
            ],
            fields: vec![],
        }
    }

    async fn init(_: &mut dyn JavaContext, this: JavaObjectProxy<Font>) -> JavaResult<()> {
        tracing::warn!("stub org.kwis.msp.lcdui.Font::<init>({:#x})", this.ptr_instance);

        Ok(())
    }

    async fn get_height(_: &mut dyn JavaContext) -> JavaResult<i32> {
        tracing::warn!("stub org.kwis.msp.lcdui.Font::get_height");

        Ok(12) // TODO: hardcoded
    }

    async fn get_default_font(context: &mut dyn JavaContext) -> JavaResult<JavaObjectProxy<Font>> {
        tracing::warn!("stub org.kwis.msp.lcdui.Font::get_default_font");

        let instance = context.instantiate("Lorg/kwis/msp/lcdui/Font;").await?;
        context.call_method(&instance, "<init>", "()V", &[]).await?;

        Ok(instance.cast())
    }
}