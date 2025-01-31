// use rustbam::get_depths; // Import from crate
use rust_htslib::bam::{self, Read};

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_load_bam_file() {
        let bam_file = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/example.bam");
        let mut reader = bam::Reader::from_path(bam_file)
            .expect("Failed to open BAM file");
    
        let mut record = bam::Record::new();
        let mut count = 0;
    
        while let Some(result) = reader.read(&mut record) {
            match result {
                Ok(_) => count += 1,
                Err(e) => panic!("Error reading BAM record: {:?}", e),
            }
        }
    
        println!("Found {} records in BAM file", count);
    
        assert!(count > 0, "No records found in BAM file");
    }

    // #[test]
    // fn test_get_depths_rust() {
    //     let bam_file = "tests/example.bam";
    //     let chromosome = "chr1";
    //     let start = 1000000;
    //     let end = 1000100;
    //     let step = 10;
    //     let min_mapq = 30;
    //     let min_bq = 20;
    //     let max_depth = 1000;
    //     let num_threads = 4;

    //     let result = get_depths(
    //         bam_file,
    //         chromosome,
    //         start,
    //         end,
    //         step,
    //         min_mapq,
    //         min_bq,
    //         max_depth,
    //         num_threads
    //     ).expect("Failed to compute depths");

    //     assert!(!result.is_empty(), "Depth map should not be empty");

    //     for (&pos, &depth) in &result {
    //         assert!(pos >= start && pos <= end, "Position {} is out of range", pos);
    //         assert!(depth > 0, "Depth at position {} should be positive", pos);
    //         assert_eq!(
    //             (pos - start) % step, 0,
    //             "Position {} does not match the expected step interval of {}", pos, step
    //         );
    //     }

    //     println!("Test passed: Successfully computed depths for {} positions", result.len());
    // }
}
