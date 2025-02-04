pub use rust_htslib::bam::IndexedReader;
pub use rust_htslib::bam::Record;

use pyo3::prelude::*;
use rust_htslib::bam::Read;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

/// Computes sequencing depth across a genomic region from a BAM file.
///
/// This function calculates the depth of sequencing coverage at specified genomic positions 
/// within a given region using a BAM file. It supports multi-threaded processing and allows
/// filtering based on mapping quality (MAPQ), base quality (BQ), and depth thresholds.
///
/// Args:
///     bam_path (str): Path to the indexed BAM file.
///     chromosome (str): Chromosome name (e.g., `"chr1"`, `"chrX"`).
///     start (int): 1-based start position (inclusive).
///     end (int): 1-based end position (inclusive).
///     step (int, optional): Step size for sampling positions (default: 1).
///     min_mapq (int, optional): Minimum mapping quality for alignments (default: 0).
///     min_bq (int, optional): Minimum base quality for bases included in depth calculation (default: 13).
///     max_depth (int, optional): Maximum depth allowed at any position (default: 8000).
///     num_threads (int, optional): Number of threads for parallel processing (default: 12).
///
/// Returns:
///     Tuple[List[int], List[int]]: A tuple containing:
///         - List[int]: Genomic positions (1-based).
///         - List[int]: Corresponding sequencing depths.
///
/// Raises:
///     IOError: If the BAM file cannot be opened or indexed.
///     ValueError: If the specified chromosome is not found in the BAM header.
///
/// Notes:
///     - The BAM file must be indexed using `samtools index`.
///     - Uses a 1-based genomic coordinate system.
///     - Multi-threading is enabled using `rayon` for faster performance.
///
/// Example:
///     >>> import rustbam
///     >>> positions, depths = rustbam.get_depths(
///     ...     "example.bam", "chr1", 100000, 200000, step=10, 
///     ...     min_mapq=30, min_bq=20, max_depth=5000, num_threads=4
///     ... )
///     >>> print(positions[:5])  # [100000, 100010, 100020, 100030, 100040]
///     >>> print(depths[:5])     # [12, 15, 10, 8, 20]
#[pyfunction]
#[pyo3(signature = (
    bam_path,
    chromosome,
    start,
    end,
    step = 1,
    min_mapq = 0,
    min_bq = 13,
    max_depth = 8000,
    num_threads = 12
))]
pub fn get_depths(
    bam_path: &str, 
    chromosome: &str, 
    start: u64, 
    end: u64, 
    step: u64, 
    min_mapq: u8,  
    min_bq: u8,    
    max_depth: usize, 
    num_threads: usize  
) -> PyResult<(Vec<u64>, Vec<u32>)> {  

    let bam = IndexedReader::from_path(bam_path)
        .map_err(|e| pyo3::exceptions::PyIOError::new_err(format!("Failed to open BAM: {}", e)))?;

    let tid = bam.header().tid(chromosome.as_bytes()).ok_or_else(|| {
        pyo3::exceptions::PyValueError::new_err(format!("Chromosome {} not found in BAM header", chromosome))
    })?;

    // **Precompute chunk boundaries for parallel processing**
    let chunk_starts: Vec<u64> = (0..num_threads)
        .map(|i| (start as f64 + ((end - start) as f64 / num_threads as f64 * i as f64)).floor() as u64)
        .collect();
    
    let chunk_ends: Vec<u64> = (0..num_threads)
        .map(|i| (start as f64 + ((end - start) as f64 / num_threads as f64 * (i + 1) as f64)).floor() as u64)
        .collect();

    // **Store results in parallel-safe vectors**
    let positions = Arc::new(Mutex::new(Vec::<u64>::new()));
    let depths = Arc::new(Mutex::new(Vec::<u32>::new()));

    // **Parallelize pileup processing**
    (0..num_threads).into_par_iter().for_each(|i| {
        let chunk_start = chunk_starts[i];
        let chunk_end = chunk_ends[i];

        let mut bam_thread = IndexedReader::from_path(bam_path).expect("Failed to open BAM file");
        bam_thread.fetch((tid, chunk_start as i64 - 1, chunk_end as i64)).expect("Failed to fetch region");

        let mut local_results: Vec<(u64, u32)> = Vec::new();
        let mut pileup_engine = bam_thread.pileup();
        pileup_engine.set_max_depth(max_depth as u32);

        while let Some(pileup) = pileup_engine.next() {
            let pileup = pileup.expect("Error in pileup");
            let pos = pileup.pos() as u64 + 1; // Convert 0-based to 1-based

            // Skip positions outside chunk boundaries
            if pos < chunk_start || pos >= chunk_end {
                continue;
            }

            // Ensure global alignment of `step` with `start`
            if (pos - start) % step != 0 {
                continue;
            }

            let mut filtered_depth = 0;

            for alignment in pileup.alignments() {
                let record = alignment.record();

                const FLAG_FILTER: u16 = 0x4 | 0x100 | 0x800 | 0x400 | 0x200;
                if record.flags() & FLAG_FILTER != 0 { continue; }

                if record.mapq() < min_mapq { continue; }

                if let Some(qpos) = alignment.qpos() {
                    if record.qual()[qpos] < min_bq { continue; }
                }

                filtered_depth += 1;
                if filtered_depth >= max_depth as u32 { break; }
            }

            local_results.push((pos, filtered_depth));
        }

        // **Safely merge local results into global storage**
        let mut global_positions = positions.lock().unwrap();
        let mut global_depths = depths.lock().unwrap();

        for (pos, depth) in local_results {
            global_positions.push(pos);
            global_depths.push(depth);
        }
    });

    // **Extract final sorted vectors**
    let final_positions = Arc::try_unwrap(positions).unwrap().into_inner().unwrap();
    let final_depths = Arc::try_unwrap(depths).unwrap().into_inner().unwrap();

    // **Ensure sorting by position**
    let mut combined: Vec<(u64, u32)> = final_positions.into_iter().zip(final_depths.into_iter()).collect();
    combined.sort_unstable();  // Faster than stable sort

    let (sorted_positions, sorted_depths): (Vec<u64>, Vec<u32>) = combined.into_iter().unzip();
    Ok((sorted_positions, sorted_depths))
}

/// Python module definition
#[pymodule]
fn rustbam(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_depths, m)?)?;
    Ok(())
}