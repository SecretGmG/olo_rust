# olo_rust.pyi
from typing import Optional

TO_FEYNMAN: float
"""
Conversion factor from the Ellis-Zanderighi / OneLOop normalization
of one-loop scalar integrals to standard Feynman-diagram normalization.

Numerically: -1/(16 π²)
"""

class PyOLOResult:
    """
    Represents the Laurent expansion coefficients of a one-loop scalar function.
    
    Attributes:
        epsilon_0: ε⁰ coefficient (finite term)
        epsilon_minus_1: ε⁻¹ coefficient (first-order divergence, zero if IR-finite)
        epsilon_minus_2: ε⁻² coefficient (second-order divergence, zero if IR-finite)
    """
    @property
    def epsilon_0(self) -> float: ...
    
    @property
    def epsilon_minus_1(self) -> float: ...
    
    @property
    def epsilon_minus_2(self) -> float: ...
    
    def __repr__(self) -> str: ...

def one_point(m: float) -> PyOLOResult:
    """
    Computes the 1-point scalar (tadpole) function.

    Args:
        m: The squared mass of the propagator. Imaginary part should be ≤ 0.

    Returns:
        PyOLOResult containing the evaluated integral.
    
    Notes:
        Uses Ellis-Zanderighi normalization. Multiply by TO_FEYNMAN for standard Feynman normalization.
    """
    ...

def two_point(p: float, m1: float, m2: float) -> PyOLOResult:
    """
    Computes the 2-point scalar (bubble) function.

    Args:
        p: Squared momentum flowing through the propagator pair (imaginary part 0)
        m1: Squared mass of first propagator (imaginary part ≤ 0)
        m2: Squared mass of second propagator (imaginary part ≤ 0)

    Returns:
        PyOLOResult containing the evaluated integral.
    """
    ...

def three_point(p1: float, p2: float, p3: float, m1: float, m2: float, m3: float) -> PyOLOResult:
    """
    Computes the 3-point scalar (triangle) function.

    Args:
        p1, p2, p3: Squared momenta of the legs (imaginary part 0)
        m1, m2, m3: Squared masses of the propagators (imaginary part ≤ 0)

    Returns:
        PyOLOResult containing the evaluated integral.
    """
    ...

def four_point(
    p1: float, p2: float, p3: float, p4: float,
    p12: float, p23: float,
    m1: float, m2: float, m3: float, m4: float
) -> PyOLOResult:
    """
    Computes the 4-point scalar (box) function.

    Args:
        p1..p4: Squared momenta of the legs (imaginary part 0)
        p12, p23: Squared sums of momenta (imaginary part 0)
        m1..m4: Squared masses of the propagators (imaginary part ≤ 0)

    Returns:
        PyOLOResult containing the evaluated integral.
    """
    ...

def set_renormalization_scale(mu: float) -> None:
    """
    Sets the renormalization scale μ for OneLOop calculations.
    """
    ...

def set_log_level(unit_name: str, value: Optional[int] = ...) -> None:
    """
    Sets the output unit for OneLOop messages.

    Args:
        unit_name: One of "PrintAll", "Message", "Warning", "Error"
        value: Optional Fortran unit number (default None)
    """
    ...

def set_onshell_threshold(threshold: float) -> None:
    """
    Sets the on-shell threshold for OneLOop calculations.
    """
    ...
