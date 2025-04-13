# Collatz-reverse-rust
A high-performance Rust implementation for tracing any odd integer to a known G1 root using a custom reverse Collatz step formula. Supports extremely large numbers and computes cumulative step counts using optimized bigint arithmetic.
# Reverse Collatz to G1 (Rust)

This project implements a high-performance algorithm in Rust that recursively traces any odd integer backward to a known **G1 root**, using a custom variation of the Collatz Conjecture. It is optimized for extremely large numbers and efficiently calculates the total cumulative step count (Σk) during the reverse traversal.

---

## 📘 Overview

Given an odd integer `n`, the algorithm reverses the formula:


At each step, it finds the smallest `k` such that the result is a valid **odd** integer. The traversal stops when `m` matches a known **G1 root**:


The total step count is the sum of all `k` values used in the reverse process, including the final `k` that generates the G1 root.

---

## 🚀 Features

- Handles arbitrarily large integers with `num-bigint`
- Tracks cumulative step count (Σk)
- Supports more than 100 precomputed G1 roots
- Fast execution using bit-level logic
- Clean command-line interface
- Ideal for mathematical research and number theory experiments

---

## 🛠️ Requirements

- Rust (Install via [rustup.rs](https://rustup.rs))
- `num-bigint` and `num-traits` libraries

Add these to your `Cargo.toml`:

```toml
[dependencies]
num-bigint = "0.4"
num-traits = "0.2"
Input: 77777777777777777777

✅ Reached G1 root: 5
🧮 Total cumulative steps (Σk): 193

## 📄 License

This project is licensed under the Creative Commons BY-NC-ND 4.0 License.  
You may use and share it freely, but not for commercial or academic publishing purposes.  
See [LICENSE](./LICENSE) for full terms.
