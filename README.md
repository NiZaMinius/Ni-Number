Новый:
🌐 **Language:** English | [Русский](README.ru.md)

---

# ni-number &nbsp;[![Crates.io](https://img.shields.io/crates/v/ni-number)](https://crates.io/crates/ni-number) [![Docs.rs](https://docs.rs/ni-number/badge.svg)](https://docs.rs/ni-number) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

High-precision computation of the **Ni constant** (η_ν) — the quantum energy scattering constant — in Rust, with multiple selectable backends.

```
η_ν = 1.88937666040491913115597775087642096081019761538215...
```

---

## Concept of the Quantum Energy Scattering Constant (Ni Number)

**Author of the hypothesis:** NiZaMinius

### 1. Definition and Fundamental Meaning

The quantum energy scattering constant $\eta_{\nu}$ **(Ni Number)** is a fundamental mathematical quantity that defines the total amplitude of gauge-field attenuation at sub-Planckian distances.

The value of the Ni Number is strictly fixed mathematically and is approximately equal to **1.8893766...**

Modern physics postulates the existence of hidden, compactified spatial dimensions (Calabi–Yau manifolds). This concept asserts that when particles approach each other at distances smaller than the size of an atomic nucleus, a fraction of the virtual bosons (interaction carriers) scatters into that hidden volume. The Ni Number is the exact measure of the energy fraction that leaves the three-dimensional brane.

---

### 2. Mathematical Structure of the Constant

The constant is defined by a series normalized over all Kaluza–Klein spectral modes:

$$\eta_{\nu} = \sum_{n=1}^{\infty} \frac{\pi^n}{n! \cdot 2^{n^2}}$$

The index $n$ (1, 2, 3, ...) does not denote the ordinal number of a hidden dimension (since all dimensions exist simultaneously), but rather the **quantum mode number** (energy level). Just as a guitar string produces a set of harmonics, field quanta resonate inside a single multidimensional manifold at discrete frequencies. The infinite series sums all possible quantum states of energy leakage.

Each element of the formula has a rigorous justification in differential geometry and string theory:

#### 2.1. Geometry of Volume Resonances (numerator $\pi^n$)

Powers of $\pi$ define the geometric factor of multi-dimensionality. The growing exponent shows how the wavefront area expands as energy penetrates deeper into new coordinates at each successive harmonic.

#### 2.2. Topological Symmetry (factorial $n!$)

The appearance of the factorial is a direct consequence of the differential geometry of complex Kähler manifolds (Calabi–Yau spaces).

To compute the physical volume of such a manifold, one must integrate its fundamental volume form, which is built from the Kähler 2-form ($\omega$). The exact volume formula is:

$$V = \int_{M} \frac{\omega^n}{n!}$$

Due to the anticommutativity of the exterior product (which accounts for axis orientations), multiplication produces a vast number of overlapping coordinate permutations. Mathematics requires dividing the integral by the count of these permutations ($n!$). Thus, the factorial in the Ni Number naturally **collapses the redundant symmetries** of the compactified space, preventing the volume and leaking energy from growing infinitely.

#### 2.3. Quantum Blocking (base $2$ and exponent $n^2$)

In the physics of continuous processes, damping is typically governed by Euler's number $e$. However, at sub-Planckian scales, space is discrete.

In the matrix models of M-theory (e.g., BFSS), interactions between branes are described by $n \times n$ matrices, where $n$ is the mode quantum number. The matrix size $n^2$ determines the number of degrees of freedom of the quantum channel. Since at the fundamental level quantum information is binary (a qubit: connection present or absent, spin up or down), **two** is the basis of the state.

The total number of microstates is $2^{n^2}$. Using $e$ would mean applying classical statistics to quantum gravity. The base 2 is the signature of the fundamental informational nature of space. The quadratic growth of connections causes an avalanche-like resistance at high frequencies, ensuring **superfast convergence** of the series.

---

### 3. Modification of Interaction Laws at Ultra-Small Distances

According to this hypothesis, the classical inverse-square laws (Newton and Coulomb) break down at sub-Planckian distances, as predicted by models with extra dimensions (e.g., Randall–Sundrum theory).

The Ni Number ($\eta_{\nu}$) does not merely halve the force in the macroworld — it acts as a **dynamic integral coefficient** in the field equation itself, serving as the transition amplitude between the brane (the 3D world) and the bulk (the higher-dimensional space).

The modified force law is:

$$F(r) = \frac{G \cdot m_1 m_2}{r^2} \cdot \frac{1}{1 + \eta_{\nu} \cdot \left(\frac{R_{\mathrm{Planck}}}{r}\right)^d}$$

where $R_{\mathrm{Planck}}$ is the Planck length, $r$ is the distance, and $d$ is the effective number of hidden dimensions.

**Physical meaning:**

