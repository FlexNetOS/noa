# AGENTS.md - AI Agent Guidelines for PRP

This document provides specific instructions for AI coding assistants (Claude, GPT-4, Copilot, etc.) working on the PRP project. These guidelines supplement the CLAUDE.md development guide with **mandatory policies** that must be followed.

**Created by**: Vasilisa Versus
**Project Goal**: Bootstrap context-driven development workflow based on Product Requirement Prompts (PRPs) and orchestrate execution with LOOP MODE.

---
> SYSTEM PART! NEVER EDIT THIS PART! USER SECTION BELOW!

---

## 🚀 SACRED RULES (Never Violate)

1. **PRP-First Development**: All progress and reports MUST be commented in PRP files. No exceptions.
2. **Signal-Driven Progress**: Every completed job MUST be noted with comment about work done and corresponding signal in related PRP progress.
3. **PRP reporting**: Always read PRP first, work only within PRP scope, leave comment and signal before context compaction or then progress happen.
4. **No orphan files**: Never create tmp/scripts/md files without deleting them right after. All tmp files - write about it in PRP first!
5. **No Paperovers**: Never use `--no-verify`, `--force`, or disable linting. Instead, comment signal describing the issue and work on solutions.
6. **Cleanup Responsibility**: Any `/tmp`, dev servers, ports, or external resources MUST be documented in PRP for cleanup.
7. **Low Confidence Handling**: Before any uncertain action, leave comment explaining risk and wait for guidance.

---

## 🔄 WORKFLOW

### **PRP Creation & Analysis**
- Research problem domain - robo-system-analyst investigates requirements
- Draft complete PRP - Include DoR, DoD, acceptance criteria
- Review with team - Developer and QA provide feedback
- Prioritize work - Orchestrator schedules implementation
**Outcomes**: Goal clarification, goal not achievable, ready for preparation, validation required

### **Preparation & Planning**
- Refine requirements - Break down into implementable tasks with plan how to validate result after
- Create implementation plan - Define task sequence and dependencies
- Estimate effort - can be PRP done at once? or need arrange a several PR with milestones and checkpoints?
- Validate approach - Ensure technical feasibility
- Write down affected files list - parallel agent working and proper code review description should always rely on file list. We always during implementation working only with prp related files
**Outcomes**: Research request, verification plan, implementation plan ready, experiment required

### **Implementation**
- TDD approach - Write tests before implementation
- Development progress - Incremental commits with clear progression
- Handle blockers - Identify and resolve technical dependencies
- Research requests - Address unknowns or gaps in knowledge
- Prp scope - We working only with prp related files, need edit or create file? then update PRP first!
**Outcomes**: Tests prepared, development progress, blocker resolved, research completed

### **Verification & Testing**
- Test execution - robo-aqa runs comprehensive test suite
- Bug handling - Identify, fix, and verify bug resolution
- Code quality - Ensure quality standards and linting pass
- CI/CD validation - Automated testing and deployment pipeline
- Never trust code - Always rely on behavior
**Outcomes**: Tests written, bugs fixed, quality passed, CI passed, tests failed, CI failed, pre-release checklist completed, PR created, review progressed, cleanup done, review passed

### **Release & Deployment**
- Implementation verification - Confirm requirements met
- Release approval - Get authorization for deployment
- Merge & release - Deploy changes to production
- Post-release check - Verify deployment success
**Outcomes**: Implementation verified, release approved, merged, released

### **Post-Release**
- Post-release validation - Monitor system health and user feedback
- Incident handling - Address any production issues
- Post-mortem analysis - Document lessons learned
- Implementation verification - Confirm deployment goals achieved
**Outcomes**: Post-release checked, incident occurred, incident resolved, post-mortem written in PRP, implementation verified

---

## 🎵 ♫ SIGNAL SYSTEM

