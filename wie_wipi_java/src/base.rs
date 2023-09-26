use alloc::{boxed::Box, string::String, vec::Vec};

use wie_backend::{task::SleepFuture, Backend};

use crate::{
    array::Array,
    method::{MethodBody, MethodImpl, TypeConverter},
    proxy::JavaObjectProxy,
    r#impl::java::lang::Object,
};

pub struct JavaClassProto {
    pub parent_class: Option<&'static str>,
    pub interfaces: Vec<&'static str>,
    pub methods: Vec<JavaMethodProto>,
    pub fields: Vec<JavaFieldProto>,
}

pub type JavaError = anyhow::Error;
pub type JavaResult<T> = anyhow::Result<T>;

#[derive(Eq, PartialEq)]
pub enum JavaMethodFlag {
    NONE,
    STATIC = 0x8,
    NATIVE = 0x100,
}

#[derive(Eq, PartialEq)]
pub enum JavaFieldAccessFlag {
    NONE,
    STATIC = 0x8,
}

pub struct JavaMethodProto {
    pub name: String,
    pub signature: String,
    pub body: JavaMethodBody,
    pub flag: JavaMethodFlag,
}

pub struct JavaFieldProto {
    pub name: String,
    pub signature: String,
    pub access_flag: JavaFieldAccessFlag,
}

impl JavaFieldProto {
    pub fn new(name: &str, signature: &str, access_flag: JavaFieldAccessFlag) -> Self {
        Self {
            name: name.into(),
            signature: signature.into(),
            access_flag,
        }
    }
}

pub type JavaMethodBody = Box<dyn MethodBody<JavaError>>;

impl JavaMethodProto {
    pub fn new<M, F, R, P>(name: &str, signature: &str, method: M, flag: JavaMethodFlag) -> Self
    where
        M: MethodImpl<F, R, JavaError, P>,
    {
        Self {
            name: name.into(),
            signature: signature.into(),
            body: method.into_body(),
            flag,
        }
    }

    pub fn new_abstract(name: &str, signature: &str, flag: JavaMethodFlag) -> Self {
        struct AbstractCall {
            name: String,
            signature: String,
        }

        #[async_trait::async_trait(?Send)]
        impl MethodBody<JavaError> for AbstractCall {
            async fn call(&self, _: &mut dyn JavaContext, _: &[u32]) -> Result<u32, JavaError> {
                // TODO throw java.lang.AbstractMethodError
                anyhow::bail!("Call to abstract function {}{}", self.name, self.signature)
            }
        }

        Self {
            name: name.into(),
            signature: signature.into(),
            body: Box::new(AbstractCall {
                name: name.into(),
                signature: signature.into(),
            }),
            flag,
        }
    }
}

#[async_trait::async_trait(?Send)]
pub trait JavaContext {
    async fn instantiate(&mut self, type_name: &str) -> JavaResult<JavaObjectProxy<Object>>; // new
    async fn instantiate_array(&mut self, element_type_name: &str, count: u32) -> JavaResult<JavaObjectProxy<Array>>; // newarray
    fn destroy(&mut self, instance: JavaObjectProxy<Object>) -> JavaResult<()>;
    async fn call_method(&mut self, instance: &JavaObjectProxy<Object>, method_name: &str, signature: &str, args: &[u32]) -> JavaResult<u32>; // invokespecial/invokevirtual
    async fn call_static_method(&mut self, class_name: &str, method_name: &str, signature: &str, args: &[u32]) -> JavaResult<u32>; // invokestatic
    fn get_field(&self, instance: &JavaObjectProxy<Object>, field_name: &str) -> JavaResult<u32>; // getfield
    fn put_field(&mut self, instance: &JavaObjectProxy<Object>, field_name: &str, value: u32) -> JavaResult<()>; // putfield
    fn get_static_field(&self, class_name: &str, field_name: &str) -> JavaResult<u32>; // getstatic
    fn put_static_field(&mut self, class_name: &str, field_name: &str, value: u32) -> JavaResult<()>; // putstatic
    fn store_array_u32(&mut self, array: &JavaObjectProxy<Array>, offset: u32, values: &[u32]) -> JavaResult<()>; // iastore
    fn load_array_u32(&self, array: &JavaObjectProxy<Array>, offset: u32, count: u32) -> JavaResult<Vec<u32>>; // iaload
    fn store_array_u8(&mut self, array: &JavaObjectProxy<Array>, offset: u32, values: &[u8]) -> JavaResult<()>; // bastore
    fn load_array_u8(&self, array: &JavaObjectProxy<Array>, offset: u32, count: u32) -> JavaResult<Vec<u8>>; // baload
    fn array_element_size(&self, array: &JavaObjectProxy<Array>) -> JavaResult<usize>;
    fn array_length(&self, array: &JavaObjectProxy<Array>) -> JavaResult<u32>; // arraylength
    fn backend(&mut self) -> &mut Backend;
    fn spawn(&mut self, callback: JavaMethodBody) -> JavaResult<()>;
    fn sleep(&mut self, duration: u64) -> SleepFuture;
}

