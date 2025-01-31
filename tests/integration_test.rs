extern crate rustbam; // ✅ Required for integration tests

use rustbam::get_depths;

#[test]
fn test_depth_integration() {
    let bam_path = format!("{}/tests/example.bam", env!("CARGO_MANIFEST_DIR"));

    let result = get_depths(&bam_path, "chr1", 1000, 2000, 50, 0, 13, 8000);
    
    assert!(result.is_ok(), "get_depths failed: {:?}", result);
}
