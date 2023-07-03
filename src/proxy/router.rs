use std::collections::HashMap;

use super::forward::ForwardMethod;
use super::request::Request;
use crate::models::slots::Slot;
use crate::utils::error::ProxyResult;
use crate::utils::redis::InfoCache;

pub struct Router {}

impl Router {
    fn start(&self) {
        todo!()
    }

    fn close(&self) {
        todo!()
    }

    fn get_slots(&self) -> Vec<Box<Slot>> {
        todo!()
    }

    fn get_slot(&self, id: u64) -> Box<Slot> {
        todo!()
    }

    fn has_switched(&self) -> bool {
        todo!()
    }

    fn fill_slot(&self, model: Box<Slot>) -> ProxyResult<()> {
        todo!()
    }

    fn keep_alive(&self) -> ProxyResult<()> {
        todo!()
    }

    fn switch_masters(masters: HashMap<u64, String>) -> ProxyResult<()> {
        todo!()
    }

    fn _is_online(&self) -> bool {
        todo!()
    }

    fn _dispatch(&self, r: &Request) -> ProxyResult<()> {
        todo!()
    }

    fn _dispatch_slot(&self, r: &Request, id: u64) -> ProxyResult<()> {
        todo!()
    }

    fn _dispatch_addr(&self, r: &Request, addr: &str) -> bool {
        todo!()
    }

    fn _fill_slot(&self, m: &Slot, switched: bool, method: &dyn ForwardMethod) {
        todo!()
    }

    fn _try_switch_master(&self, id: u64, masters: HashMap<u64, String>, cache: &InfoCache) {
        todo!()
    }
}

mod tests {}
