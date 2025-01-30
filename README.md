# 🦀 rustbam - Fast BAM Depth Calculator in Rust & Python

🚀 **rustbam** is a high-performance BAM depth calculator written in **Rust**, with **Python bindings** for fast and efficient genomic data analysis. It offers a `pysam`-like API but is significantly faster due to Rust’s performance advantages.

## 📦 Installation  

### **Install from PyPI (No Conda Required)** 

You can install `rustbam` directly with `pip`:

```
pip install rustbam
```

## 🛠️ Usage

### **Python API**

After installation, you can use `rustbam` in Python:

python

CopyEdit

```
import rustbam
depths = rustbam.get_depth("example.bam", "chr1", 1000000, 1000100, 
    step=10,     # As in range(start, end, step)
    min_mapq=0,  # Minimum mapping quality (samtools -q)
    min_bq=13,   # Minimum base quality (samtools -Q)
    max_depth=8000  # Max per-file depth (samtools -d)
)
print(depths)
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

