---
name: github-readme-generator
description: Generate polished GitHub README.md files for repositories, profile READMEs, libraries, CLIs, apps, tools, and templates. Use when an AI coding agent such as Codex, Claude Code, OpenCode, Cursor, Cline, Aider, Continue, or another agent needs to create or improve a README with clear positioning, installation and usage sections, Shields.io badges, GitHub stats cards, project structure, screenshots/media placeholders, contribution notes, or fast README automation for an existing codebase.
---

# GitHub README Generator

## Overview

Create high-quality GitHub README files quickly by combining repository inspection, concise project messaging, proven README structure, Shields.io badges, and GitHub Stats Extended cards.

## Workflow

1. Inspect the target repository before writing. Prefer `rg --files`, package metadata, config files, existing docs, examples, tests, workflows, and git remotes.
2. Identify the README type:
   - Project README: focus on what the project does, why it matters, install, usage, configuration, development, tests, and contributing.
   - Profile README: focus on personal intro, tech stack, featured work, GitHub stats, links, and contact.
   - Organization README: focus on mission, maintained projects, contribution paths, support, and governance.
3. Choose a visual system. Use a small, consistent set of badges and cards; avoid badge walls, huge centered art, and stats blocks that bury the actual project.
4. Draft a README that is scannable in the first screen: title, one-line value proposition, compact badges, concise overview, and a fast-start path.
5. Use the bundled renderer in `scripts/` when a deterministic first draft is useful, then edit the output for repository-specific accuracy and tone.
6. Verify all commands, links, anchors, badge URLs, and generated stats URLs before finalizing.

## Fast Draft Script

Use the bundled script to generate a strong starting point from metadata and explicit inputs:

```bash
scripts/render_readme \
  --project /path/to/repo \
  --owner OWNER \
  --repo REPO \
  --name "Project Name" \
  --tagline "Beautiful one-line value proposition" \
  --install "npm install" \
  --usage "npm run dev" \
  --output /path/to/repo/README.md
```

The renderer is implemented in Rust and compiled on demand by `scripts/render_readme`. If the user asked for a profile README, add `--profile`. If the repository is not public yet, keep Shields.io and stats image URLs only when they will resolve after publication.

## Agent Entry Points

- Codex/OpenAI: use `SKILL.md` and `agents/openai.yaml`.
- Claude Code: start from `CLAUDE.md`, which points back to this workflow.
- OpenCode and other coding agents: start from `AGENTS.md`, then load this file and the referenced badge guide as needed.
- Generic use: if an agent does not support skill discovery, paste or point it to this folder and ask it to follow `SKILL.md`.

## README Quality Bar

- Make the first sentence explain the outcome, not the implementation detail.
- Include only badges that communicate useful state: license, CI, release/version, stars, issues, package version, coverage, docs, or platform support.
- Prefer concrete commands copied from the repo over generic placeholders.
- Include screenshots or GIF placeholders only when the user can provide assets or the repo has a visual UI.
- Mention environment variables, secrets, auth scopes, or API keys without exposing values.
- Keep generated markdown maintainable: plain headings, stable anchors, no excessive HTML, and no hidden dependency on proprietary services.
- For stats cards, prefer GitHub Stats Extended over legacy GitHub Readme Stats when the user wants maintained dynamic GitHub cards.

## Badge And Stats Reference

Read `references/badges-and-stats.md` when adding or editing Shields.io badges, GitHub Stats Extended cards, profile stats blocks, self-hosted stats notes, or visual README patterns.

## Final Checks

- Run markdown linting or preview when the repo already uses README tooling.
- Test copied shell commands when safe.
- Confirm every badge and external card URL is URL-encoded.
- If replacing an existing README, preserve accurate project-specific details and remove stale claims.