pub fn get_class_proto(name: &str) -> Option<JavaClassProto> {
    Some(match name {
        "java/io/ByteArrayInputStream" => crate::r#impl::java::io::ByteArrayInputStream::as_proto(),
        "java/io/IOException" => crate::r#impl::java::io::IOException::as_proto(),
        "java/io/InputStream" => crate::r#impl::java::io::InputStream::as_proto(),
        "java/lang/Class" => crate::r#impl::java::lang::Class::as_proto(),
        "java/lang/Exception" => crate::r#impl::java::lang::Exception::as_proto(),
        "java/lang/IllegalArgumentException" => crate::r#impl::java::lang::IllegalArgumentException::as_proto(),
        "java/lang/InterruptedException" => crate::r#impl::java::lang::InterruptedException::as_proto(),
        "java/lang/Object" => crate::r#impl::java::lang::Object::as_proto(),
        "java/lang/Runnable" => crate::r#impl::java::lang::Runnable::as_proto(),
        "java/lang/Runtime" => crate::r#impl::java::lang::Runtime::as_proto(),
        "java/lang/String" => crate::r#impl::java::lang::String::as_proto(),
        "java/lang/StringBuffer" => crate::r#impl::java::lang::StringBuffer::as_proto(),
        "java/lang/System" => crate::r#impl::java::lang::System::as_proto(),
        "java/lang/Thread" => crate::r#impl::java::lang::Thread::as_proto(),
        "java/lang/Throwable" => crate::r#impl::java::lang::Throwable::as_proto(),
        "java/util/Hashtable" => crate::r#impl::java::util::Hashtable::as_proto(),
        "java/util/Random" => crate::r#impl::java::util::Random::as_proto(),
        "java/util/Vector" => crate::r#impl::java::util::Vector::as_proto(),
        "org/kwis/msp/db/DataBase" => crate::r#impl::org::kwis::msp::db::DataBase::as_proto(),
        "org/kwis/msp/db/DataBaseRecordException" => crate::r#impl::org::kwis::msp::db::DataBaseRecordException::as_proto(),
        "org/kwis/msp/handset/BackLight" => crate::r#impl::org::kwis::msp::handset::BackLight::as_proto(),
        "org/kwis/msp/handset/HandsetProperty" => crate::r#impl::org::kwis::msp::handset::HandsetProperty::as_proto(),
        "org/kwis/msp/lcdui/Card" => crate::r#impl::org::kwis::msp::lcdui::Card::as_proto(),
        "org/kwis/msp/lcdui/Display" => crate::r#impl::org::kwis::msp::lcdui::Display::as_proto(),
        "org/kwis/msp/lcdui/EventQueue" => crate::r#impl::org::kwis::msp::lcdui::EventQueue::as_proto(),
        "org/kwis/msp/lcdui/Font" => crate::r#impl::org::kwis::msp::lcdui::Font::as_proto(),
        "org/kwis/msp/lcdui/Graphics" => crate::r#impl::org::kwis::msp::lcdui::Graphics::as_proto(),
        "org/kwis/msp/lcdui/Image" => crate::r#impl::org::kwis::msp::lcdui::Image::as_proto(),
        "org/kwis/msp/lcdui/Main" => crate::r#impl::org::kwis::msp::lcdui::Main::as_proto(),
        "org/kwis/msp/lcdui/Jlet" => crate::r#impl::org::kwis::msp::lcdui::Jlet::as_proto(),
        "org/kwis/msp/lcdui/JletEventListener" => crate::r#impl::org::kwis::msp::lcdui::JletEventListener::as_proto(),
        "org/kwis/msp/lwc/AnnunciatorComponent" => crate::r#impl::org::kwis::msp::lwc::AnnunciatorComponent::as_proto(),
        "org/kwis/msp/lwc/Component" => crate::r#impl::org::kwis::msp::lwc::Component::as_proto(),
        "org/kwis/msp/lwc/ContainerComponent" => crate::r#impl::org::kwis::msp::lwc::ContainerComponent::as_proto(),
        "org/kwis/msp/lwc/ShellComponent" => crate::r#impl::org::kwis::msp::lwc::ShellComponent::as_proto(),
        "org/kwis/msp/lwc/TextComponent" => crate::r#impl::org::kwis::msp::lwc::TextComponent::as_proto(),
        "org/kwis/msp/lwc/TextFieldComponent" => crate::r#impl::org::kwis::msp::lwc::TextFieldComponent::as_proto(),
        "org/kwis/msp/media/Clip" => crate::r#impl::org::kwis::msp::media::Clip::as_proto(),
        "org/kwis/msp/media/Player" => crate::r#impl::org::kwis::msp::media::Player::as_proto(),
        "org/kwis/msp/media/PlayListener" => crate::r#impl::org::kwis::msp::media::PlayListener::as_proto(),

        _ => return None,
    })
}

impl TypeConverter<u32> for u32 {
    fn to_rust(_: &mut dyn JavaContext, raw: u32) -> u32 {
        raw
    }

    fn from_rust(_: &mut dyn JavaContext, rust: u32) -> u32 {
        rust
    }
}

impl TypeConverter<i32> for i32 {
    fn to_rust(_: &mut dyn JavaContext, raw: u32) -> i32 {
        i32::from_le_bytes(raw.to_le_bytes())
    }

    fn from_rust(_: &mut dyn JavaContext, rust: i32) -> u32 {
        u32::from_le_bytes(rust.to_le_bytes())
    }
}

impl TypeConverter<()> for () {
    fn to_rust(_: &mut dyn JavaContext, _: u32) {}

    fn from_rust(_: &mut dyn JavaContext, _: ()) -> u32 {
        0
    }
}