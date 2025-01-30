use pyo3::prelude::*;
use rust_htslib::bam::{Read, IndexedReader};
use std::collections::{BTreeMap, HashSet};  // ✅ Sorted dictionary + Fast lookups

#[pyfunction]
fn get_depths(
    bam_path: &str, 
    chromosome: &str, 
    start: u64, 
    end: u64, 
    step: u64, 
    min_mapq: u8,  // Equivalent to samtools -q
    min_bq: u8,    // Equivalent to samtools -Q
    max_depth: usize // Equivalent to samtools -d
) -> PyResult<BTreeMap<u64, u32>> {  

    let mut bam = IndexedReader::from_path(bam_path)
        .map_err(|e| pyo3::exceptions::PyIOError::new_err(format!("Failed to open BAM: {}", e)))?;

    let tid = bam.header().tid(chromosome.as_bytes()).ok_or_else(|| {
        pyo3::exceptions::PyValueError::new_err(format!("Chromosome {} not found in BAM header", chromosome))
    })?;

    // ✅ **Single fetch for entire region (avoiding multiple `fetch()` calls)**
    bam.fetch((tid, start as i64 - 1, end as i64))
        .map_err(|e| pyo3::exceptions::PyIOError::new_err(format!("Failed to fetch region: {}", e)))?;

    let mut depths: BTreeMap<u64, u32> = BTreeMap::new();
    let valid_positions: HashSet<u64> = (start..=end).step_by(step as usize).collect();  // ✅ Fast lookup for step positions

    let mut pileup_engine = bam.pileup();
    pileup_engine.set_max_depth(max_depth as u32);

    let mut last_step = start;

    while let Some(pileup) = pileup_engine.next() {
        let pileup = pileup.map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("Error in pileup: {}", e)))?;
        let pos = pileup.pos() as u64 + 1;

        // ✅ **Skip until reaching the next step position**
        if pos < last_step { continue; }
        if !valid_positions.contains(&pos) { continue; }

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

        depths.insert(pos, filtered_depth);
        last_step = pos + step;  // ✅ **Jump to next valid step**
    }

    Ok(depths)  // ✅ Sorted by default
}


#[pymodule]  // ✅ Marks this module as a Python module
fn rustbam(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_depths, m)?)?;  // ✅ Registers `get_depths`
    Ok(())
}
