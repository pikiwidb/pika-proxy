use std::collections::HashMap;

use crate::models::Request;

use crate::error::Result;
use crate::models::Slot;
use crate::utils::redis::InfoCache;

mod default_router;

pub trait Router {
    fn get_slots(&self) -> Vec<Box<Slot>>;
    fn get_slot(&self, id: u64) -> Box<Slot>;
    fn has_switched(&self) -> bool;
    fn fill_slot(&self, model: Box<Slot>) -> Result<()>;
    fn switch_masters(&self, masters: HashMap<u64, String>) -> Result<()>;
    fn dispatch(&self, request: Request) -> Result<()>;
    fn _dispatch_slot(&self, request: Request, id: u64) -> Result<()>;
    fn _dispatch_addr(&self, request: Request, addr: &str) -> bool;
    //fn _fill_slot(&self, m: &Slot, switched: bool, method: &dyn ForwardMethod);
    fn _try_switch_master(&self, id: u64, masters: HashMap<u64, String>, cache: &InfoCache);
}
