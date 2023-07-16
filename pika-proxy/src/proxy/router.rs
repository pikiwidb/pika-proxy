use std::collections::HashMap;

use super::forward::ForwardMethod;
use super::request::Request;
use crate::error::{Error, Result};
use crate::models::slots::Slot;
use crate::utils::redis::InfoCache;

pub trait Router {
    fn start(&self);
    fn close(&self);
    fn get_slots(&self) -> Vec<Box<Slot>>;
    fn get_slot(&self, id: u64) -> Box<Slot>;
    fn has_switched(&self) -> bool;
    fn fill_slot(&self, model: Box<Slot>) -> Result<()>;
    fn keep_alive(&self) -> Result<()>;
    fn switch_masters(masters: HashMap<u64, String>) -> Result<()>;
    fn _is_online(&self) -> bool;
    fn _dispatch(&self, r: &Request) -> Result<()>;
    fn _dispatch_slot(&self, r: &Request, id: u64) -> Result<()>;
    fn _dispatch_addr(&self, r: &Request, addr: &str) -> bool;
    fn _fill_slot(&self, m: &Slot, switched: bool, method: &dyn ForwardMethod);
    fn _try_switch_master(&self, id: u64, masters: HashMap<u64, String>, cache: &InfoCache);
}
