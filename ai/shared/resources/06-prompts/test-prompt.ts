#!/usr/bin/env npx ts-node

/**
 * Prompt Quality Test Runner
 *
 * Usage:
 *   npx ts-node test-prompt.ts <prompt-file.md>
 *   npx ts-node test-prompt.ts project-mgmt-prompt.md --verbose
 *   npx ts-node test-prompt.ts project-mgmt-prompt.md --json
 */

import * as fs from 'fs';
import * as path from 'path';

// ============ INTERFACES ============

interface TestResult {
  testId: string;
  testName: string;
  category: 'clarity' | 'context' | 'flow' | 'results';
  passed: boolean;
  score: number;
  maxScore: number;
  details: string;
}

interface PromptTestReport {
  promptFile: string;
  timestamp: string;
  overallScore: number;
  categoryScores: {
    clarity: number;
    context: number;
    flow: number;
    results: number;
  };
  tests: TestResult[];
  passed: boolean;
  recommendations: string[];
}

// ============ PROMPT TESTER CLASS ============

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
    const words = this.content.split(/\s+/).filter(w => w.length > 0).length;
    const sentences = this.content.split(/[.!?]+/).filter(s => s.trim()).length;
    const syllables = this.countSyllables(this.content);

    const score = Math.max(0, Math.min(100,
      206.835 - 1.015 * (words / Math.max(1, sentences)) - 84.6 * (syllables / Math.max(1, words))
    ));
    const passed = score >= 40; // Adjusted for technical content

    return {
      testId: 'CLR-001',
      testName: 'Readability Score',
      category: 'clarity',
      passed,
      score: Math.round(score),
      maxScore: 100,
      details: `Flesch-Kincaid: ${Math.round(score)} (target: ≥40 for technical)`
    };
  }

  testSentenceLength(): TestResult {
    const sentences = this.content.split(/[.!?]+/).filter(s => s.trim().length > 10);
    if (sentences.length === 0) {
      return { testId: 'CLR-002', testName: 'Sentence Length', category: 'clarity', passed: true, score: 100, maxScore: 100, details: 'N/A - No sentences' };
    }
    const avgLength = sentences.reduce((sum, s) => sum + s.split(/\s+/).length, 0) / sentences.length;
    const passed = avgLength <= 30;

    return {
      testId: 'CLR-002',
      testName: 'Sentence Length',
      category: 'clarity',
      passed,
      score: passed ? 100 : Math.round((30 / avgLength) * 100),
      maxScore: 100,
      details: `Average: ${avgLength.toFixed(1)} words/sentence (target: ≤30)`
    };
  }

  testAmbiguityCheck(): TestResult {
    const vagueWords = ['should', 'might', 'could', 'possibly', 'maybe', 'perhaps', 'generally', 'usually', 'sometimes'];
    const words = this.content.toLowerCase().split(/\s+/);
    const vagueCount = words.filter(w => vagueWords.includes(w.replace(/[^a-z]/g, ''))).length;
    const percentage = (vagueCount / Math.max(1, words.length)) * 100;
    const passed = percentage <= 3;

    return {
      testId: 'CLR-004',
      testName: 'Ambiguity Check',
      category: 'clarity',
      passed,
      score: passed ? 100 : Math.round((3 / percentage) * 100),
      maxScore: 100,
      details: `Vague words: ${percentage.toFixed(2)}% (target: ≤3%)`
    };
  }

  testStructuredFormatting(): TestResult {
    const hasHeadings = /^#{1,4}\s/m.test(this.content);
    const hasTables = /\|[^|]+\|[^|]+\|/.test(this.content);
    const hasCodeBlocks = /```[\s\S]*?```/.test(this.content);
    const hasLists = /^[\s]*[-*]\s/m.test(this.content);

    const score = (hasHeadings ? 25 : 0) + (hasTables ? 25 : 0) + (hasCodeBlocks ? 25 : 0) + (hasLists ? 25 : 0);
    const passed = score >= 75;

    return {
      testId: 'CLR-006',
      testName: 'Structured Formatting',
      category: 'clarity',
      passed,
      score,
      maxScore: 100,
      details: `Headings: ${hasHeadings ? '✓' : '✗'}, Tables: ${hasTables ? '✓' : '✗'}, Code: ${hasCodeBlocks ? '✓' : '✗'}, Lists: ${hasLists ? '✓' : '✗'}`
    };
  }

  // ============ CONTEXT ENGINEERING TESTS ============

  testMissionStatement(): TestResult {
    const first200Words = this.content.split(/\s+/).slice(0, 200).join(' ').toLowerCase();
    const missionIndicators = ['mission', 'objective', 'purpose', 'goal', 'consolidate', 'create', 'build', 'unified', 'merge'];
    const hasMission = missionIndicators.filter(m => first200Words.includes(m)).length >= 2;

    return {
      testId: 'CTX-001',
      testName: 'Mission Statement',
      category: 'context',
      passed: hasMission,
      score: hasMission ? 100 : 0,
      maxScore: 100,
      details: hasMission ? 'Mission indicators found in opening' : 'MISSING: Add clear mission statement'
    };
  }

  testScopeDefinition(): TestResult {
    const hasInScope = /in.?scope|includes?|covers|target/i.test(this.content);
    const hasOutScope = /out.?of.?scope|excludes?|not.?include|non.?goal/i.test(this.content);
    const passed = hasInScope && hasOutScope;

    return {
      testId: 'CTX-002',
      testName: 'Scope Definition',
      category: 'context',
      passed,
      score: (hasInScope ? 50 : 0) + (hasOutScope ? 50 : 0),
      maxScore: 100,
      details: `In-scope: ${hasInScope ? '✓' : '✗'}, Out-of-scope: ${hasOutScope ? '✓' : '✗'}`
    };
  }

  testSuccessCriteria(): TestResult {
    const metricsPatterns = [
      /\|\s*[^|]+\s*\|\s*[^|]+\s*\|\s*[0-9]+%/g,
      /target[:=]\s*[0-9]+/gi,
      /≥\s*[0-9]+/g,
      /100%/g,
      /< [0-9]+/g
    ];

    let metricCount = 0;
    metricsPatterns.forEach(pattern => {
      const matches = this.content.match(pattern);
      metricCount += matches ? matches.length : 0;
    });

    const passed = metricCount >= 5;

    return {
      testId: 'CTX-006',
      testName: 'Success Criteria',
      category: 'context',
      passed,
      score: Math.min(100, (metricCount / 5) * 100),
      maxScore: 100,
      details: `Found ${metricCount} metrics/targets (target: ≥5)`
    };
  }

  testVersionControl(): TestResult {
    const hasVersion = /\*?\*?version\*?\*?[:=\s]+[0-9]+\.[0-9]+/i.test(this.content);
    const hasChangelog = /changelog|##\s*v[0-9]|###\s*v[0-9]/i.test(this.content);
    const hasDate = /\d{4}-\d{2}-\d{2}/.test(this.content);

    const score = (hasVersion ? 40 : 0) + (hasChangelog ? 40 : 0) + (hasDate ? 20 : 0);
    const passed = score >= 80;

    return {
      testId: 'CTX-008',
      testName: 'Version Control',
      category: 'context',
      passed,
      score,
      maxScore: 100,
      details: `Version: ${hasVersion ? '✓' : '✗'}, Changelog: ${hasChangelog ? '✓' : '✗'}, Date: ${hasDate ? '✓' : '✗'}`
    };
  }

  // ============ FLOW TESTS ============

  testLogicalSequence(): TestResult {
    const headings = this.lines
      .filter(l => /^#{1,3}\s/.test(l))
      .map(h => h.replace(/^#+\s*/, '').trim().toLowerCase());

    // Check for logical progression keywords
    const progressionKeywords = ['overview', 'introduction', 'architecture', 'implementation', 'testing', 'deployment', 'conclusion'];
    const foundProgression = progressionKeywords.filter(k =>
      headings.some(h => h.includes(k))
    );

    const hasNumberedSections = headings.some(h => /^[0-9]+[\.\)]\s/.test(h));
    const score = Math.max(
      (foundProgression.length / progressionKeywords.length) * 100,
      hasNumberedSections ? 80 : 0
    );
    const passed = score >= 60;

    return {
      testId: 'FLW-001',
      testName: 'Logical Sequence',
      category: 'flow',
      passed,
      score: Math.round(score),
      maxScore: 100,
      details: `Found ${foundProgression.length} progression indicators or numbered sections`
    };
  }

  testCrossReferences(): TestResult {
    const internalLinks = this.content.match(/\[([^\]]+)\]\(#[^)]+\)/g) || [];
    const headingAnchors = this.lines
      .filter(l => /^#{1,4}\s/.test(l))
      .map(h => h.replace(/^#+\s*/, '').trim().toLowerCase().replace(/[^a-z0-9]+/g, '-'));

    if (internalLinks.length === 0) {
      return {
        testId: 'FLW-003',
        testName: 'Cross-References',
        category: 'flow',
        passed: true,
        score: 70, // Partial credit - links not required but helpful
        maxScore: 100,
        details: 'No internal links (optional but recommended)'
      };
    }

    const validLinks = internalLinks.filter(link => {
      const anchor = link.match(/\(#([^)]+)\)/)?.[1]?.toLowerCase() || '';
      return headingAnchors.some(h => h.includes(anchor) || anchor.includes(h));
    });

    const score = (validLinks.length / internalLinks.length) * 100;
    const passed = score >= 80;

    return {
      testId: 'FLW-003',
      testName: 'Cross-References',
      category: 'flow',
      passed,
      score: Math.round(score),
      maxScore: 100,
      details: `${validLinks.length}/${internalLinks.length} internal links valid`
    };
  }

  testHierarchyDepth(): TestResult {
    const depths = this.lines
      .filter(l => /^#+\s/.test(l))
      .map(l => (l.match(/^#+/) || [''])[0].length);

    const maxDepth = depths.length > 0 ? Math.max(...depths) : 1;
    const passed = maxDepth <= 4;

    return {
      testId: 'FLW-007',
      testName: 'Hierarchy Depth',
      category: 'flow',
      passed,
      score: passed ? 100 : Math.round((4 / maxDepth) * 100),
      maxScore: 100,
      details: `Max heading depth: ${maxDepth} (target: ≤4)`
    };
  }

  testPipelineCoverage(): TestResult {
    const requiredConcepts = [
      'goal', 'policy', 'rules', 'plan', 'spec', 'task',
      'execute', 'memory', 'orchestrat', 'feedback', 'test'
    ];
    const contentLower = this.content.toLowerCase();
    const foundConcepts = requiredConcepts.filter(c => contentLower.includes(c));
    const coverage = foundConcepts.length / requiredConcepts.length;
    const passed = coverage >= 0.7;

    return {
      testId: 'FLW-008',
      testName: 'Pipeline Coverage',
      category: 'flow',
      passed,
      score: Math.round(coverage * 100),
      maxScore: 100,
      details: `${foundConcepts.length}/${requiredConcepts.length} concepts: ${foundConcepts.join(', ')}`
    };
  }

  // ============ DESIRED RESULTS TESTS ============

  testOutputExamples(): TestResult {
    const codeBlocks = this.content.match(/```[\s\S]*?```/g) || [];
    const typescriptBlocks = codeBlocks.filter(b => /```(typescript|ts|javascript|js)/i.test(b));
    const yamlBlocks = codeBlocks.filter(b => /```yaml/i.test(b));
    const bashBlocks = codeBlocks.filter(b => /```(bash|sh|shell)/i.test(b));

    const hasVariety = typescriptBlocks.length > 0 && (yamlBlocks.length > 0 || bashBlocks.length > 0);
    const passed = codeBlocks.length >= 3 && hasVariety;

    return {
      testId: 'RES-001',
      testName: 'Output Examples',
      category: 'results',
      passed,
      score: Math.min(100, (codeBlocks.length / 3) * 50 + (hasVariety ? 50 : 0)),
      maxScore: 100,
      details: `${codeBlocks.length} code blocks (TS: ${typescriptBlocks.length}, YAML: ${yamlBlocks.length}, Bash: ${bashBlocks.length})`
    };
  }

  testErrorHandling(): TestResult {
    const errorKeywords = ['error', 'fail', 'exception', 'recovery', 'rollback', 'fallback', 'retry', 'heal'];
    const contentLower = this.content.toLowerCase();
    const errorMentions = errorKeywords.filter(k => contentLower.includes(k));
    const passed = errorMentions.length >= 4;

    return {
      testId: 'RES-002',
      testName: 'Error Handling',
      category: 'results',
      passed,
      score: Math.min(100, (errorMentions.length / 4) * 100),
      maxScore: 100,
      details: `Error concepts: ${errorMentions.join(', ')} (${errorMentions.length}/4)`
    };
  }

  testValidationRules(): TestResult {
    const hasChecklist = /\[[ x]\]|\- \[|\* \[/i.test(this.content);
    const hasValidation = /valid|verify|check|confirm|ensure|require/i.test(this.content);
    const hasCriteria = /criteria|requirement|constraint|must|shall/i.test(this.content);

    const score = (hasChecklist ? 40 : 0) + (hasValidation ? 30 : 0) + (hasCriteria ? 30 : 0);
    const passed = score >= 70;

    return {
      testId: 'RES-003',
      testName: 'Validation Rules',
      category: 'results',
      passed,
      score,
      maxScore: 100,
      details: `Checklist: ${hasChecklist ? '✓' : '✗'}, Validation: ${hasValidation ? '✓' : '✗'}, Criteria: ${hasCriteria ? '✓' : '✗'}`
    };
  }

  testMetricsDashboard(): TestResult {
    const tableRows = this.content.match(/\|[^|\n]+\|[^|\n]+\|[^|\n]+\|/g) || [];
    const metricsRows = tableRows.filter(r => /[0-9]+%|≥|≤|>|<|target|score/i.test(r));
    const passed = metricsRows.length >= 8;

    return {
      testId: 'RES-006',
      testName: 'Metrics Dashboard',
      category: 'results',
      passed,
      score: Math.min(100, (metricsRows.length / 8) * 100),
      maxScore: 100,
      details: `Found ${metricsRows.length} metric rows (target: ≥8)`
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
    const tests: TestResult[] = [
      // Clarity
      this.testReadabilityScore(),
      this.testSentenceLength(),
      this.testAmbiguityCheck(),
      this.testStructuredFormatting(),
      // Context
      this.testMissionStatement(),
      this.testScopeDefinition(),
      this.testSuccessCriteria(),
      this.testVersionControl(),
      // Flow
      this.testLogicalSequence(),
      this.testCrossReferences(),
      this.testHierarchyDepth(),
      this.testPipelineCoverage(),
      // Results
      this.testOutputExamples(),
      this.testErrorHandling(),
      this.testValidationRules(),
      this.testMetricsDashboard()
    ];

    // Calculate category scores
    const byCategory = (cat: string) => tests.filter(t => t.category === cat);
    const avgScore = (arr: TestResult[]) => arr.reduce((s, t) => s + t.score, 0) / Math.max(1, arr.length);

    const categoryScores = {
      clarity: Math.round(avgScore(byCategory('clarity'))),
      context: Math.round(avgScore(byCategory('context'))),
      flow: Math.round(avgScore(byCategory('flow'))),
      results: Math.round(avgScore(byCategory('results')))
    };

    const overallScore = Math.round(
      (categoryScores.clarity + categoryScores.context + categoryScores.flow + categoryScores.results) / 4
    );

    const recommendations = tests
      .filter(t => !t.passed)
      .map(t => `[${t.testId}] ${t.testName}: ${t.details}`);

    return {
      promptFile: 'test',
      timestamp: new Date().toISOString(),
      overallScore,
      categoryScores,
      tests,
      passed: overallScore >= 70,
      recommendations
    };
  }
}

// ============ CLI INTERFACE ============

function printReport(report: PromptTestReport, verbose: boolean = false): void {
  const colors = {
    reset: '\x1b[0m',
    bright: '\x1b[1m',
    green: '\x1b[32m',
    red: '\x1b[31m',
    yellow: '\x1b[33m',
    cyan: '\x1b[36m'
  };

  console.log('\n' + '═'.repeat(70));
  console.log(`${colors.bright}PROMPT QUALITY TEST REPORT${colors.reset}`);
  console.log('═'.repeat(70));
  console.log(`File: ${report.promptFile}`);
  console.log(`Date: ${report.timestamp}`);
  console.log('─'.repeat(70));

  const scoreColor = report.passed ? colors.green : colors.red;
  console.log(`${colors.bright}Overall Score: ${scoreColor}${report.overallScore}/100${colors.reset} ${report.passed ? '✓ PASSED' : '✗ FAILED'}`);

  console.log('\n' + '─'.repeat(70));
  console.log(`${colors.cyan}Category Scores:${colors.reset}`);
  console.log(`  Clarity:  ${report.categoryScores.clarity}%`);
  console.log(`  Context:  ${report.categoryScores.context}%`);
  console.log(`  Flow:     ${report.categoryScores.flow}%`);
  console.log(`  Results:  ${report.categoryScores.results}%`);

  if (verbose) {
    const categories = ['clarity', 'context', 'flow', 'results'] as const;
    for (const cat of categories) {
      const catTests = report.tests.filter(t => t.category === cat);
      console.log('\n' + '─'.repeat(70));
      console.log(`${colors.cyan}${cat.toUpperCase()} TESTS:${colors.reset}`);
      for (const test of catTests) {
        const status = test.passed ? `${colors.green}✓${colors.reset}` : `${colors.red}✗${colors.reset}`;
        console.log(`  ${status} [${test.testId}] ${test.testName}: ${test.score}/${test.maxScore}`);
        console.log(`      ${test.details}`);
      }
    }
  }

  if (report.recommendations.length > 0) {
    console.log('\n' + '─'.repeat(70));
    console.log(`${colors.yellow}RECOMMENDATIONS (${report.recommendations.length}):${colors.reset}`);
    for (const rec of report.recommendations) {
      console.log(`  • ${rec}`);
    }
  }

  console.log('\n' + '═'.repeat(70) + '\n');
}

// ============ MAIN ============

async function main() {
  const args = process.argv.slice(2);

  if (args.length === 0 || args.includes('--help')) {
    console.log(`
Prompt Quality Test Runner

Usage:
  npx ts-node test-prompt.ts <prompt-file.md> [options]

Options:
  --verbose    Show detailed test results
  --json       Output as JSON
  --threshold=N  Set pass threshold (default: 70)
  --help       Show this help

Examples:
  npx ts-node test-prompt.ts project-mgmt-prompt.md
  npx ts-node test-prompt.ts project-mgmt-prompt.md --verbose
  npx ts-node test-prompt.ts project-mgmt-prompt.md --json --threshold=80
`);
    process.exit(0);
  }

  const promptFile = args.find(a => !a.startsWith('--')) || '';
  const verbose = args.includes('--verbose');
  const jsonOutput = args.includes('--json');
  const thresholdArg = args.find(a => a.startsWith('--threshold='));
  const threshold = thresholdArg ? parseInt(thresholdArg.split('=')[1]) : 70;

  // Resolve file path
  let filePath = promptFile;
  if (!path.isAbsolute(filePath)) {
    filePath = path.join(process.cwd(), filePath);
  }

  if (!fs.existsSync(filePath)) {
    console.error(`Error: File not found: ${filePath}`);
    process.exit(1);
  }

  const content = fs.readFileSync(filePath, 'utf-8');
  const tester = new PromptTester(content);
  const report = tester.runAllTests();
  report.promptFile = promptFile;

  if (jsonOutput) {
    console.log(JSON.stringify(report, null, 2));
  } else {
    printReport(report, verbose);
  }

  // Exit with error code if below threshold
  if (report.overallScore < threshold) {
    process.exit(1);
  }
}

main().catch(err => {
  console.error('Error:', err.message);
  process.exit(1);
});

export { PromptTester, PromptTestReport, TestResult };
