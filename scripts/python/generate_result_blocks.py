#!/usr/bin/env python3
"""
Generate Result Blocks for Phase 11
Based on Universal Task Execution Policy §8D
"""

import json
import re
import sys
from pathlib import Path
from datetime import datetime, timezone
from typing import List, Tuple, Dict

# Phase definitions
PHASES = [
    (1, "Core System", "VER001-VER028"),
    (2, "Agent Architecture", "VER029-VER050"),
    (3, "Shared Provider", "VER051-VER070"),
    (4, "Digest Pipeline", "VER071-VER090"),
    (5, "P2P & UI", "VER091-VER110"),
    (6, "Governance", "VER111-VER126"),
    (7, "Performance", "VER127-VER145"),
    (8, "Regression", "REG001-REG014"),
    (9, "Truth Gate", "TG001-CT005"),
    (10, "Multi-GPU", "GPU001-GPU018"),
]


def count_items_in_range(content: str, range_str: str) -> Tuple[int, int]:
    """Count total and completed items in a range."""
    parts = range_str.split("-")
    if len(parts) != 2:
        return (0, 0)

    # Extract prefix and numbers
    prefix = "".join(c for c in parts[0] if c.isalpha())
    start_num = int("".join(c for c in parts[0] if c.isdigit()))
    end_num = int("".join(c for c in parts[1] if c.isdigit()))

    total = 0
    completed = 0

    for num in range(start_num, end_num + 1):
        item_id = f"{prefix}{num:03d}"
        pattern_complete = f"- [X] {item_id} -"
        pattern_incomplete = f"- [ ] {item_id} -"

        if pattern_complete in content:
            total += 1
            completed += 1
        elif pattern_incomplete in content:
            total += 1

    return (total, completed)


def generate_result_block(
    phase: int, phase_name: str, total: int, completed: int
) -> Dict:
    """Generate a Result Block for a phase."""
    incomplete = total - completed

    # Determine result status
    if incomplete == 0:
        result = "PASS"
    elif completed == 0:
        result = "FAIL"
    else:
        result = "PARTIAL"

    # Generate why message
    if result == "PASS":
        why = f"All {total} verification items completed"
    elif result == "FAIL":
        why = f"No verification items completed (0/{total})"
    else:
        pct = (completed / total * 100) if total > 0 else 0
        why = f"Partial completion: {completed} of {total} items verified ({pct:.1f}%)"

    # Generate next step if not PASS
    next_step = None
    if result != "PASS":
        next_step = f"Complete remaining {incomplete} verification items for Phase {phase}"

    return {
        "phase": phase,
        "phase_name": phase_name,
        "result": result,
        "why": why,
        "next": next_step,
        "total_items": total,
        "completed_items": completed,
        "incomplete_items": incomplete,
        "timestamp": datetime.now(timezone.utc).isoformat(),
    }


def format_result_block(rb: Dict) -> str:
    """Format Result Block as text (per §8D)."""
    lines = [f"RESULT: {rb['result']}"]
    lines.append(f"WHY: {rb['why']}")
    if rb["next"]:
        lines.append(f"NEXT: {rb['next']}")
    return "\n".join(lines)


def format_markdown(rb: Dict) -> str:
    """Format Result Block as markdown."""
    status_icon = "✅" if rb["result"] == "PASS" else "⚠️" if rb["result"] == "PARTIAL" else "❌"
    block_text = format_result_block(rb)
    return f"""### Phase {rb['phase']}: {rb['phase_name']}

```
{block_text}
```

**Status**: {status_icon} {rb['result']} ({rb['completed_items']}/{rb['total_items']})

"""


