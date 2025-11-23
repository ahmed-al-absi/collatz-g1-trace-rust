# 🔁 collatz-g1-trace-rust 

A high-performance Rust implementation for tracing **any odd integer** forward to a known G1 root using a custom reverse **Collatz step formula**. This project is designed for **extreme numeric scale**, supporting numbers with hundreds (or thousands) of digits, and efficiently computes the **cumulative step count** (Σk) with arbitrary precision.

---

## 📘 Project Overview

This tool implements a custom reverse traversal of the Collatz Conjecture.  
Given an **odd integer `n`**, the algorithm recursively applies the formula:


It searches for the **smallest `k`** such that the result `m` is an odd integer.  
The process repeats until `m` matches one of the **precomputed G1 roots**, such as:


🔢 The output includes:
- The G1 root reached
- The total sum of all `k` values (Σk)
- Optionally, a trace of all reverse steps

---

## 🧠 Mathematical Background

This implementation is based on the mathematical framework and reverse transformation formula presented in the following research:

> **Ahmed Al-Absi** (2025).  
> *A New Perspective on Proving the Collatz Conjecture*.  
> Zenodo. https://doi.org/10.5281/zenodo.15178879  
> 📅 Date: 2025-04-09

This code directly implements the reverse tracing logic discussed in the article, specifically using the transformation:



🔢 The output includes:
- The G1 root reached
- The total sum of all `k` values (Σk)
- Optionally, a trace of all reverse steps


Where each `k` is selected to ensure the result remains odd and positive.


### 🧪 Massive Input Test (2025-04-17)

- **Input length**: ~1,200 digits  
- **Reached G1 Root**: 85  
- **Cumulative Reverse Steps (Σk)**: 71,539  
- **Log Size**: 78 MB  
- **Status**: ✅ Successfully reached G1 root using Collatz-g1-trace-rust engine

---

## 🚀 Features

✅ Handles **extremely large integers** (100+ digits) with `num-bigint`  
✅ Tracks **cumulative step count** (Σk) across all reverse steps  
✅ Uses **bit-shift logic** for efficient `2^k` computation  
✅ Includes **over 100 G1 root values**, hardcoded for accuracy  
✅ Stable even at **gigantic input ranges** (~10¹⁰⁰⁰+)  
✅ Export results to `.txt` (optional)  
✅ Suitable for **mathematical research** and Collatz-based experimentation

---

## 🛠️ Requirements

🚀 Run in GitHub Codespaces

Open this repo in GitHub

Click Code → Create Codespace on main

When VS Code opens, open a terminal:

Terminal → New Terminal

[Rust](https://www.rust-lang.org/tools/install) (via `rustup`)

Run the program:

cargo run 

Or replace 111 with any odd number from let input = "121";

📄 Download the Path File

After running, the program creates:

reverse_path_111.txt


Find it in the left file explorer in Codespaces and click Download.

📁 Project Structure
src/main.rs        → main Rust program
reverse_path_*.txt → generated path files
Cargo.toml         → project config

🔧 Input Options

Use command line:

cargo run


and type the number when asked.
Terminal → New Terminal



to your desired number.
## 📄 License

This project is licensed under the Creative Commons BY-NC-ND 4.0 License.  
You may use and share it freely, but not for commercial or academic publishing purposes.  
See [LICENSE](./LICENSE) for full terms.
