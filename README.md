# 🦀`rustbam`🐍 - Rust-powered fast BAM depth extraction with Python bindings

[![CI](https://github.com/shahcompbio/rustbam/actions/workflows/integration.yaml/badge.svg)](https://github.com/shahcompbio/rustbam/actions/workflows/integration.yaml)
![Coverage](https://github.com/shahcompbio/rustbam/raw/main/coverage.svg)

`rustbam` is a high-performance BAM depth calculator written in **Rust**, with **Python bindings** for fast and efficient genomic data analysis.  
(It's only a coincidence that "bam" means "snake" in Korean.)

## 📦 Installation  

### **Install from PyPI** 

You can install `rustbam` directly with `pip`:

```
pip install rustbam
```

### **Requirements**

`rustbam` requires **Python ≥3.8** and a compatible Linux environment. Prebuilt wheels are available for most systems, but if you encounter installation issues, ensure your system has **glibc ≥2.17** (manylinux2014 compatibility). To check your version, run `ldd --version`. 

If you need to compile from source, install **Rust** (`cargo`), **Python development headers** (`python3-dev` or `python3-devel`), and GCC (`build-essential` or `Development Tools`). For the best experience, we recommend using `pip install rustbam`, but if necessary, you can force a source installation with `pip install --no-binary rustbam rustbam`.

## 🛠️ Usage

### **Python API**

After installation, you can use `rustbam` in Python:

```python
import rustbam

positions, depths = rustbam.get_depths(
    bam_path,         # path to bam file
    chromosome,       # chromosome/contig name
    start,            # 1-based inclusive start coordinate
    end,              # 1-based inclusive end coordinate
    step=10,          # step as in range(start, end, step) - default: 1
    min_mapq=0,       # minimum mapping quality - default 0
    min_bq=13,        # minimum base quality - default 13 (as in samtools mpileup)
    max_depth=8000,   # maximum depth to return per base position
    num_threads=12,   # number of threads for parallelization
)

print(positions[:5])  # e.g. [100000, 100010, 100020, 100030, 100040]
print(depths[:5])     # e.g. [12, 15, 10, 8, 20]
```


### **CLI (Command Line Interface)**

After installation, you can use `rustbam` in your shell (note that coordinates are 1-based and inclusive, as in `samtools mpileup`):

```bash
$ rustbam --help
usage: rustbam [-h] [-t STEP] [-Q MIN_MAPQ] [-q MIN_BQ] [-d MAX_DEPTH] [-n NUM_THREADS] [-j] bam chromosome start end

Compute sequencing depth from a BAM file.

positional arguments:
  bam                   Path to the indexed BAM file
  chromosome            Chromosome name (e.g., 'chr1')
  start                 Start position (1-based)
  end                   End position (1-based)

options:
  -h, --help            show this help message and exit
  -t STEP, --step STEP  Step size for sampling positions (default: 1)
  -Q MIN_MAPQ, --min_mapq MIN_MAPQ
                        Minimum mapping quality (default: 0)
  -q MIN_BQ, --min_bq MIN_BQ
                        Minimum base quality (default: 13)
  -d MAX_DEPTH, --max_depth MAX_DEPTH
                        Maximum depth allowed (default: 8000)
  -n NUM_THREADS, --num_threads NUM_THREADS
                        Number of threads (default: 12)
  -j, --json            Output results in JSON format
```

An example usage of the CLI:

```bash
$ rustbam tests/example.bam chr1 1000000 1000005
1000000 51
1000001 52
1000002 44
1000003 52
1000004 53
1000005 47
```

You can get much faster depths result compared to samtools mpileup (as long as you use the multithreading option, `-n`):

```bash
$ time samtools mpileup /path/to/a/large/bam -r chr1:1-30000000 > /dev/null
[mpileup] 1 samples in 1 input files

real    2m9.018s
user    2m8.100s
sys     0m0.685s

$ time rustbam /path/to/a/large/bam chr1 1 30000000 -n 12 > /dev/null

real    0m22.038s
user    1m40.802s
sys     0m2.060s
```

Don't even get me started about `pysam` (`rustbam` is about 12x fater with `-n 2`). 😠

---

## 🔥 Features

✅ **Fast**: Uses Rust’s efficient `rust-htslib` for BAM processing, and supports parallelism.  
✅ **Python bindings**: Seamless integration with Python via `pyo3`.  
✅ **Custom filtering**: Supports read quality (`-q`), base quality (`-Q`), and max depth (`-d`).  
✅ **Supports large BAM files**: Uses `IndexedReader` for efficient region querying.

---

## 📜 License

`rustbam` is released under the **MIT License**. See LICENSE for details.

---

## 🤝 Contributing

1. Fork the repo on GitHub.
2. Create a new branch: `git checkout -b feature-new`
3. Commit your changes: `git commit -m "Add new feature"`
4. Push to your branch: `git push origin feature-new`
5. Open a **Pull Request** 🎉

---

## 🌍 Acknowledgments

Built using **[rust-htslib](https://github.com/rust-bio/rust-htslib)** and **[pyo3](https://github.com/PyO3/pyo3)**.

