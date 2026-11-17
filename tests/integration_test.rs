//! Integration tests for SigmaOS-arch-file-indexer.

use sigmaos_arch_file_indexer_lib::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}
