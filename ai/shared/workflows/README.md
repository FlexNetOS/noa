# Workflows Directory

This directory contains orchestration workflow definitions for multi-agent coordination.

## Purpose

Workflows define how multiple agents collaborate to complete complex tasks.
They specify the sequence of agent interactions, data flow, and decision points.

## Workflow Definition Format

Workflows are defined in YAML for readability:

```yaml
$schema: https://noa.local/schemas/workflow.yaml
name: code-review-workflow
version: "1.0.0"
description: Automated code review with multiple perspectives

triggers:
  - event: pr_opened
  - event: pr_updated
  - manual: true

steps:
  - id: analyze-changes
    agent: code-analysis-agent
    input: ${trigger.diff}
    output: analysis_result

  - id: security-review
    agent: security-agent
    input: ${analysis_result}
    output: security_findings
    parallel: true

  - id: style-check
    agent: style-agent
    input: ${analysis_result}
    output: style_findings
    parallel: true

  - id: synthesize-review
    agent: reasoning-agent
    input:
      analysis: ${analysis_result}
      security: ${security_findings}
      style: ${style_findings}
    output: final_review

outputs:
  review_summary: ${final_review.summary}
  recommendations: ${final_review.recommendations}
  approval_status: ${final_review.can_merge}
```

## Workflow Types

1. **Sequential** - Agents execute in order, each receiving previous output
2. **Parallel** - Multiple agents execute simultaneously on same input
3. **Conditional** - Branching based on intermediate results
4. **Loop** - Iterative refinement until condition met

## Creating a New Workflow

1. Create a new YAML file: `my-workflow.yaml`
2. Define triggers (events or manual)
3. Specify steps with agent references
4. Define inputs/outputs for each step
5. Register in `../resources/resource-registry.json`

## Related Files

- `../agents/` - Agent definitions referenced in workflows
- `../resources/execution-memory.db` - Workflow execution state
- `../tools/` - Tools available within workflows