| Regime | Physics |
|---|---|
| $r \gg R_{\mathrm{Planck}}$ (macroworld) | The Planck fraction → 0, denominator → 1, classical law is recovered |
| $r \to R_{\mathrm{Planck}}$ (microworld) | Multi-dimensional physics activates; η_ν sets the "bandwidth" of the quantum portal; interaction carriers leak into the geometric pores of the manifold, causing a sharp drop in the registered force |

---

## Installation

Add to your `Cargo.toml`:

```toml
# Default — pure Rust, works everywhere, no system dependencies
[dependencies]
ni-number = "0.2"
```

### Backends

`ni-number` ships three selectable computation backends:

| Feature | Backend | System deps | Precision |
|---|---|---|---|
| `backend-dashu` | pure Rust **(default)** | none | arbitrary |
| `backend-rug` | GNU MPFR | GMP + MPFR | arbitrary |
| `backend-f64` | native f64 | none | ~15 digits |

**Default (pure Rust, recommended):**
```toml
[dependencies]
ni-number = "0.2"
```

**Maximum performance via GNU MPFR:**
```toml
[dependencies]
ni-number = { version = "0.2", default-features = false, features = ["backend-rug"] }
```

**Minimal footprint (embedded / WASM):**
```toml
[dependencies]
ni-number = { version = "0.2", default-features = false, features = ["backend-f64"] }
```

### System dependencies for `backend-rug`

Only needed if you explicitly enable `features = ["backend-rug"]`.

**Ubuntu / Debian**
```bash
sudo apt-get install libgmp-dev libmpfr-dev libmpc-dev
```

**macOS (Homebrew)**
```bash
brew install gmp mpfr libmpc
```

**Windows (MSYS2)**

1. Install [MSYS2](https://www.msys2.org)
2. In the MSYS2 MinGW64 terminal:
```bash
pacman -S --needed base-devel mingw-w64-x86_64-toolchain m4 make mingw-w64-x86_64-gmp mingw-w64-x86_64-mpfr
```
3. Add to PATH in Git Bash (`~/.bashrc`):
```bash
export PATH="/c/msys64/mingw64/bin:/c/msys64/usr/bin:$PATH"
```
4. Switch Rust toolchain:
```bash
rustup toolchain install stable-x86_64-pc-windows-gnu
rustup default stable-x86_64-pc-windows-gnu
```

---

## Usage

```rust
use ni_number::{NI_F64, ni_number_digits, ni_number, bits_for_digits};
use ni_number::backend::NiFloat;

fn main() {
    // Fast f64 constant — no computation, pre-baked (~15 digits)
    println!("η_ν ≈ {:.15}", NI_F64);

    // 100 decimal digits as String — result is cached after first call
    let s = ni_number_digits(100);
    println!("η_ν = {}", s);

    // Arbitrary-precision value for further computation
    let eta = ni_number(bits_for_digits(500));
    println!("{}", eta.to_decimal_string(500));
}
```

### Run the examples

```bash
# Basic demo
cargo run --example basic --release

# High-precision benchmark
cargo run --example high_precision --release
```

---

## API

| Item | Returns | Description |
|---|---|---|
| `NI_F64` | `f64` | Pre-computed constant at double precision |
| `NI_F32` | `f32` | Pre-computed constant at single precision |
| `NI_50_DIGITS` | `&str` | First 50 decimal digits, static string |
| `ni_number(bits)` | backend float | Full arbitrary-precision value, cached |
| `ni_number_digits(n)` | `String` | Decimal string with `n` digits after the point, cached |
| `ni_series(bits)` | `NiSeries` | Lazy iterator over series terms |
| `bits_for_digits(n)` | `u32` | Bit precision needed for `n` decimal digits |
| `clear_cache()` | `()` | Free cached values from memory |

---

## Performance

The series converges in fewer than 10 iterations for any practical precision level — viable even on low-end hardware.
Results are cached after the first call — subsequent calls at the same precision are instant.

| Time (release build, dashu) |
| Digits | dashu (series) | dashu (total) | rug (total) |
|--------|---------------|---------------|-------------|
| 100    | ~0.14 ms      | ~0.14 ms      | ~0.1 ms     |
| 1 000  | ~0.9 ms       | ~0.9 ms       | ~0.6 ms     |
| 5 000  | ~13.5 ms      | ~13.5 ms      | ~10.9 ms    |
| 10 000 | ~146 ms       | ~449 s        | ~52 ms      |

*dashu: series only. Formatting 10 000+ digits requires `backend-rug` for practical use.
---

## Testing

```bash
# Default backend (dashu)
cargo test

# With rug backend
cargo test --no-default-features --features backend-rug

# With f64 backend
cargo test --no-default-features --features backend-f64
```

---

## License

MIT — see [LICENSE](LICENSE).
