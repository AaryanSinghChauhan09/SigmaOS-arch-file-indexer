// Integration test for SigmaOS-arch-file-indexer
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use arch_file_indexer as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
