use pyo3::prelude::*;
use rust_htslib::bam::{self, Read, IndexedReader, record::Record};
use std::collections::HashMap;

#[pyfunction]
fn get_depth(
    bam_path: &str, 
    chromosome: &str, 
    start: u64, 
    end: u64, 
    step: u64, 
    min_mapq: u8, // Equivalent to samtools -q
    min_bq: u8,    // Equivalent to samtools -Q
    max_depth: usize // Equivalent to samtools -d
) -> PyResult<Vec<(u64, u32)>> {
    let mut bam = IndexedReader::from_path(bam_path)
        .map_err(|e| pyo3::exceptions::PyIOError::new_err(format!("Failed to open BAM: {}", e)))?;

    let tid = bam.header().tid(chromosome.as_bytes()).ok_or_else(|| {
        pyo3::exceptions::PyValueError::new_err(format!("Chromosome {} not found in BAM header", chromosome))
    })?;

    // Fetch only the requested region
    bam.fetch((tid, start as i64 - 1, end as i64))
        .map_err(|e| pyo3::exceptions::PyIOError::new_err(format!("Failed to fetch region: {}", e)))?;

    let mut depths: HashMap<u64, u32> = HashMap::new();

    // Pre-fill all positions with 0 at step intervals
    for pos in (start..=end).step_by(step as usize) {
        depths.insert(pos, 0);
    }

    // Bitmask for filtering flags (like samtools --ff)
    const FLAG_UNMAPPED: u16 = 0x4;
    const FLAG_SECONDARY: u16 = 0x100;
    const FLAG_SUPPLEMENTARY: u16 = 0x800;
    const FLAG_DUPLICATE: u16 = 0x400;
    const FLAG_QCFAIL: u16 = 0x200;

    // Process pileup with samtools-like filtering
    for pileup in bam.pileup() {
        let pileup = pileup.map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("Error in pileup: {}", e)))?;
        let pos = pileup.pos() as u64 + 1; // Convert 0-based to 1-based

        if depths.contains_key(&pos) {
            let mut filtered_depth = 0;

            for alignment in pileup.alignments() {
                let record = alignment.record();

                let flags = record.flags();

                // **Apply samtools-like read filters**
                if flags & (FLAG_UNMAPPED | FLAG_SECONDARY | FLAG_SUPPLEMENTARY | FLAG_DUPLICATE | FLAG_QCFAIL) != 0 {
                    continue; // Skip filtered reads
                }

                // **Apply minimum mapping quality filter (`-q`)**
                if record.mapq() < min_mapq {
                    continue;
                }

                // **Apply minimum base quality filter (`-Q`)**
                if let Some(qpos) = alignment.qpos() {
                    if record.qual()[qpos] < min_bq {
                        continue;
                    }
                }

                filtered_depth += 1;

                // **Apply maximum depth filter (`-d`)**
                if filtered_depth >= max_depth as u32 {
                    break;
                }
            }

            depths.insert(pos, filtered_depth);
        }
    }

    // Convert HashMap to sorted Vec<(position, depth)>
    let mut sorted_depths: Vec<(u64, u32)> = depths.into_iter().collect();
    sorted_depths.sort_by_key(|&(pos, _)| pos);

    Ok(sorted_depths)
}

#[pymodule]
fn rustbam(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_depth, m)?)?;
    Ok(())
}

