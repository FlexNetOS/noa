#!/usr/bin/env node

/**
 * Prompt Quality Test Runner (Node.js)
 *
 * Usage:
 *   node test-prompt.js <prompt-file.md>
 *   node test-prompt.js project-mgmt-prompt.md --verbose
 */

const fs = require('fs');
const path = require('path');

// ============ TEST FUNCTIONS ============

function testReadabilityScore(content) {
  const words = content.split(/\s+/).filter(w => w.length > 0).length;
  const sentences = content.split(/[.!?]+/).filter(s => s.trim()).length || 1;
  const avgSentenceLen = words / sentences;
  const score = Math.min(100, Math.max(0, 100 - (avgSentenceLen - 15) * 3));

  return {
    testId: 'CLR-001',
    testName: 'Readability Score',
    category: 'clarity',
    passed: score >= 60,
    score: Math.round(score),
    maxScore: 100,
    details: `Avg sentence: ${avgSentenceLen.toFixed(1)} words (target: ≤25)`
  };
}

function testStructuredFormatting(content) {
  const hasHeadings = /^#{1,4}\s/m.test(content);
  const hasTables = /\|[^|]+\|[^|]+\|/.test(content);
  const hasCode = /```[\s\S]*?```/.test(content);
  const hasLists = /^[\s]*[-*]\s/m.test(content);

  const score = (hasHeadings ? 25 : 0) + (hasTables ? 25 : 0) + (hasCode ? 25 : 0) + (hasLists ? 25 : 0);

  return {
    testId: 'CLR-006',
    testName: 'Structured Formatting',
    category: 'clarity',
    passed: score >= 75,
    score,
    maxScore: 100,
    details: `H:${hasHeadings ? 'Y' : 'N'} T:${hasTables ? 'Y' : 'N'} C:${hasCode ? 'Y' : 'N'} L:${hasLists ? 'Y' : 'N'}`
  };
}

function testAmbiguityCheck(content) {
  const vagueWords = ['should', 'might', 'could', 'possibly', 'maybe', 'perhaps', 'generally'];
  const words = content.toLowerCase().split(/\s+/);
  const vagueCount = words.filter(w => vagueWords.includes(w.replace(/[^a-z]/g, ''))).length;
  const percentage = (vagueCount / Math.max(1, words.length)) * 100;

  return {
    testId: 'CLR-004',
    testName: 'Ambiguity Check',
    category: 'clarity',
    passed: percentage <= 3,
    score: percentage <= 3 ? 100 : Math.round((3 / percentage) * 100),
    maxScore: 100,
    details: `Vague words: ${percentage.toFixed(2)}% (target: ≤3%)`
  };
}

function testMissionStatement(content) {
  const first200 = content.split(/\s+/).slice(0, 200).join(' ').toLowerCase();
  const missionWords = ['mission', 'objective', 'purpose', 'goal', 'consolidate', 'unified', 'create'];
  const found = missionWords.filter(m => first200.includes(m)).length;
  const hasMission = found >= 2;

  return {
    testId: 'CTX-001',
    testName: 'Mission Statement',
    category: 'context',
    passed: hasMission,
    score: hasMission ? 100 : 0,
    maxScore: 100,
    details: hasMission ? `Found ${found} mission indicators` : 'MISSING: Add mission statement'
  };
}

function testScopeDefinition(content) {
  const hasInScope = /in.?scope|includes?|covers|target/i.test(content);
  const hasOutScope = /out.?of.?scope|excludes?|not.?include|non.?goal/i.test(content);
  const score = (hasInScope ? 50 : 0) + (hasOutScope ? 50 : 0);

  return {
    testId: 'CTX-002',
    testName: 'Scope Definition',
    category: 'context',
    passed: score >= 50,
    score,
    maxScore: 100,
    details: `In-scope: ${hasInScope ? 'Y' : 'N'}, Out-of-scope: ${hasOutScope ? 'Y' : 'N'}`
  };
}

function testSuccessCriteria(content) {
  const patterns = [
    /[0-9]+%/g,
    /target[:=]\s*[0-9]+/gi,
    /≥\s*[0-9]+/g,
    /< [0-9]+/g
  ];

  let metricCount = 0;
  patterns.forEach(p => {
    const matches = content.match(p);
    metricCount += matches ? matches.length : 0;
  });

  return {
    testId: 'CTX-006',
    testName: 'Success Criteria',
    category: 'context',
    passed: metricCount >= 5,
    score: Math.min(100, (metricCount / 5) * 100),
    maxScore: 100,
    details: `Found ${metricCount} metrics (target: ≥5)`
  };
}

function testVersionControl(content) {
  const hasVersion = /version[:\s]+[0-9]+\.[0-9]|v[0-9]+\.[0-9]+|Prompt Version/i.test(content);
  const hasChangelog = /changelog|### v[0-9]|## v[0-9]/i.test(content);
  const hasDate = /\d{4}-\d{2}-\d{2}/.test(content);

  const score = (hasVersion ? 40 : 0) + (hasChangelog ? 40 : 0) + (hasDate ? 20 : 0);

  return {
    testId: 'CTX-008',
    testName: 'Version Control',
    category: 'context',
    passed: score >= 80,
    score,
    maxScore: 100,
    details: `Ver:${hasVersion ? 'Y' : 'N'} Log:${hasChangelog ? 'Y' : 'N'} Date:${hasDate ? 'Y' : 'N'}`
  };
}

