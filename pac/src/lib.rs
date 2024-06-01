#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]

pub mod generic;
use generic::*;

#[cfg(feature = "at32f413")]
pub mod at32f413;
#[cfg(feature = "at32f415")]
pub mod at32f415;
#[cfg(feature = "at32f421")]
pub mod at32f421;
#[cfg(feature = "at32f435")]
pub mod at32f435;
#[cfg(feature = "at32f437")]
pub mod at32f437;
