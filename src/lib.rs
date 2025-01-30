use pyo3::prelude::*;
use rust_htslib::bam::{Read, IndexedReader, pileup::Pileup};
use rayon::prelude::*;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

#[pyfunction]
fn get_depths(
    bam_path: &str, 
    chromosome: &str, 
    start: u64, 
    end: u64, 
    step: u64, 
    min_mapq: u8,  
    min_bq: u8,    
    max_depth: usize, 
    num_threads: usize  
) -> PyResult<BTreeMap<u64, u32>> {  

    let tid = {
        let bam = IndexedReader::from_path(bam_path)
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(format!("Failed to open BAM: {}", e)))?;
        bam.header().tid(chromosome.as_bytes()).ok_or_else(|| {
            pyo3::exceptions::PyValueError::new_err(format!("Chromosome {} not found in BAM header", chromosome))
        })?
    };

    let depths = Arc::new(Mutex::new(BTreeMap::<u64, u32>::new()));

    let chunk_size = (end - start) / num_threads as u64;
    let step_positions: Vec<u64> = (start..=end).step_by(step as usize).collect();

    // ✅ **Parallelize over independent regions**
    (0..num_threads).into_par_iter().for_each(|i| {
        let chunk_start = start + i as u64 * chunk_size;
        let chunk_end = if i == num_threads - 1 { end } else { chunk_start + chunk_size };

        let mut bam_thread = IndexedReader::from_path(bam_path).expect("Failed to open BAM file");
        bam_thread.fetch((tid, chunk_start as i64 - 1, chunk_end as i64)).expect("Failed to fetch region");

        let mut pileup_engine = bam_thread.pileup();
        pileup_engine.set_max_depth(max_depth as u32);

        let mut local_depths: BTreeMap<u64, u32> = BTreeMap::new();

        while let Some(pileup) = pileup_engine.next() {
            let pileup = pileup.expect("Error in pileup");
            let pos = pileup.pos() as u64 + 1;

            if !step_positions.contains(&pos) {
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

            local_depths.insert(pos, filtered_depth);
        }

        // ✅ **Safely merge local results into global depths**
        let mut global_depths = depths.lock().unwrap();
        global_depths.extend(local_depths);
    });

    let final_result = Arc::try_unwrap(depths).unwrap().into_inner().unwrap();
    Ok(final_result)
}


#[pymodule]  // ✅ Marks this module as a Python module
fn rustbam(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_depths, m)?)?;  // ✅ Registers `get_depths`
    Ok(())
}
