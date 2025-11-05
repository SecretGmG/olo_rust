import olo_rust

# Set log level to print everything
olo_rust.set_log_level("printall")

factor = olo_rust.TO_FEYNMAN

# Set renormalization scale
olo_rust.set_renormalization_scale(10)

# Set on-shell threshold
olo_rust.set_onshell_threshold(0.001)

# 1-point (tadpole) example
m = 0.5
r1 = olo_rust.one_point(m)
print("1-point:", r1)

# 2-point (bubble) example
p = 1.0
m1 = 0.5
m2 = 0.2
r2 = olo_rust.two_point(p, m1, m2)
print("2-point:", r2)

# 3-point (triangle) example
p1 = 0.01
p2 = 0.01
p3 = 0.001
m = 0.02
r3 = olo_rust.three_point(p1, p2, p3, m, m, m)
print("3-point:", r3)

# 4-point (box) example
p4 = 0.0
p12 = 0.01
p23 = 0.0
r4 = olo_rust.four_point(p1, p2, p3, p4, p12, p23, m, m, m, m)
print("4-point:", r4)

# Print individual Laurent coefficients
print("3-point ε⁰:", r3.epsilon_0)
print("3-point ε⁻¹:", r3.epsilon_minus_1)
print("3-point ε⁻²:", r3.epsilon_minus_2)
