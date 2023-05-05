use crate::wipi::java::{JavaClassProto, JavaMethodProto, Jvm};

// class org.kwis.msp.media.Clip
pub struct Clip {}

impl Clip {
    pub fn as_proto() -> JavaClassProto {
        JavaClassProto {
            methods: vec![JavaMethodProto::new("<init>", "(I)V", Self::init)],
        }
    }

    fn init(_: &mut dyn Jvm, _: u32) {
        log::debug!("Clip::<init>");
    }
}