# OneLOop Rust Wrapper

This crate provides a **Rust interface** to the [OneLOop](https://helac-phegas.web.cern.ch/OneLOop.html) Fortran library, enabling computation of **scalar one-loop integrals** in a safe and idiomatic Rust way.


---

## Features

- Safe Rust wrappers for OneLOop scalar functions:
  - `olo_1_point_complex` → 1-point (tadpole) function
  - `olo_2_point_complex` → 2-point (bubble) function
  - `olo_3_point_complex` → 3-point (triangle) function
  - `olo_4_point_complex` → 4-point (box) function
- Returns a `ResultOLO` struct containing the Laurent expansion coefficients:
  - `ε⁰` → finite term
  - `ε⁻¹` → first-order divergence (zero if IR-finite)
  - `ε⁻²` → second-order divergence (zero if IR-finite)
---

## Installation and Build Process for rust projects

The wrapper can simply be added with `cargo add`:

```bash
cargo add --git https://github.com/SecretGmG/olo_rust
```

When you build the crate via `cargo build`

A `build.rs` script checks whether `oneloop/libavh_olo.a` exists.

If it is missing, the script runs `python3 create.py` inside the `oneloop/` directory to generate the static library.

Cargo then links the library (`libavh_olo.a`) and the Fortran runtime (`gfortran` and `quadmath`) automatically.

**Requirements:**

- Python3 (`python3` in PATH)
- `gfortran` compiler
- `m4`

Note: Windows is not currently supported.

Yes, that looks correct and clear. A few minor tweaks I’d suggest for clarity and correctness:

1. In the Python section, clarify that you need the **full repo** because `maturin` builds the extension. Right now it just says “clone this git repository” but doesn’t explain why.
2. Fix the typo: “ad above” → “as above”.
3. Optional: show how to **import and use the Python bindings** after building with `maturin`.

For example, your Python section could be:

---

## Installation and Build Process for Python projects

When building with the `python` feature enabled, Python bindings are built using `pyo3`.
To use them, clone the repository and build with `maturin`. The same dependencies as above still apply.
See `python_example.py` for more examples.

```bash
git clone https://github.com/SecretGmG/olo_rust.git
maturin develop --manifest-path olo_rust/Cargo.toml --release
```

After building, you can use the bindings in Python:

```python
import olo_rust
from num_complex import Complex

r = olo_rust.one_point(Complex(0.1, 0.0))
print(r.epsilon_0)
```

---

## Example

```rust
/// Minkowski dot product: (E^2 - px^2 - py^2 - pz^2)
fn minkowski_dot(p: [f64; 4]) -> f64 {
    p[0] * p[0] - (p[1] * p[1] + p[2] * p[2] + p[3] * p[3])
}

fn main() {
    // 2-point example (bubble)
    let p = Complex64::new(1.0, 0.0);
    let m1 = Complex64::new(0.5, 0.0);
    let m2 = Complex64::new(0.2, 0.0);
    let result = two_point(p, m1, m2);
    println!("2-point result: {:?}", result);

    // 3-point example (triangle)
    let k1 = [0.005, 0.0, 0.0, 0.005];
    let k2 = [0.005, 0.0, 0.0, -0.005];

    let p1 = Complex64::new(minkowski_dot(k1), 0.0);
    let p2 = Complex64::new(minkowski_dot(k2), 0.0);

    // p3 = (k1 + k2)^2
    let k3 = [k1[0] + k2[0], k1[1] + k2[1], k1[2] + k2[2], k1[3] + k2[3]];
    let p3 = Complex64::new(minkowski_dot(k3), 0.0);

    let m = Complex64::new(0.02, 0.0);

    let m1 = m * m;
    let m2 = m * m;
    let m3 = m * m;

    let result = three_point(p1, p2, p3, m1, m2, m3);
    println!("3-point result: {:?}", result);
    println!(
        "Interal value in Feynman convention: {:?}",
        result.epsilon_0() * TO_FEYNMAN
    )
}
```

---

## License

Licensed under the GNU GENERAL PUBLIC LICENSE Version 3
