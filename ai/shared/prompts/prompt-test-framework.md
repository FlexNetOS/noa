# Prompt Test Framework

> **Purpose:** Verify prompt quality through systematic testing of Clarity, Context Engineering, Flow, and Desired Results.

---

## 1) Test Categories

### A. Clarity Tests

| Test ID | Test Name | Description | Pass Criteria |
|---------|-----------|-------------|---------------|
| CLR-001 | Readability Score | Flesch-Kincaid readability | Score ≥ 60 (readable) |
| CLR-002 | Sentence Length | Average words per sentence | ≤ 25 words |
| CLR-003 | Jargon Density | Technical terms with definitions | 100% defined |
| CLR-004 | Ambiguity Check | Vague words (should, might, could) | ≤ 5% of content |
| CLR-005 | Action Clarity | Imperative verbs present | ≥ 1 per section |
| CLR-006 | Numbered Steps | Sequential instructions numbered | 100% numbered |
| CLR-007 | Definition Presence | Key terms defined in glossary | ≥ 95% coverage |

### B. Context Engineering Tests

| Test ID | Test Name | Description | Pass Criteria |
|---------|-----------|-------------|---------------|
| CTX-001 | Mission Statement | Clear primary objective | Present in first 100 words |
| CTX-002 | Scope Definition | Boundaries explicitly stated | Include/Exclude lists present |
| CTX-003 | Audience Identification | Target user/agent defined | Explicitly stated |
| CTX-004 | Prerequisites Listed | Required knowledge/tools | Section present |
| CTX-005 | Constraints Documented | Limitations and boundaries | ≥ 3 constraints listed |
| CTX-006 | Success Criteria | Measurable outcomes | ≥ 5 metrics defined |
| CTX-007 | Dependencies Mapped | External requirements | Table present |
| CTX-008 | Version Control | Version + changelog | Present |

### C. Flow Tests

| Test ID | Test Name | Description | Pass Criteria |
|---------|-----------|-------------|---------------|
| FLW-001 | Logical Sequence | Sections follow logical order | A→B dependency valid |
| FLW-002 | Progressive Complexity | Simple → Complex ordering | Complexity score increases |
| FLW-003 | Cross-References | Internal links work | 100% valid |
| FLW-004 | No Orphan Sections | All sections referenced | 0 orphans |
| FLW-005 | Transition Markers | Section connectors present | ≥ 80% sections |
| FLW-006 | Table of Contents | TOC matches actual sections | 100% match |
| FLW-007 | Hierarchy Depth | Heading levels (H1→H6) | ≤ 4 levels deep |
| FLW-008 | Pipeline Coverage | End-to-end workflow complete | All stages covered |

### D. Desired Results Tests

| Test ID | Test Name | Description | Pass Criteria |
|---------|-----------|-------------|---------------|
| RES-001 | Output Examples | Sample outputs provided | ≥ 3 examples |
| RES-002 | Error Handling | Failure scenarios documented | ≥ 5 scenarios |
| RES-003 | Validation Rules | Output verification criteria | Checklist present |
| RES-004 | Quality Gates | Pass/fail thresholds | Defined for each phase |
| RES-005 | Rollback Procedures | Recovery from failures | Steps documented |
| RES-006 | Metrics Dashboard | KPIs for success | ≥ 10 metrics |
| RES-007 | Test Cases | Executable test scenarios | ≥ 5 test cases |
| RES-008 | Acceptance Criteria | Definition of done | Present per feature |

---

## 2) Automated Test Script

