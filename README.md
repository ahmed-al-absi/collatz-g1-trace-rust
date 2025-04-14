# 🔁 Collatz Reverse (Rust)

A high-performance Rust implementation for tracing **any odd integer** backward to a known G1 root using a custom reverse **Collatz step formula**. This project is designed for **extreme numeric scale**, supporting numbers with hundreds (or thousands) of digits, and efficiently computes the **cumulative step count** (Σk) with arbitrary precision.

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

- [Rust](https://www.rust-lang.org/tools/install) (via `rustup`)
- `num-bigint` and `num-traits` libraries

Add this to your `Cargo.toml`:

```toml
[dependencies]
num-bigint = "0.4"
num-traits = "0.2"

output format:
<value> → k=<step>
...

## 📄 License

This project is licensed under the Creative Commons BY-NC-ND 4.0 License.  
You may use and share it freely, but not for commercial or academic publishing purposes.  
See [LICENSE](./LICENSE) for full terms.
