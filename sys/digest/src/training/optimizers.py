"""
DSPy Optimizers (MIPROv2, COPRO)

T378: Simplified optimizer stubs that score module outputs and return
the top-ranked prompt variants.
"""

from __future__ import annotations

from dataclasses import dataclass
from typing import Iterable

from .modules import ModuleOutput


@dataclass
class OptimizerResult:
    name: str
    best_prompt: str
    trials: list[ModuleOutput]


class MIPROv2Optimizer:
    name = "MIPROv2"

    def optimize(self, outputs: Iterable[ModuleOutput]) -> OptimizerResult:
        ranked = sorted(outputs, key=lambda o: o.score, reverse=True)
        best = ranked[0] if ranked else ModuleOutput("", "", "No outputs", 0.0)
        return OptimizerResult(self.name, best.prompt, list(ranked))


class COPROOptimizer:
    name = "COPRO"

    def optimize(self, outputs: Iterable[ModuleOutput]) -> OptimizerResult:
        ranked = sorted(outputs, key=lambda o: len(o.response), reverse=True)
        best = ranked[0] if ranked else ModuleOutput("", "", "No outputs", 0.0)
        return OptimizerResult(self.name, best.prompt, list(ranked))


def build_optimizer(name: str):
    if name.lower() == "miprov2":
        return MIPROv2Optimizer()
    if name.lower() == "copro":
        return COPROOptimizer()
    return MIPROv2Optimizer()
