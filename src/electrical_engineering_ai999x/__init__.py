"""electrical-engineering-ai999x — offline electrical engineering answers with their method, steps and verification.

    import electrical_engineering_ai999x as m
    m.ask("Find the Thevenin equivalent between nodes 2 and 0 of the circuit: V1 1 0 12; R1 1 2 1000; R2 2 0 2000.").show()
    m.ask("Find the gain margin of G(s) = 10/(s(s+1)(s+5)).").show()
    m.solve("series_rlc_impedance", R="100 ohm", L="50 mH", C="10 uF", f="1 kHz").show()
    m.capabilities()                   # every certified capability

Covers DC circuits (series/parallel combinations, dividers, mesh and nodal analysis, netlists, Thevenin/Norton,
bridges, delta-wye), AC circuits (impedance, phasors, complex power, power-factor correction, three-phase loads,
resonance and Q, RC filters, transformers), transients (first-order and series RLC step responses), operational
amplifiers, diodes and transistors (Shockley diode, Zener regulator, BJT divider bias, MOSFET saturation), digital
logic (Boolean evaluation, equivalence, truth tables and minimal sum of products, number bases, two's complement,
Gray code, ADC codes) and control systems (Routh-Hurwitz, closed-loop poles, gain and phase margins, Bode points,
steady-state error, second-order step specifications, state space, lead compensators, DC motors). Every capability is
a deterministic algorithm whose result is checked before it is shown; a question that is not determined is refused
rather than guessed.

Creator / engineer: Ibnul Jaif Farabi. Company: AI999X.
"""
from __future__ import annotations

from pathlib import Path

try:                                       # the installed package carries its own copy of the engine
    from ._engine import Answer
    from ._engine.product import Product
except ImportError:                        # the factory's source tree, during development
    from ai999x_factory.runtime import Answer
    from ai999x_factory.runtime.product import Product

__version__ = "1.0.0"
_P = Product("electrical_engineering", [Path(__file__).resolve().parent / "pack", Path(__file__).resolve().parents[3]])
pack, ask, solve, compile_problem = _P.pack, _P.ask, _P.solve, _P.compile_problem
formula, formulas, quantities, conditions, capabilities = _P.formula, _P.formulas, _P.quantities, _P.conditions, _P.capabilities
examples, family_examples, structured_example, to_si = _P.examples, _P.family_examples, _P.structured_example, _P.to_si
templates, procedure_examples = _P.templates, _P.procedure_examples
prove, derivations, about = _P.prove, _P.derivations, (lambda: {**_P.about(), "version": __version__})
knowledge, lookup, related = _P.knowledge, _P.lookup, _P.related
plot, plot_relation, search = _P.plot, _P.plot_relation, _P.search   # figures (SVG) and capability search ([search])
__all__ = ["ask", "solve", "prove", "derivations", "about", "formula", "formulas", "quantities", "conditions", "capabilities",
           "compile_problem", "pack", "Answer", "plot", "plot_relation", "search", "__version__"]
