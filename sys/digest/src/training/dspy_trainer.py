"""
DSPy Training Pipeline

T376: Orchestrates DSPy modules, optimizers, and PRD loader into a small training loop.
"""

from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path
from typing import List

from .modules import ChainOfThoughtModule, ModuleOutput, PredictModule, ReActModule, run_modules
from .optimizers import OptimizerResult, build_optimizer
from .prd_loader import PRDExample, load_prd_samples


@dataclass
class TrainingConfig:
    optimizer: str = "miprov2"
    output_dir: Path = Path("data/training")
    max_examples: int = 50


class DSPyTrainer:
    def __init__(self):
        self.modules = [
            PredictModule(),
            ChainOfThoughtModule(),
            ReActModule(),
        ]

    def train(self, prd_path: Path, config: TrainingConfig | None = None) -> OptimizerResult:
        cfg = config or TrainingConfig()
        dataset = load_prd_samples(prd_path)[: cfg.max_examples]

        trials: List[ModuleOutput] = []
        for example in dataset:
            trials.extend(self._run_modules_for_example(example))

        optimizer = build_optimizer(cfg.optimizer)
        result = optimizer.optimize(trials)

        self._persist_results(cfg.output_dir, result)
        return result

    def _run_modules_for_example(self, example: PRDExample) -> List[ModuleOutput]:
        prompt = f"{example.user_story}\nAcceptance: {'; '.join(example.acceptance_criteria)}"
        return run_modules(
            self.modules,
            {
                "prompt": prompt,
                "expected": example.acceptance_criteria[0] if example.acceptance_criteria else "",
            },
        )

    def _persist_results(self, output_dir: Path, result: OptimizerResult) -> None:
        output_dir.mkdir(parents=True, exist_ok=True)
        summary_path = output_dir / "dspy_training_summary.txt"

        lines = [
            f"Optimizer: {result.name}",
            f"Best prompt: {result.best_prompt}",
            f"Trials: {len(result.trials)}",
        ]
        summary_path.write_text("\n".join(lines), encoding="utf-8")