function testHierarchyDepth(content) {
  const lines = content.split('\n');
  let maxDepth = 0;

  lines.forEach(line => {
    const match = line.match(/^(#+)\s/);
    if (match) {
      maxDepth = Math.max(maxDepth, match[1].length);
    }
  });

  return {
    testId: 'FLW-007',
    testName: 'Hierarchy Depth',
    category: 'flow',
    passed: maxDepth <= 4,
    score: maxDepth <= 4 ? 100 : Math.round((4 / maxDepth) * 100),
    maxScore: 100,
    details: `Max depth: ${maxDepth} (target: ≤4)`
  };
}

function testPipelineCoverage(content) {
  const concepts = ['goal', 'policy', 'rules', 'plan', 'spec', 'task', 'execute', 'memory', 'orchestrat', 'test'];
  const contentLower = content.toLowerCase();
  const found = concepts.filter(c => contentLower.includes(c));
  const coverage = found.length / concepts.length;

  return {
    testId: 'FLW-008',
    testName: 'Pipeline Coverage',
    category: 'flow',
    passed: coverage >= 0.7,
    score: Math.round(coverage * 100),
    maxScore: 100,
    details: `${found.length}/${concepts.length} concepts: ${found.join(', ')}`
  };
}

function testLogicalSequence(content) {
  const progressionKeywords = ['overview', 'introduction', 'architecture', 'implementation', 'testing', 'deployment'];
  const headings = content.split('\n')
    .filter(l => /^#{1,3}\s/.test(l))
    .map(h => h.replace(/^#+\s*/, '').trim().toLowerCase());

  const foundProgression = progressionKeywords.filter(k => headings.some(h => h.includes(k)));
  const hasNumbered = headings.some(h => /^[0-9]+[\.\)]\s/.test(h));
  const score = Math.max((foundProgression.length / progressionKeywords.length) * 100, hasNumbered ? 80 : 0);

  return {
    testId: 'FLW-001',
    testName: 'Logical Sequence',
    category: 'flow',
    passed: score >= 60,
    score: Math.round(score),
    maxScore: 100,
    details: `Found ${foundProgression.length} progression indicators`
  };
}

function testOutputExamples(content) {
  const codeBlocks = content.match(/```[\s\S]*?```/g) || [];
  const tsBlocks = codeBlocks.filter(b => /```(typescript|ts|javascript|js)/i.test(b));
  const yamlBlocks = codeBlocks.filter(b => /```yaml/i.test(b));

  const hasVariety = tsBlocks.length > 0 && yamlBlocks.length > 0;
  const score = Math.min(100, (codeBlocks.length / 3) * 50 + (hasVariety ? 50 : 0));

  return {
    testId: 'RES-001',
    testName: 'Output Examples',
    category: 'results',
    passed: codeBlocks.length >= 3,
    score: Math.round(score),
    maxScore: 100,
    details: `${codeBlocks.length} code blocks (TS: ${tsBlocks.length}, YAML: ${yamlBlocks.length})`
  };
}

function testErrorHandling(content) {
  const keywords = ['error', 'fail', 'exception', 'recovery', 'rollback', 'fallback', 'retry', 'heal'];
  const contentLower = content.toLowerCase();
  const found = keywords.filter(k => contentLower.includes(k));

  return {
    testId: 'RES-002',
    testName: 'Error Handling',
    category: 'results',
    passed: found.length >= 4,
    score: Math.min(100, (found.length / 4) * 100),
    maxScore: 100,
    details: `Error concepts: ${found.join(', ')} (${found.length}/4)`
  };
}

function testValidationRules(content) {
  const hasChecklist = /\[[ x]\]|\- \[|\* \[/i.test(content);
  const hasValidation = /valid|verify|check|confirm|ensure|require/i.test(content);
  const hasCriteria = /criteria|requirement|constraint|must|shall/i.test(content);

  const score = (hasChecklist ? 40 : 0) + (hasValidation ? 30 : 0) + (hasCriteria ? 30 : 0);

  return {
    testId: 'RES-003',
    testName: 'Validation Rules',
    category: 'results',
    passed: score >= 70,
    score,
    maxScore: 100,
    details: `Checklist:${hasChecklist ? 'Y' : 'N'} Valid:${hasValidation ? 'Y' : 'N'} Criteria:${hasCriteria ? 'Y' : 'N'}`
  };
}

function testMetricsDashboard(content) {
  const tableMatches = content.match(/\|[^|\n]+\|[^|\n]+\|/g) || [];
  const metricRows = tableMatches.filter(r => /[0-9]+%|target|score|≥|≤/i.test(r));

  return {
    testId: 'RES-006',
    testName: 'Metrics Dashboard',
    category: 'results',
    passed: metricRows.length >= 8,
    score: Math.min(100, (metricRows.length / 8) * 100),
    maxScore: 100,
    details: `Found ${metricRows.length} metric rows (target: ≥8)`
  };
}

// ============ MAIN TEST RUNNER ============

function runAllTests(content) {
  return [
    // Clarity
    testReadabilityScore(content),
    testStructuredFormatting(content),
    testAmbiguityCheck(content),
    // Context
    testMissionStatement(content),
    testScopeDefinition(content),
    testSuccessCriteria(content),
    testVersionControl(content),
    // Flow
    testHierarchyDepth(content),
    testPipelineCoverage(content),
    testLogicalSequence(content),
    // Results
    testOutputExamples(content),
    testErrorHandling(content),
    testValidationRules(content),
    testMetricsDashboard(content)
  ];
}

function printReport(results, promptFile, verbose) {
  const colors = {
    reset: '\x1b[0m',
    bright: '\x1b[1m',
    green: '\x1b[32m',
    red: '\x1b[31m',
    yellow: '\x1b[33m',
    cyan: '\x1b[36m',
    dim: '\x1b[2m'
  };

  console.log('\n' + '='.repeat(70));
  console.log(`${colors.bright}PROMPT QUALITY TEST REPORT (Node.js)${colors.reset}`);
  console.log('='.repeat(70));
  console.log(`File: ${promptFile}`);
  console.log(`Date: ${new Date().toISOString()}`);
  console.log('-'.repeat(70));

  // Calculate scores by category
  const byCategory = (cat) => results.filter(t => t.category === cat);
  const avgScore = (arr) => arr.reduce((s, t) => s + t.score, 0) / Math.max(1, arr.length);

  const categoryScores = {
    clarity: Math.round(avgScore(byCategory('clarity'))),
    context: Math.round(avgScore(byCategory('context'))),
    flow: Math.round(avgScore(byCategory('flow'))),
    results: Math.round(avgScore(byCategory('results')))
  };

  const overallScore = Math.round(
    (categoryScores.clarity + categoryScores.context + categoryScores.flow + categoryScores.results) / 4
  );
  const passed = overallScore >= 70;

  const scoreColor = passed ? colors.green : colors.red;
  console.log(`${colors.bright}Overall Score: ${scoreColor}${overallScore}/100${colors.reset} ${passed ? 'PASSED' : 'FAILED'}`);

  console.log('\n' + '-'.repeat(70));
  console.log(`${colors.cyan}Category Scores:${colors.reset}`);
  console.log(`  Clarity:  ${categoryScores.clarity}%`);
  console.log(`  Context:  ${categoryScores.context}%`);
  console.log(`  Flow:     ${categoryScores.flow}%`);
  console.log(`  Results:  ${categoryScores.results}%`);

  if (verbose) {
    const categories = ['clarity', 'context', 'flow', 'results'];
    for (const cat of categories) {
      const catTests = byCategory(cat);
      console.log('\n' + '-'.repeat(70));
      console.log(`${colors.cyan}${cat.toUpperCase()} TESTS:${colors.reset}`);
      for (const test of catTests) {
        const statusColor = test.passed ? colors.green : colors.red;
        const statusIcon = test.passed ? '[PASS]' : '[FAIL]';
        console.log(`  ${statusColor}${statusIcon}${colors.reset} [${test.testId}] ${test.testName}: ${test.score}/${test.maxScore}`);
        console.log(`        ${colors.dim}${test.details}${colors.reset}`);
      }
    }
  }

  const recommendations = results.filter(t => !t.passed);
  if (recommendations.length > 0) {
    console.log('\n' + '-'.repeat(70));
    console.log(`${colors.yellow}RECOMMENDATIONS (${recommendations.length}):${colors.reset}`);
    for (const rec of recommendations) {
      console.log(`  * [${rec.testId}] ${rec.testName}: ${rec.details}`);
    }
  }

  console.log('\n' + '='.repeat(70) + '\n');

  return { overallScore, passed, results, categoryScores };
}

// ============ CLI ============

function main() {
  const args = process.argv.slice(2);

  if (args.length === 0 || args.includes('--help')) {
    console.log(`
Prompt Quality Test Runner (Node.js)

Usage:
  node test-prompt.js <prompt-file.md> [options]

Options:
  --verbose    Show detailed test results
  --json       Output as JSON
  --threshold=N  Set pass threshold (default: 70)
  --help       Show this help
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
  const results = runAllTests(content);

  if (jsonOutput) {
    const report = {
      promptFile,
      timestamp: new Date().toISOString(),
      tests: results,
      overallScore: Math.round(results.reduce((s, t) => s + t.score, 0) / results.length),
      passed: results.filter(t => t.passed).length === results.length
    };
    console.log(JSON.stringify(report, null, 2));
  } else {
    const { overallScore } = printReport(results, promptFile, verbose);

    if (overallScore < threshold) {
      process.exit(1);
    }
  }
}

main();

