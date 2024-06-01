#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]

#[cfg(feature = "rt")]
pub use crate::pac::interrupt;

pub mod bb;

mod sealed {
    pub trait Sealed {}
}
pub(crate) use sealed::Sealed;

fn stripped_type_name<T>() -> &'static str {
    let s = core::any::type_name::<T>();
    let p = s.split("::");
    p.last().unwrap()
}

#[cfg(feature = "at32f413")]
pub use at32_pac::{at32f413 as pac, generic};
#[cfg(feature = "at32f415")]
pub use at32_pac::{at32f415 as pac, generic};
#[cfg(feature = "at32f421")]
pub use at32_pac::{at32f421 as pac, generic};
#[cfg(feature = "at32f435")]
pub use at32_pac::{at32f435 as pac, generic};
#[cfg(feature = "at32f437")]
pub use at32_pac::{at32f437 as pac, generic};
