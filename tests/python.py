import olo_rust
from cmath import rect

# Simple tadpole test
res = olo_rust.olo_1_point_complex_py(complex(1.0, 0.0))
print("ε⁰:", res.epsilon_0, "ε⁻¹:", res.epsilon_minus_1, "ε⁻²:", res.epsilon_minus_2)

# Set scale
olo_rust.olo_scale_py(91.1876)

# Set unit
olo_rust.set_olo_unit_py("PrintAll", 6)

# On-shell threshold
olo_rust.olo_onshell_py(1e-10)
