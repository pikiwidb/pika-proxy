const MAX_SLOT_NUM: usize = 1024;

pub struct Slot {
    id: u64,
    locked: bool,

    backend_addr: String,
    backend_add_group_id: u64,
    migrate_from: String,
    migrate_from_group_id: u64,

    forward_method: u64,
    replica_groups: Vec<Vec<String>>,
}