```typescript
// prompt-test.ts - Automated Prompt Quality Verification

interface TestResult {
  testId: string;
  testName: string;
  passed: boolean;
  score: number;
  maxScore: number;
  details: string;
}

interface PromptTestReport {
  promptFile: string;
  timestamp: string;
  overallScore: number;
  categories: {
    clarity: TestResult[];
    context: TestResult[];
    flow: TestResult[];
    results: TestResult[];
  };
  passed: boolean;
  recommendations: string[];
}

class PromptTester {
  private content: string;
  private lines: string[];
  private sections: Map<string, string>;

  constructor(promptContent: string) {
    this.content = promptContent;
    this.lines = promptContent.split('\n');
    this.sections = this.parseSections();
  }

  // ============ CLARITY TESTS ============

  testReadabilityScore(): TestResult {
    // Flesch-Kincaid calculation
    const words = this.content.split(/\s+/).length;
    const sentences = this.content.split(/[.!?]+/).length;
    const syllables = this.countSyllables(this.content);

    const score = 206.835 - 1.015 * (words / sentences) - 84.6 * (syllables / words);
    const passed = score >= 60;

    return {
      testId: 'CLR-001',
      testName: 'Readability Score',
      passed,
      score: Math.round(score),
      maxScore: 100,
      details: `Flesch-Kincaid: ${Math.round(score)} (target: ≥60)`
    };
  }

  testSentenceLength(): TestResult {
    const sentences = this.content.split(/[.!?]+/).filter(s => s.trim());
    const avgLength = sentences.reduce((sum, s) => sum + s.split(/\s+/).length, 0) / sentences.length;
    const passed = avgLength <= 25;

    return {
      testId: 'CLR-002',
      testName: 'Sentence Length',
      passed,
      score: passed ? 100 : Math.round((25 / avgLength) * 100),
      maxScore: 100,
      details: `Average: ${avgLength.toFixed(1)} words/sentence (target: ≤25)`
    };
  }

  testAmbiguityCheck(): TestResult {
    const vagueWords = ['should', 'might', 'could', 'possibly', 'maybe', 'perhaps', 'generally'];
    const words = this.content.toLowerCase().split(/\s+/);
    const vagueCount = words.filter(w => vagueWords.includes(w)).length;
    const percentage = (vagueCount / words.length) * 100;
    const passed = percentage <= 5;

    return {
      testId: 'CLR-004',
      testName: 'Ambiguity Check',
      passed,
      score: passed ? 100 : Math.round((5 / percentage) * 100),
      maxScore: 100,
      details: `Vague words: ${percentage.toFixed(2)}% (target: ≤5%)`
    };
  }

  testActionClarity(): TestResult {
    const imperatives = ['create', 'build', 'implement', 'execute', 'run', 'configure', 'define', 'ensure'];
    const sectionCount = this.sections.size;
    let imperativeSections = 0;

    this.sections.forEach((content) => {
      if (imperatives.some(imp => content.toLowerCase().includes(imp))) {
        imperativeSections++;
      }
    });

    const passed = imperativeSections >= sectionCount * 0.8;

    return {
      testId: 'CLR-005',
      testName: 'Action Clarity',
      passed,
      score: Math.round((imperativeSections / sectionCount) * 100),
      maxScore: 100,
      details: `${imperativeSections}/${sectionCount} sections have imperative verbs`
    };
  }

  // ============ CONTEXT ENGINEERING TESTS ============

  testMissionStatement(): TestResult {
    const first100Words = this.content.split(/\s+/).slice(0, 100).join(' ').toLowerCase();
    const missionIndicators = ['mission', 'objective', 'purpose', 'goal', 'consolidate', 'create', 'build'];
    const hasMission = missionIndicators.some(m => first100Words.includes(m));

    return {
      testId: 'CTX-001',
      testName: 'Mission Statement',
      passed: hasMission,
      score: hasMission ? 100 : 0,
      maxScore: 100,
      details: hasMission ? 'Mission statement found in first 100 words' : 'MISSING: Add mission statement'
    };
  }

  testScopeDefinition(): TestResult {
    const hasInclude = /include|in.?scope|covers/i.test(this.content);
    const hasExclude = /exclude|out.?of.?scope|not.?include/i.test(this.content);
    const passed = hasInclude && hasExclude;

    return {
      testId: 'CTX-002',
      testName: 'Scope Definition',
      passed,
      score: (hasInclude ? 50 : 0) + (hasExclude ? 50 : 0),
      maxScore: 100,
      details: `Include list: ${hasInclude ? '✓' : '✗'}, Exclude list: ${hasExclude ? '✓' : '✗'}`
    };
  }

  testSuccessCriteria(): TestResult {
    const metricsPattern = /\|\s*\*?\*?[A-Za-z\s]+\*?\*?\s*\|\s*[^|]+\|\s*[0-9%<>≥≤]+/g;
    const metrics = this.content.match(metricsPattern) || [];
    const passed = metrics.length >= 5;

    return {
      testId: 'CTX-006',
      testName: 'Success Criteria',
      passed,
      score: Math.min(100, (metrics.length / 5) * 100),
      maxScore: 100,
      details: `Found ${metrics.length} metrics (target: ≥5)`
    };
  }

  testVersionControl(): TestResult {
    const hasVersion = /version[:=\s]+[0-9]+\.[0-9]+/i.test(this.content);
    const hasChangelog = /changelog|## v[0-9]/i.test(this.content);
    const passed = hasVersion && hasChangelog;

    return {
      testId: 'CTX-008',
      testName: 'Version Control',
      passed,
      score: (hasVersion ? 50 : 0) + (hasChangelog ? 50 : 0),
      maxScore: 100,
      details: `Version: ${hasVersion ? '✓' : '✗'}, Changelog: ${hasChangelog ? '✓' : '✗'}`
    };
  }

  // ============ FLOW TESTS ============

  testTableOfContents(): TestResult {
    const headings = this.lines.filter(l => /^#{1,3}\s/.test(l)).map(h => h.replace(/^#+\s*/, '').trim());
    const tocPattern = /\[([^\]]+)\]\(#[^\)]+\)/g;
    const tocEntries = [...this.content.matchAll(tocPattern)].map(m => m[1]);

    if (tocEntries.length === 0) {
      return {
        testId: 'FLW-006',
        testName: 'Table of Contents',
        passed: false,
        score: 0,
        maxScore: 100,
        details: 'No TOC found'
      };
    }

    const matches = tocEntries.filter(t => headings.some(h => h.toLowerCase().includes(t.toLowerCase())));
    const passed = matches.length / tocEntries.length >= 0.9;

    return {
      testId: 'FLW-006',
      testName: 'Table of Contents',
      passed,
      score: Math.round((matches.length / tocEntries.length) * 100),
      maxScore: 100,
      details: `${matches.length}/${tocEntries.length} TOC entries match sections`
    };
  }

  testHierarchyDepth(): TestResult {
    const maxDepth = Math.max(...this.lines
      .filter(l => /^#+\s/.test(l))
      .map(l => (l.match(/^#+/) || [''])[0].length)
    );
    const passed = maxDepth <= 4;

    return {
      testId: 'FLW-007',
      testName: 'Hierarchy Depth',
      passed,
      score: passed ? 100 : Math.round((4 / maxDepth) * 100),
      maxScore: 100,
      details: `Max heading depth: ${maxDepth} (target: ≤4)`
    };
  }

  testPipelineCoverage(): TestResult {
    const requiredStages = [
      'goal', 'policy', 'rules', 'plan', 'spec', 'task', 'execute',
      'memory', 'orchestration', 'feedback'
    ];
    const foundStages = requiredStages.filter(s =>
      this.content.toLowerCase().includes(s)
    );
    const coverage = foundStages.length / requiredStages.length;
    const passed = coverage >= 0.8;

    return {
      testId: 'FLW-008',
      testName: 'Pipeline Coverage',
      passed,
      score: Math.round(coverage * 100),
      maxScore: 100,
      details: `Found ${foundStages.length}/${requiredStages.length} stages: ${foundStages.join(', ')}`
    };
  }

  // ============ DESIRED RESULTS TESTS ============

  testOutputExamples(): TestResult {
    const codeBlocks = this.content.match(/```[\s\S]*?```/g) || [];
    const passed = codeBlocks.length >= 3;

    return {
      testId: 'RES-001',
      testName: 'Output Examples',
      passed,
      score: Math.min(100, (codeBlocks.length / 3) * 100),
      maxScore: 100,
      details: `Found ${codeBlocks.length} code examples (target: ≥3)`
    };
  }

  testErrorHandling(): TestResult {
    const errorKeywords = ['error', 'fail', 'exception', 'recovery', 'rollback', 'fallback'];
    const errorMentions = errorKeywords.filter(k =>
      this.content.toLowerCase().includes(k)
    );
    const passed = errorMentions.length >= 5;

    return {
      testId: 'RES-002',
      testName: 'Error Handling',
      passed,
      score: Math.min(100, (errorMentions.length / 5) * 100),
      maxScore: 100,
      details: `Error handling keywords: ${errorMentions.join(', ')}`
    };
  }

  testValidationRules(): TestResult {
    const hasChecklist = /\[[ x]\]|\- \[|\* \[/i.test(this.content);
    const hasValidation = /valid|verify|check|confirm|ensure/i.test(this.content);
    const passed = hasChecklist && hasValidation;

    return {
      testId: 'RES-003',
      testName: 'Validation Rules',
      passed,
      score: (hasChecklist ? 50 : 0) + (hasValidation ? 50 : 0),
      maxScore: 100,
      details: `Checklist: ${hasChecklist ? '✓' : '✗'}, Validation keywords: ${hasValidation ? '✓' : '✗'}`
    };
  }

  testMetricsDashboard(): TestResult {
    const tableRows = this.content.match(/\|[^|]+\|[^|]+\|[^|]+\|/g) || [];
    const metricsRows = tableRows.filter(r => /[0-9]+%|≥|≤|>|</i.test(r));
    const passed = metricsRows.length >= 10;

    return {
      testId: 'RES-006',
      testName: 'Metrics Dashboard',
      passed,
      score: Math.min(100, (metricsRows.length / 10) * 100),
      maxScore: 100,
      details: `Found ${metricsRows.length} metric rows (target: ≥10)`
    };
  }

  // ============ HELPER METHODS ============

  private parseSections(): Map<string, string> {
    const sections = new Map<string, string>();
    let currentSection = 'intro';
    let currentContent = '';

    for (const line of this.lines) {
      if (/^#{1,3}\s/.test(line)) {
        if (currentContent) {
          sections.set(currentSection, currentContent);
        }
        currentSection = line.replace(/^#+\s*/, '').trim();
        currentContent = '';
      } else {
        currentContent += line + '\n';
      }
    }
    sections.set(currentSection, currentContent);
    return sections;
  }

  private countSyllables(text: string): number {
    const words = text.toLowerCase().split(/\s+/);
    return words.reduce((total, word) => {
      word = word.replace(/[^a-z]/g, '');
      if (word.length <= 3) return total + 1;
      const syllables = word.replace(/(?:[^laeiouy]es|ed|[^laeiouy]e)$/, '')
                           .replace(/^y/, '')
                           .match(/[aeiouy]{1,2}/g);
      return total + (syllables ? syllables.length : 1);
    }, 0);
  }

  // ============ RUN ALL TESTS ============

  runAllTests(): PromptTestReport {
    const results: PromptTestReport = {
      promptFile: 'prompt-under-test.md',
      timestamp: new Date().toISOString(),
      overallScore: 0,
      categories: {
        clarity: [
          this.testReadabilityScore(),
          this.testSentenceLength(),
          this.testAmbiguityCheck(),
          this.testActionClarity()
        ],
        context: [
          this.testMissionStatement(),
          this.testScopeDefinition(),
          this.testSuccessCriteria(),
          this.testVersionControl()
        ],
        flow: [
          this.testTableOfContents(),
          this.testHierarchyDepth(),
          this.testPipelineCoverage()
        ],
        results: [
          this.testOutputExamples(),
          this.testErrorHandling(),
          this.testValidationRules(),
          this.testMetricsDashboard()
        ]
      },
      passed: false,
      recommendations: []
    };

    // Calculate overall score
    const allTests = [
      ...results.categories.clarity,
      ...results.categories.context,
      ...results.categories.flow,
      ...results.categories.results
    ];

    const totalScore = allTests.reduce((sum, t) => sum + t.score, 0);
    const maxScore = allTests.reduce((sum, t) => sum + t.maxScore, 0);
    results.overallScore = Math.round((totalScore / maxScore) * 100);
    results.passed = results.overallScore >= 80;

    // Generate recommendations for failed tests
    allTests.filter(t => !t.passed).forEach(t => {
      results.recommendations.push(`[${t.testId}] ${t.testName}: ${t.details}`);
    });

    return results;
  }
}

// Export for use
export { PromptTester, PromptTestReport, TestResult };
```