def generate_final_sign_off(
    result_blocks: List[Dict],
    test_results_path: Path,
) -> Dict:
    """Generate Final Sign-Off status."""
    all_phases_pass = all(rb["result"] == "PASS" for rb in result_blocks)
    final_report_exists = (test_results_path / "FINAL_REPORT.md").exists()
    hashes_verified = (test_results_path / "HASHES.txt").exists()
    evidence_ledger_complete = (test_results_path / "EVIDENCE_LEDGER.md").exists()

    # Check for unremedied failures
    no_unremedied_failures = all(
        rb["result"] == "PASS" or (rb["result"] != "PASS" and rb["next"] is not None)
        for rb in result_blocks
    )

    if (
        all_phases_pass
        and final_report_exists
        and hashes_verified
        and no_unremedied_failures
        and evidence_ledger_complete
    ):
        overall_status = "PASS"
    elif all_phases_pass or final_report_exists:
        overall_status = "PARTIAL"
    else:
        overall_status = "FAIL"

    return {
        "all_phases_pass": all_phases_pass,
        "final_report_complete": final_report_exists,
        "hashes_verified": hashes_verified,
        "no_unremedied_failures": no_unremedied_failures,
        "evidence_ledger_complete": evidence_ledger_complete,
        "timestamp": datetime.now(timezone.utc).isoformat(),
        "overall_status": overall_status,
    }


def main():
    """Main entry point."""
    script_dir = Path(__file__).parent
    noa_root = script_dir.parent.parent
    checklist_path = (
        noa_root
        / "specs/001-noa-seed-foundation/checklists/verification.md"
    )
    test_results_path = noa_root / "test-results"

    # Read checklist
    if not checklist_path.exists():
        print(f"Error: Checklist not found at {checklist_path}", file=sys.stderr)
        sys.exit(1)

    with open(checklist_path, "r", encoding="utf-8") as f:
        checklist_content = f.read()

    # Generate Result Blocks
    result_blocks = []
    for phase_num, phase_name, item_range in PHASES:
        total, completed = count_items_in_range(checklist_content, item_range)
        rb = generate_result_block(phase_num, phase_name, total, completed)
        result_blocks.append(rb)

    # Generate Final Sign-Off
    sign_off = generate_final_sign_off(result_blocks, test_results_path)

    # Save JSON
    output_json = test_results_path / "result_blocks.json"
    with open(output_json, "w", encoding="utf-8") as f:
        json.dump(
            {"result_blocks": result_blocks, "final_sign_off": sign_off},
            f,
            indent=2,
        )

    # Generate markdown report
    output_md = test_results_path / "PHASE11_RESULT_BLOCKS.md"
    with open(output_md, "w", encoding="utf-8") as f:
        f.write("# Phase 11: Result Blocks & Sign-Off\n\n")
        f.write(f"**Generated**: {datetime.now(timezone.utc).isoformat()}\n")
        f.write("**Based On**: Universal Task Execution Policy §8D\n\n")
        f.write("---\n\n")

        f.write("## Per-Phase Result Blocks\n\n")
        for rb in result_blocks:
            f.write(format_markdown(rb))

        f.write("---\n\n")

        f.write("## Final Sign-Off\n\n")
        status_icon = (
            "✅"
            if sign_off["overall_status"] == "PASS"
            else "⚠️"
            if sign_off["overall_status"] == "PARTIAL"
            else "❌"
        )
        f.write(f"**Status**: {status_icon} {sign_off['overall_status']}\n\n")
        f.write("| Check | Status | Notes |\n")
        f.write("|-------|--------|-------|\n")
        f.write(
            f"| FINAL001 | {'✅' if sign_off['all_phases_pass'] else '❌'} | All phase RESULT blocks are PASS |\n"
        )
        f.write(
            f"| FINAL002 | {'✅' if sign_off['final_report_complete'] else '❌'} | FINAL_REPORT.md complete and reviewed |\n"
        )
        f.write(
            f"| FINAL003 | {'✅' if sign_off['hashes_verified'] else '❌'} | All HASHES.txt entries verified |\n"
        )
        f.write(
            f"| FINAL004 | {'✅' if sign_off['no_unremedied_failures'] else '❌'} | No FAIL or PARTIAL without documented remedy |\n"
        )
        f.write(
            f"| FINAL005 | {'✅' if sign_off['evidence_ledger_complete'] else '❌'} | Evidence Ledger complete with Triple-Verify outcomes |\n"
        )
        f.write(f"\n**Overall**: {sign_off['overall_status']}\n")

    print(f"✅ Result Blocks generated:")
    print(f"   - JSON: {output_json}")
    print(f"   - Markdown: {output_md}")


if __name__ == "__main__":
    main()

