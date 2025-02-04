# 🦀 `rustbam` - Rust-powered fast BAM depth extraction with Python bindings

🚀 **rustbam** is a high-performance BAM depth calculator written in **Rust**, with **Python bindings** for fast and efficient genomic data analysis.

## 📦 Installation  

### **Install from PyPI (No Conda Required)** 

You can install `rustbam` directly with `pip`:

```
pip install rustbam
```

## 🛠️ Usage

### **Python API**

After installation, you can use `rustbam` in Python:

```bash
import rustbam

positions, depths = rustbam.get_depths(
    bam_path,         # path to bam file
    chromosome,       # chromosome/contig name
    start,            # 1-based inclusive start coordinate
    end,              # 1-based inclusive end coordinate
    step=1,           # step as in range(start, end, step) - default: 1
    min_mapq=0,       # minimum mapping quality - default 0
    min_bq=13,        # minimum base quality - default 13 (as in samtools mpileup)
    max_depth=8000,   # maximum depth to return per base position
    num_threads=12,   # number of threads for parallelization
)

print(positions[:5])  # e.g. [100000, 100010, 100020, 100030, 100040]
print(depths[:5])     # e.g. [12, 15, 10, 8, 20]
```


### **CLI (Command Line Interface)**

Coming soon! 🚀

---

## 🔥 Features

✅ **Fast**: Uses Rust’s efficient `rust-htslib` for BAM processing.  
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

