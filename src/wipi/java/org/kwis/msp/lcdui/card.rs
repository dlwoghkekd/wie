use std::any::Any;

use crate::wipi::java::{JavaClassImpl, JavaMethodImpl};

// class org.kwis.msp.lcdui.Card
pub struct Card {}

impl Card {
    pub fn as_java_impl() -> JavaClassImpl {
        JavaClassImpl {
            name: "org/kwis/msp/lcdui/Card".to_owned(),
            methods: vec![JavaMethodImpl {
                name: "H()V+<init>".to_owned(),
                body: Box::new(Self::init),
            }],
        }
    }

    fn init(_: Vec<Box<dyn Any>>) -> Box<dyn Any> {
        Box::new(())
    }
}
