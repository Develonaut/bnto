# Code Review

Perform a comprehensive code review of the current Go project following the Bento Box Principle.

## Instructions

You MUST read these documents before starting:

### Core Principles
- `.claude/BENTO_BOX_PRINCIPLE.md` - The foundational philosophy

### Workflow Checklists (CRITICAL)
- `.claude/workflow/MANDATORY_CHECKLIST.md` - Pre-work requirements
- `.claude/workflow/ENFORCEMENT_CHECKLIST.md` - Karen's enforcement rules
- `.claude/workflow/REVIEW_CHECKLIST.md` - Comprehensive review criteria

### Detailed Review Process
- `.claude/commands/code-review.md` - Complete review process and format

## Review Process

Coordinate three specialized agents in parallel:

1. **Colossus** (`.claude/agents/Colossus.md`) - Go Standards Guardian
2. **Karen** (`.claude/agents/Karen.md`) - Bento Box Enforcer

Each agent must:
- Read all required documents listed above
- Follow the detailed process in `.claude/commands/code-review.md`
- Provide their specific report format
- Give APPROVED or CHANGES REQUIRED verdict

After all three agents complete their reviews, provide a combined final report with overall verdict.

**CRITICAL**: All three reviewers must approve before code can be committed. Karen enforces with ZERO TOLERANCE.
