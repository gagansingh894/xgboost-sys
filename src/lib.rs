#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::upper_case_acronyms)]

use std::env;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
#[cfg(test)]
mod tests {
    use super::*;
}
