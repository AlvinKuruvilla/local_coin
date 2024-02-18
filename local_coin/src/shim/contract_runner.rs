use sysinfo::System;

pub fn is_substrate_node_running() -> bool {
    let s = System::new_all();
    if s.processes_by_name("substrate-contracts-node")
        .peekable()
        .peek()
        .is_some()
    {
        return true;
    }
    false
}