---

## 3) Test Execution Commands

```bash
# Run prompt tests via CLI
npm run test:prompt -- --file=project-mgmt-prompt.md

# Run specific category
npm run test:prompt -- --file=project-mgmt-prompt.md --category=clarity

# Generate HTML report
npm run test:prompt -- --file=project-mgmt-prompt.md --report=html

# CI/CD integration
npm run test:prompt -- --file=project-mgmt-prompt.md --ci --threshold=80
```

---

## 4) Sample Test Output

```
╔══════════════════════════════════════════════════════════════════╗
║                    PROMPT TEST REPORT                             ║
╠══════════════════════════════════════════════════════════════════╣
║ File: project-mgmt-prompt.md                                     ║
║ Date: 2025-12-04T12:00:00Z                                       ║
║ Overall Score: 87/100 ✓ PASSED                                   ║
╠══════════════════════════════════════════════════════════════════╣

CLARITY TESTS (4/4 passed)
┌─────────┬────────────────────┬────────┬───────────────────────────┐
│ Test ID │ Test Name          │ Score  │ Details                   │
├─────────┼────────────────────┼────────┼───────────────────────────┤
│ CLR-001 │ Readability Score  │ 72/100 │ Flesch-Kincaid: 72        │
│ CLR-002 │ Sentence Length    │ 100    │ Avg: 18.3 words           │
│ CLR-004 │ Ambiguity Check    │ 96/100 │ Vague words: 2.1%         │
│ CLR-005 │ Action Clarity     │ 100    │ 25/25 sections            │
└─────────┴────────────────────┴────────┴───────────────────────────┘

CONTEXT ENGINEERING TESTS (4/4 passed)
┌─────────┬────────────────────┬────────┬───────────────────────────┐
│ Test ID │ Test Name          │ Score  │ Details                   │
├─────────┼────────────────────┼────────┼───────────────────────────┤
│ CTX-001 │ Mission Statement  │ 100    │ Found in first 100 words  │
│ CTX-002 │ Scope Definition   │ 100    │ Include ✓, Exclude ✓      │
│ CTX-006 │ Success Criteria   │ 100    │ Found 22 metrics          │
│ CTX-008 │ Version Control    │ 100    │ Version ✓, Changelog ✓    │
└─────────┴────────────────────┴────────┴───────────────────────────┘

FLOW TESTS (3/3 passed)
┌─────────┬────────────────────┬────────┬───────────────────────────┐
│ Test ID │ Test Name          │ Score  │ Details                   │
├─────────┼────────────────────┼────────┼───────────────────────────┤
│ FLW-006 │ Table of Contents  │ 95/100 │ 19/20 entries match       │
│ FLW-007 │ Hierarchy Depth    │ 100    │ Max depth: 3              │
│ FLW-008 │ Pipeline Coverage  │ 90/100 │ 9/10 stages found         │
└─────────┴────────────────────┴────────┴───────────────────────────┘

DESIRED RESULTS TESTS (3/4 passed)
┌─────────┬────────────────────┬────────┬───────────────────────────┐
│ Test ID │ Test Name          │ Score  │ Details                   │
├─────────┼────────────────────┼────────┼───────────────────────────┤
│ RES-001 │ Output Examples    │ 100    │ Found 45 code blocks      │
│ RES-002 │ Error Handling     │ 80/100 │ 4/5 error keywords        │
│ RES-003 │ Validation Rules   │ 100    │ Checklist ✓, Validation ✓ │
│ RES-006 │ Metrics Dashboard  │ 100    │ Found 26 metric rows      │
└─────────┴────────────────────┴────────┴───────────────────────────┘

RECOMMENDATIONS:
  None - All critical tests passed!

╚══════════════════════════════════════════════════════════════════╝
```

