use std::any::Any;

use crate::wipi::java_impl::{JavaClassImpl, JavaMethodImpl};

// class java.lang.Object
pub struct Object {}

impl Object {
    pub fn as_java_impl() -> JavaClassImpl {
        JavaClassImpl {
            name: "java/lang/Object".into(),
            methods: vec![JavaMethodImpl {
                name: "<init>".into(),
                signature: "()V".into(),
                body: Box::new(Self::init),
            }],
        }
    }

    fn init(_: Vec<Box<dyn Any>>) -> Box<dyn Any> {
        log::debug!("Object::<init>");

        Box::new(())
    }
}