PRP is a place where we keeping our actual work progress status and next steps. We using special signals to communicate and push forward work. ALWAYS after some progress done leave details as comments and signal related to situation in PRP you workin on;

ALL PRPs/*.md should satisfy following structure:
```md
# prp-name

> prp main goal, or original user request

## progress
signal | comment | time | role-name (model name)
[FF], AGENT ALWAYS LEFT COMMENT HERE WHILE WORK, now, ADMIN
...

## dod
- [ ] always one by line, mesurable and possible to verification

## dor
- [ ] each should by prepared during robo-system-analyst work

## pre-release checklist
- [ ] should be prepared before implementation and executed before pr

## post-release checklist
- [ ] should be prepared before implementation and executed after release confirmed

## plan
- [ ] one line per one file change we make and what we want do do, below can contain some details in sub ###

## research materials
- url...
```


### **System Signals (Using internaly)**
**[HF]** - Health Feedback (orchestration cycle start)
**[pr]** - Pull Request Preparation (optimization pre-catch)
**[PR]** - Pull Request Created (PR activity detected)
**[FF]** - System Fatal Error (corruption/unrecoverable errors)
**[TF]** - Terminal Closed (graceful session end)
**[TC]** - Terminal Crushed (process crash)
**[TI]** - Terminal Idle (inactivity timeout)

### **Agent Signals (should be always found in PRP)**

#### [bb] Blocker
- **WHO**: Any Robo-Agent
- **WHEN**: Technical dependency, configuration, or external requirement blocks progress
- **WHAT**: Document blocker details in PRP, specify unblocking actions needed, continue with other tasks

#### [af] Feedback Request
- **WHO**: Any Robo-Agent
- **WHEN**: Decision needed on design approach, implementation strategy, or requirement interpretation
- **WHAT**: Provide context and options in PRP, request specific guidance, wait for  direction before proceeding

#### [gg] Goal Clarification
- **WHO**: Robo-System-Analyst
- **WHEN**: PRP requirements are ambiguous, conflicting, or insufficient for implementation
- **WHAT**: Ask specific clarifying questions, propose requirement refinements, update PRP with clarified scope

#### [ff] Goal Not Achievable
- **WHO**: Robo-System-Analyst
- **WHEN**: Analysis shows PRP goals cannot be achieved with current constraints/technology
- **WHAT**: Document impossibility analysis, propose alternative approaches or modified goals, update PRP

#### [dA] Done Assessment
- **WHO**: Any Robo-Agent
- **WHEN**: Task or milestone completed, ready for Definition of Done validation
- **WHAT**: Provide completion evidence in PRP, reference DoD criteria, request validation before proceeding to next phase

#### [no] Not Obvious
- **WHO**: Any Robo-Agent
- **WHEN**: Implementation complexity, technical uncertainty, or unknown dependencies discovered
- **WHAT**: Document complexity details, request research time or clarification, wait for analysis before proceeding

#### [rp] Ready for Preparation
- **WHO**: Robo-System-Analyst
- **WHEN**: PRP analysis complete, requirements clear, ready to move to planning phase
- **WHAT**: Signal completion of analysis phase, transition PRP status to preparation, trigger planning workflow

#### [vr] Validation Required
- **WHO**: Robo-System-Analyst
- **WHEN**: PRP needs external validation, stakeholder approval, or compliance review before proceeding
- **WHAT**: Document validation requirements, specify validators needed, pause workflow until validation received

#### [rr] Research Request
- **WHO**: Any Robo-Agent
- **WHEN**: Unknown dependencies, technology gaps, or market research needed to proceed
- **WHAT**: Document research questions, estimate research time, request robo-system-analyst research assignment

#### [vp] Verification Plan
- **WHO**: Robo-System-Analyst
- **WHEN**: Complex requirements need verification approach or multi-stage validation strategy
- **WHAT**: Create verification checklist, define validation milestones, specify success criteria

#### [ip] Implementation Plan
- **WHO**: Robo-System-Analyst
- **WHEN**: Requirements analysis complete, ready to break down into implementable tasks
- **WHAT**: Document task breakdown, dependencies, estimates, and acceptance criteria

#### [er] Experiment Required
- **WHO**: Robo-System-Analyst
- **WHEN**: Technical uncertainty requires proof-of-concept or experimental validation
- **WHAT**: Define experiment scope, success metrics, and integration criteria

#### [tp] Tests Prepared
- **WHO**: Robo-Developer
- **WHEN**: TDD test cases written before implementation, ready for coding phase
- **WHAT**: Document test coverage, link to test files, signal ready for implementation

#### [dp] Development Progress
- **WHO**: Robo-Developer
- **WHEN**: Significant implementation milestone completed or increment ready
- **WHAT**: Document progress, update completion percentage, note any emerging issues

#### [br] Blocker Resolved
- **WHO**: Any Robo-Agent
- **WHEN**: Previously documented blocker has been successfully resolved
- **WHAT**: Document resolution method, update PRP status, signal ready to continue work

#### [rc] Research Complete
- **WHO**: Robo-System-Analyst
- **WHEN**: Commissioned research investigation completed with findings
- **WHAT**: Provide research findings, recommendations, and impact on PRP requirements

#### [tw] Tests Written
- **WHO**: Robo-Developer
- **WHEN**: Unit tests, integration tests, or E2E tests implemented for feature
- **WHAT**: Document test coverage, link to test files, signal ready for testing phase

#### [bf] Bug Fixed
- **WHO**: Robo-Developer
- **WHEN**: Bug or issue has been identified, resolved, and tested
- **WHAT**: Document bug details, fix approach, and verification results

#### [cq] Code Quality
- **WHO**: Robo-AQA
- **WHEN**: Code passes linting, formatting, and quality gate checks
- **WHAT**: Document quality metrics, any issues resolved, and overall quality status

#### [cp] CI Passed
- **WHO**: Robo-AQA
- **WHEN**: Continuous integration pipeline completes successfully
- **WHAT**: Document CI results, link to build artifacts, signal deployment readiness

#### [tr] Tests Red
- **WHO**: Robo-AQA
- **WHEN**: Test suite fails with failing tests identified
- **WHAT**: Document failing tests, error details, and debugging requirements

#### [tg] Tests Green
- **WHO**: Robo-AQA
- **WHEN**: All tests passing with full coverage achieved
- **WHAT**: Document test results, coverage metrics, and quality status

#### [cf] CI Failed
- **WHO**: Robo-AQA
- **WHEN**: Continuous integration pipeline fails with errors
- **WHAT**: Document CI failure details, debugging steps, and resolution requirements

#### [pc] Pre-release Complete
- **WHO**: Robo-AQA
- **WHEN**: All pre-release checks completed including documentation, changelogs, and verification
- **WHAT**: Document checklist completion, final quality status, and release readiness

#### [rg] Review Progress
- **WHO**: Any Robo-Agent
- **WHEN**: Code review in progress with feedback being addressed
- **WHAT**: Document review status, feedback items, and resolution timeline

#### [cd] Cleanup Done
- **WHO**: Robo-Developer
- **WHEN**: Code cleanup, temporary file removal, and final polishing completed
- **WHAT**: Document cleanup actions, removed artifacts, and final code state

#### [rv] Review Passed
- **WHO**: Robo-AQA
- **WHEN**: Code review completed successfully with all feedback addressed
- **WHAT**: Document review completion, approvals received, and merge readiness

#### [iv] Implementation Verified
- **WHO**: Robo-QC
- **WHEN**: Manual visual testing completed against published package or testable deployment
- **WHAT**: Document visual verification results, user experience validation, and final approval

#### [ra] Release Approved
- **WHO**: Robo-System-Analyst
- **WHEN**: All prerequisites met, stakeholder approval received, ready for release
- **WHAT**: Document approval details, release scope, and deployment authorization

#### [mg] Merged
- **WHO**: Robo-Developer
- **WHEN**: Code successfully merged to target branch with integration complete
- **WHAT**: Document merge details, integration status, and any merge conflicts resolved

#### [rl] Released
- **WHO**: Robo-Developer
- **WHEN**: Deployment completed successfully with release published
- **WHAT**: Document release details, deployment status, and user availability

#### [ps] Post-release Status
- **WHO**: Robo-System-Analyst
- **WHEN**: Post-release monitoring and status check completed
- **WHAT**: Document post-release health, user feedback, and system stability

#### [ic] Incident
- **WHO**: System Monitor/Any Agent
- **WHEN**: Production issue, error, or unexpected behavior detected
- **WHAT**: Document incident details, impact assessment, and immediate response actions

#### [JC] Jesus Christ (Incident Resolved)
- **WHO**: Robo-Developer/Robo-SRE
- **WHEN**: Critical production incident successfully resolved and service restored
- **WHAT**: Document resolution details, root cause, and prevention measures

#### [pm] Post-mortem
- **WHO**: Robo-System-Analyst
- **WHEN**: Incident analysis complete with lessons learned documented
- **WHAT**: Document incident timeline, root causes, improvements, and prevention strategies

#### [oa] Orchestrator Attention
- **WHO**: Any Robo-Agent
- **WHEN**: Need coordination of parallel work, resource allocation, or workflow orchestration
- **WHAT**: Request orchestrator intervention for task distribution, agent coordination, or workflow optimization

#### [AA] Admin Attention
- **WHO**: Any Robo-Agent/PRP
- **WHEN**: Report generation required, system status needed, or administrative oversight requested
- **WHAT**: Specify report requirements, timeline, and format needed for administrative review

#### [ap] Admin Preview Ready
- **WHO**: Robo-System-Analyst/Robo-AQA
- **WHEN**: Comprehensive report, analysis, or review ready for admin preview with how-to guide
- **WHAT**: Provide preview package with summary, guide, and admin instructions for review

#### [cc] Cleanup Complete
- **WHO**: Robo-Developer
- **WHEN**: All cleanup tasks completed before final commit (temp files, logs, artifacts removed)
- **WHAT**: Document cleanup actions, removed items, and system ready for final commit

---

### 🎨 UX/UI DESIGNER SIGNALS

#### [du] Design Update
- **WHO**: Robo-UX/UI-Designer
- **WHEN**: Design changes, new components, or visual updates are created
- **WHAT**: Document design modifications, update design system, signal design handoff readiness

#### [ds] Design System Updated
- **WHO**: Robo-UX/UI-Designer
- **WHEN**: Design system components, tokens, or guidelines are modified
- **WHAT**: Update design system documentation, coordinate with development on implementation

#### [dr] Design Review Requested
- **WHO**: Robo-UX/UI-Designer
- **WHEN**: Design proposals need feedback or approval
- **WHAT**: Present design concepts, request specific feedback, wait for review before proceeding

#### [dh] Design Handoff Ready
- **WHO**: Robo-UX/UI-Designer
- **WHEN**: Design assets and specifications are ready for development
- **WHAT**: Provide complete design package, assets, and implementation guidelines

#### [dd] Design Assets Delivered
- **WHO**: Robo-UX/UI-Designer
- **WHEN**: Final design assets are exported and available
- **WHAT**: Document asset delivery, formats, and optimization status

#### [dc] Design Change Implemented
- **WHO**: Robo-UX/UI-Designer
- **WHEN**: Design modifications are reflected in the live application
- **WHAT**: Verify design implementation accuracy, document any deviations

#### [df] Design Feedback Received
- **WHO**: Robo-UX/UI-Designer
- **WHEN**: User feedback, stakeholder input, or testing results are available
- **WHAT**: Document feedback insights, plan design iterations based on findings

#### [di] Design Issue Identified
- **WHO**: Robo-UX/UI-Designer
- **WHEN**: UX problems, accessibility issues, or design inconsistencies are found
- **WHAT**: Document design issues, impact assessment, and proposed solutions

#### [dt] Design Testing Complete
- **WHO**: Robo-UX/UI-Designer
- **WHEN**: User testing, A/B tests, or usability studies are finished
- **WHAT**: Provide test results, recommendations, and design improvements

#### [dp] Design Prototype Ready
- **WHO**: Robo-UX/UI-Designer
- **WHEN**: Interactive prototypes or mockups are available for review
- **WHAT**: Present prototype functionality, user flows, and interaction patterns

---

### ⚙️ DEVOPS/SRE SIGNALS

#### [id] Infrastructure Deployed
- **WHO**: Robo-DevOps/SRE
- **WHEN**: Infrastructure changes are deployed and verified
- **WHAT**: Document infrastructure updates, performance impact, and health status

#### [cd] CI/CD Pipeline Updated
- **WHO**: Robo-DevOps/SRE
- **WHEN**: Build, test, or deployment pipelines are modified
- **WHAT**: Update pipeline documentation, test new workflows, verify integration

#### [mo] Monitoring Online
- **WHO**: Robo-DevOps/SRE
- **WHEN**: Monitoring systems are configured and operational
- **WHAT**: Document monitoring coverage, alert rules, and dashboard availability

#### [ir] Incident Resolved
- **WHO**: Robo-DevOps/SRE
- **WHEN**: Production incidents are fixed and services restored
- **WHAT**: Document incident resolution, root cause, and prevention measures

#### [so] System Optimized
- **WHO**: Robo-DevOps/SRE
- **WHEN**: Performance improvements or cost optimizations are implemented
- **WHAT**: Document optimization results, performance gains, and resource savings

#### [sc] Security Check Complete
- **WHO**: Robo-DevOps/SRE
- **WHEN**: Security scans, vulnerability assessments, or compliance checks are done
- **WHAT**: Provide security findings, remediation status, and compliance validation

#### [pb] Performance Baseline Set
- **WHO**: Robo-DevOps/SRE
- **WHEN**: Performance benchmarks and baselines are established
- **WHAT**: Document performance metrics, thresholds, and monitoring targets

#### [dr] Disaster Recovery Tested
- **WHO**: Robo-DevOps/SRE
- **WHEN**: Disaster recovery procedures are validated through testing
- **WHAT**: Document test results, recovery times, and improvement areas

#### [cu] Capacity Updated
- **WHO**: Robo-DevOps/SRE
- **WHEN**: System capacity is scaled or resource allocation is modified
- **WHAT**: Document capacity changes, scaling triggers, and cost implications

#### [ac] Automation Configured
- **WHO**: Robo-DevOps/SRE
- **WHEN**: New automation workflows or scripts are implemented
- **WHAT**: Document automation coverage, efficiency gains, and maintenance requirements

#### [sl] SLO/SLI Updated
- **WHO**: Robo-DevOps/SRE
- **WHEN**: Service Level Objectives or Indicators are modified
- **WHAT**: Update reliability targets, measurement criteria, and monitoring alerts

#### [eb] Error Budget Status
- **WHO**: Robo-DevOps/SRE
- **WHEN**: Error budget consumption is tracked or thresholds are reached
- **WHAT**: Document error budget usage, burn rate, and release freeze decisions

#### [ip] Incident Prevention
- **WHO**: Robo-DevOps/SRE
- **WHEN**: Proactive measures are taken to prevent potential incidents
- **WHAT**: Document prevention actions, risk mitigation, and monitoring improvements

#### [rc] Reliability Check Complete
- **WHO**: Robo-DevOps/SRE
- **WHEN**: System reliability assessments or health checks are performed
- **WHAT**: Provide reliability status, identified risks, and improvement recommendations

#### [rt] Recovery Time Measured
- **WHO**: Robo-DevOps/SRE
- **WHEN**: Recovery time objectives are measured or tested
- **WHAT**: Document RTO metrics, recovery procedures, and performance against targets

#### [Ao] Alert Optimized
- **WHO**: Robo-DevOps/SRE
- **WHEN**: Alert rules, thresholds, or notification systems are improved
- **WHAT**: Document alert changes, noise reduction, and response time improvements

#### [ps] Post-mortem Started
- **WHO**: Robo-DevOps/SRE
- **WHEN**: Incident post-mortem analysis begins
- **WHAT**: Document post-mortem scope, participants, and investigation timeline

#### [ts] Troubleshooting Session
- **WHO**: Robo-DevOps/SRE
- **WHEN**: Active troubleshooting of system issues is in progress
- **WHAT**: Document investigation steps, findings, and resolution progress

#### [er] Escalation Required
- **WHO**: Robo-DevOps/SRE
- **WHEN**: Issues require escalation to senior teams or external vendors
- **WHAT**: Document escalation reasons, current status, and expected resolution timeline

---

### 🔄 PARALLEL COORDINATION SIGNALS

#### [pc] Parallel Coordination Needed
- **WHO**: Any Robo-Agent
- **WHEN**: Multiple agents need to synchronize work or resolve dependencies
- **WHAT**: Request coordination meeting, identify conflicts, propose resolution approach

#### [fo] File Ownership Conflict
- **WHO**: Any Robo-Agent
- **WHEN**: File ownership or modification conflicts arise between agents
- **WHAT**: Document conflict details, propose ownership resolution, coordinate changes

#### [cc] Component Coordination
- **WHO**: Robo-UX/UI-Designer & Robo-Developer
- **WHEN**: UI components need coordinated design and development
- **WHAT**: Sync component specifications, coordinate implementation timelines

#### [sr] Asset Sync Required
- **WHO**: Robo-UX/UI-Designer & Robo-DevOps/SRE
- **WHEN**: Design assets need deployment or CDN updates
- **WHAT**: Coordinate asset delivery, optimization, and deployment pipeline

#### [pt] Performance Testing Design
- **WHO**: Robo-UX/UI-Designer & Robo-DevOps/SRE
- **WHEN**: Design changes require performance validation
- **WHAT**: Coordinate performance testing, measure design impact, optimize delivery

#### [pe] Parallel Environment Ready
- **WHO**: Robo-DevOps/SRE
- **WHEN**: Staging or testing environments are ready for parallel work
- **WHAT**: Document environment status, access details, and coordination requirements

#### [fs] Feature Flag Service Updated
- **WHO**: Robo-DevOps/SRE
- **WHEN**: Feature flags need configuration for parallel development
- **WHAT**: Update feature flag configurations, coordinate rollout strategies

#### [ds] Database Schema Sync
- **WHO**: Robo-DevOps/SRE & Robo-Developer
- **WHEN**: Database changes require coordinated deployment
- **WHAT**: Sync schema changes, coordinate migration timing, validate compatibility

#### [rb] Rollback Prepared
- **WHO**: Robo-DevOps/SRE
- **WHEN**: Rollback procedures need preparation for parallel deployments
- **WHAT**: Document rollback plans, test rollback procedures, verify recovery paths

---

## 🚀 EMOTIONAL STATE TRACKING & MENTAL HEALTH

### **Agent Personalities & Communication Style**
- **System Analyst**: Uses Portuguese expressions (Encantado ✨, Incrível 🎉)
- **Developer**: Pragmatic, focused (Confident ✅, Blocked 🚫)
- **Tester**: Skeptical, thorough (Validated 🎯, Frustrated 😤)
- **Designer**: Visual, aesthetic (Excited 🎉, Optimistic 🌟)
- **UX/UI Designer**: Creative and user-centered (Inspired ✨, User-focused 🎯, Creative 💡)
- **DevOps/SRE**: Systematic and reliability-focused (System Optimized ⚙️, Infrastructure Stable 🛡️, Automated 🤖)

### **Mental Health Best Practices**
- **PRP Comments**: Always leave comments about work done and how you feel about it
- **Cleanup Documentation**: Comment on `/tmp` files, dev servers, ports that need cleanup
- **Work Scope Boundaries**: Comment when working on files outside expected PRP scope
- **Uncertainty Handling**: Comment on uncertainty and wait for guidance for complex decisions
- **Context Management**: Create checkpoints when context limits are reached
- **Frustration Escalation**: Use proper escalation paths when technically blocked

### **Gate-Based Validation Using Actual Signals**
- **DoD Verification**: Use `[dA]` signal when ready for Definition of Done validation
- **Quality Gates**: Signal when each quality gate is passed or failed
- **Pre-Release**: Signal when pre-release checklist completed
- **Release Approval**: Signal when release is approved for deployment

---

## 🔄 PARALLEL COORDINATION RULES

> !! launch 10+ parallel sub-agents simultaneously NOT one-by-one, but all what you can, we should work in parallel when possible and use sub-agents what most suitable for always !!

### **File Ownership Management**
- **Primary Ownership**: Each agent has defined file patterns they own primarily
- **Shared Files**: Coordination required for files that overlap ownership boundaries
- **Conflict Resolution**: Use `[fo]` signal for ownership conflicts, escalate to orchestrator if unresolved
- **Change Notification**: Agents must signal changes to shared files using appropriate coordination signals

### **Design-DevOps Coordination**
- **Asset Pipeline**: Robo-UX/UI-Designer creates assets → `[dd]` signal → Robo-DevOps/SRE optimizes deployment → `[sr]` signal
- **Performance Impact**: Design changes requiring performance validation trigger `[pt]` signal
- **Design System Updates**: Design system changes require `[ds]` signal and coordination with development team

### **Development-DevOps Coordination**
- **Infrastructure Changes**: Development requirements trigger `[id]` signal from Robo-DevOps/SRE
- **Database Schemas**: Schema changes require `[ds]` signal coordination between developer and SRE
- **Environment Management**: Parallel development requires `[pe]` signal for environment readiness

### **Cross-Functional Workflows**
- **Component Development**: `[cc]` signal coordinates design and development work
- **Feature Rollouts**: `[fs]` signal manages feature flag coordination
- **Incident Response**: `[er]` signal escalates issues requiring multiple agents

### **Synchronization Protocols**
- **Daily Checkpoints**: Agents use `[oa]` signal for orchestrator coordination
- **Milestone Alignment**: Major deliverables require `[pc]` signal for parallel work sync
- **Quality Gates**: Cross-agent quality checks use `[rg]` signal for review coordination

### **Parallel Work Optimization**
- **Independent Work**: Agents can work independently on owned files without coordination
- **Dependent Work**: Required coordination signals must be used before dependent work begins
- **Simultaneous Delivery**: Multiple agents can deliver simultaneously when dependencies are resolved

### **Conflict Prevention**
- **Pre-emptive Communication**: Agents signal upcoming changes that might affect others
- **Shared Roadmap**: Regular coordination through `[oa]` signal maintains alignment
- **Resource Allocation**: Orchestrator manages competing priorities through `[pc]` signal

---

> SYSTEM PART END! NEVER EDIT ABOVE

## USER SECTION!

### release flow
TBD

### landing gh-pages deploy
TBD

### mondatory project rules!
- NEVER git stash or play with git branch or history! NEVER! i need you always ask confirmation
- ALWAYS update ONLY related to prp files, before start work leave list of files you will work on, then work only with related files! ALL CODE REVIEW MAXIMUM ALERT IF ANYTHING OUTSIDE PRP SCOPE EDITED WITHOUT REASON AND NOTICE!
