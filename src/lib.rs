#![no_std]
#![allow(non_camel_case_types)]
#![doc = include_str!("../README.md")]

#[cfg_attr(feature = "nrf51", path = "./chips/nrf51/pac.rs")]
#[cfg_attr(feature = "nrf52805", path = "./chips/nrf52805/pac.rs")]
#[cfg_attr(feature = "nrf52810", path = "./chips/nrf52810/pac.rs")]
#[cfg_attr(feature = "nrf52811", path = "./chips/nrf52811/pac.rs")]
#[cfg_attr(feature = "nrf52820", path = "./chips/nrf52820/pac.rs")]
#[cfg_attr(feature = "nrf52832", path = "./chips/nrf52832/pac.rs")]
#[cfg_attr(feature = "nrf52833", path = "./chips/nrf52833/pac.rs")]
#[cfg_attr(feature = "nrf52840", path = "./chips/nrf52840/pac.rs")]
#[cfg_attr(feature = "nrf5340-app", path = "./chips/nrf5340-app/pac.rs")]
#[cfg_attr(feature = "nrf5340-net", path = "./chips/nrf5340-net/pac.rs")]
#[cfg_attr(feature = "nrf9120", path = "./chips/nrf9120/pac.rs")]
#[cfg_attr(feature = "nrf9160", path = "./chips/nrf9160/pac.rs")]
mod inner;
pub use inner::*;
