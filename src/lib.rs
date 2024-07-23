#![no_std]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::redundant_static_lifetimes)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::transmute_int_to_bool)]
#![feature(offset_of)]


include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
include!(concat!(env!("OUT_DIR"), "/interrupt.rs"));