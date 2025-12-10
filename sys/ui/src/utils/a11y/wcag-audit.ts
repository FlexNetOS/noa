// WCAG 2.1 AAA audit helpers (T786)
// Provides lightweight contrast and aria labeling checks to aid accessibility audits.

export type ContrastResult = {
  foreground: string;
  background: string;
  ratio: number;
  passesAA: boolean;
  passesAAA: boolean;
};

export type AuditIssue =
  | { type: "contrast"; detail: ContrastResult }
  | { type: "aria"; element: string; message: string };

export type AuditReport = {
  issues: AuditIssue[];
  summary: {
    total: number;
    contrast: number;
    aria: number;
  };
};

/**
 * Compute relative luminance per WCAG.
 */
function luminance(hex: string): number {
  const rgb = hexToRgb(hex);
  const [r, g, b] = rgb.map((v) => {
    const c = v / 255;
    return c <= 0.03928 ? c / 12.92 : Math.pow((c + 0.055) / 1.055, 2.4);
  });
  return 0.2126 * r + 0.7152 * g + 0.0722 * b;
}

function hexToRgb(hex: string): [number, number, number] {
  const normalized = hex.replace("#", "");
  const parts =
    normalized.length === 3
      ? normalized.split("").map((c) => c + c)
      : [normalized.slice(0, 2), normalized.slice(2, 4), normalized.slice(4, 6)];
  return parts.map((p) => parseInt(p, 16)) as [number, number, number];
}

/**
 * Calculate contrast ratio between two colors.
 */
export function contrastRatio(foreground: string, background: string): ContrastResult {
  const L1 = luminance(foreground) + 0.05;
  const L2 = luminance(background) + 0.05;
  const ratio = L1 > L2 ? L1 / L2 : L2 / L1;
  return {
    foreground,
    background,
    ratio: parseFloat(ratio.toFixed(2)),
    passesAA: ratio >= 4.5,
    passesAAA: ratio >= 7.0,
  };
}

/**
 * Run a minimal audit across declared colors and aria labels.
 */
export function runWcagAudit(params: {
  colorPairs: Array<{ fg: string; bg: string; element: string }>;
  ariaLabels: Array<{ element: string; label?: string }>;
}): AuditReport {
  const issues: AuditIssue[] = [];

  for (const pair of params.colorPairs) {
    const result = contrastRatio(pair.fg, pair.bg);
    if (!result.passesAAA) {
      issues.push({ type: "contrast", detail: result });
    }
  }

  for (const aria of params.ariaLabels) {
    if (!aria.label || aria.label.trim().length === 0) {
      issues.push({
        type: "aria",
        element: aria.element,
        message: "Missing or empty aria-label",
      });
    }
  }

  const summary = {
    total: issues.length,
    contrast: issues.filter((i) => i.type === "contrast").length,
    aria: issues.filter((i) => i.type === "aria").length,
  };

  return { issues, summary };
}