---

## 5) Integration with CI/CD

```yaml
# .github/workflows/prompt-quality.yml
name: Prompt Quality Gate

on:
  push:
    paths:
      - 'ai/shared/prompts/**/*.md'
  pull_request:
    paths:
      - 'ai/shared/prompts/**/*.md'

jobs:
  test-prompts:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '20'

      - name: Install dependencies
        run: npm ci

      - name: Run Prompt Tests
        run: |
          npm run test:prompt -- \
            --file=ai/shared/prompts/project-mgmt-prompt.md \
            --threshold=80 \
            --report=json > prompt-test-results.json

      - name: Upload Test Results
        uses: actions/upload-artifact@v4
        with:
          name: prompt-test-results
          path: prompt-test-results.json

      - name: Fail on Low Score
        run: |
          SCORE=$(jq '.overallScore' prompt-test-results.json)
          if [ "$SCORE" -lt 80 ]; then
            echo "Prompt quality score ($SCORE) below threshold (80)"
            exit 1
          fi
```

---

## 6) Quick Test Checklist (Manual)

Use this checklist to manually verify a prompt:

### Clarity ✓
- [ ] Can a new team member understand the purpose in < 2 minutes?
- [ ] Are all technical terms defined or linked to definitions?
- [ ] Are instructions specific and actionable (not vague)?
- [ ] Is formatting consistent (headings, bullets, code blocks)?

### Context Engineering ✓
- [ ] Is the mission/objective stated clearly upfront?
- [ ] Are boundaries (in-scope/out-of-scope) explicitly defined?
- [ ] Are all dependencies and prerequisites listed?
- [ ] Is version information and changelog present?

### Flow ✓
- [ ] Do sections follow a logical progression?
- [ ] Can you follow from start to finish without jumping around?
- [ ] Are cross-references valid and helpful?
- [ ] Is there a clear entry point and exit criteria?

### Desired Results ✓
- [ ] Are expected outputs clearly described with examples?
- [ ] Are error scenarios and recovery procedures documented?
- [ ] Are success metrics defined and measurable?
- [ ] Can results be validated against the criteria?

---

**Framework Version:** 1.0
**Created:** 2025-12-04
**Maintainer:** Project Team
