use crate::wipi::java::{JavaClassProto, JavaMethodProto, Jvm};

// class java.lang.Thread
pub struct Thread {}

impl Thread {
    pub fn as_proto() -> JavaClassProto {
        JavaClassProto {
            methods: vec![JavaMethodProto::new("<init>", "()V", Self::init)],
        }
    }

    fn init(_: &mut dyn Jvm) {
        log::debug!("Thread::<init>");
    }
}