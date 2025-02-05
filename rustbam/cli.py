import argparse
import json
from rustbam import get_depths  # import from rust module

def get_depths_cli():
    parser = argparse.ArgumentParser(
        description="Compute sequencing depth from a BAM file."
    )

    # Positional Arguments (Required, without `-b`, `-c`, etc.)
    parser.add_argument("bam", help="Path to the indexed BAM file")
    parser.add_argument("chromosome", help="Chromosome name (e.g., 'chr1')")
    parser.add_argument("start", type=int, help="Start position (1-based)")
    parser.add_argument("end", type=int, help="End position (1-based)")

    # Optional Arguments (With Flags, Showing Defaults in Help)
    parser.add_argument(
        "-t", "--step", type=int, default=1,
        help="Step size for sampling positions (default: %(default)s)"
    )
    parser.add_argument(
        "-Q", "--min_mapq", type=int, default=0,
        help="Minimum mapping quality (default: %(default)s)"
    )
    parser.add_argument(
        "-q", "--min_bq", type=int, default=13,
        help="Minimum base quality (default: %(default)s)"
    )
    parser.add_argument(
        "-d", "--max_depth", type=int, default=8000,
        help="Maximum depth allowed (default: %(default)s)"
    )
    parser.add_argument(
        "-n", "--num_threads", type=int, default=12,
        help="Number of threads (default: %(default)s)"
    )
    parser.add_argument(
        "-j", "--json", action="store_true",
        help="Output results in JSON format"
    )

    args = parser.parse_args()

    # Call Rust function
    positions, depths = get_depths(
        args.bam, args.chromosome, args.start, args.end,
        args.step, args.min_mapq, args.min_bq, args.max_depth, args.num_threads
    )

    # Print output
    if args.json:
        print(json.dumps(dict(zip(positions, depths)), indent=4))
    else:
        for pos, depth in zip(positions, depths):
            print(f"{pos}\t{depth}")

if __name__ == "__main__":
    get_depths_cli()

