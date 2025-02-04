use clap::{Arg, Command};
use rustbam::get_depths;

fn main() {
    let matches = Command::new("rustbam")
        .about("Extract sequencing depth from a BAM file")
        .arg(Arg::new("bam").required(true).help("Path to the BAM file"))
        .arg(Arg::new("chromosome").required(true).help("Chromosome name"))
        .arg(Arg::new("start").required(true).value_parser(clap::value_parser!(u64)).help("Start position"))
        .arg(Arg::new("end").required(true).value_parser(clap::value_parser!(u64)).help("End position"))
        .arg(Arg::new("step").long("step").default_value("1").help("Sampling step size"))
        .arg(Arg::new("min_mapq").long("min-mapq").default_value("0").help("Minimum mapping quality"))
        .arg(Arg::new("min_bq").long("min-bq").default_value("13").help("Minimum base quality"))
        .arg(Arg::new("max_depth").long("max-depth").default_value("8000").help("Maximum depth"))
        .arg(Arg::new("threads").long("threads").default_value("12").help("Number of threads"))
        .get_matches();

    let bam_path = matches.get_one::<String>("bam").unwrap();
    let chromosome = matches.get_one::<String>("chromosome").unwrap();
    let start: u64 = *matches.get_one("start").unwrap();
    let end: u64 = *matches.get_one("end").unwrap();
    let step: u64 = *matches.get_one("step").unwrap();
    let min_mapq: u8 = *matches.get_one("min_mapq").unwrap();
    let min_bq: u8 = *matches.get_one("min_bq").unwrap();
    let max_depth: usize = *matches.get_one("max_depth").unwrap();
    let num_threads: usize = *matches.get_one("threads").unwrap();

    let (positions, depths) = get_depths(bam_path, chromosome, start, end, step, min_mapq, min_bq, max_depth, num_threads).unwrap();

    for (pos, depth) in positions.iter().zip(depths.iter()) {
        println!("{}\t{}", pos, depth);
    }
}
