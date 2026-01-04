import csv
import pathlib
import sys

CSV_PATH = pathlib.Path(__file__).resolve().parents[2] / "docs" / "plans" / "configs-audit-table.csv"


def normalize_row(row, width):
    if len(row) > width:
        fixed = row[: width - 1] + [", ".join(row[width - 1 :])]
        return fixed
    if len(row) < width:
        return row + [""] * (width - len(row))
    return row


def main():
    if not CSV_PATH.exists():
        print(f"Missing {CSV_PATH}", file=sys.stderr)
        return 2

    with CSV_PATH.open(newline="", encoding="utf-8") as f:
        reader = csv.reader(f)
        rows = list(reader)

    if not rows:
        print("Empty CSV", file=sys.stderr)
        return 2

    header = rows[0]
    width = len(header)

    fixed_rows = [header]
    bad = 0
    for r in rows[1:]:
        r2 = normalize_row(r, width)
        if len(r) != width:
            bad += 1
        fixed_rows.append(r2)

    tmp = CSV_PATH.with_suffix(".csv.tmp")
    with tmp.open("w", newline="", encoding="utf-8") as f:
        writer = csv.writer(f, quoting=csv.QUOTE_MINIMAL)
        writer.writerows(fixed_rows)

    tmp.replace(CSV_PATH)
    print(f"Normalized {CSV_PATH} (fixed {bad} rows)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
