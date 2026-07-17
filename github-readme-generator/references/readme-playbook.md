# README Playbook

Use this reference when a README needs to feel authored, not templated.

## First-Screen Contract

The first screen must answer:

- What is this?
- Why should the reader care?
- How can they try it?
- Is it maintained and safe enough to evaluate?

Good first screens usually include a centered title, one clear value proposition, 3-6 useful badges, and navigation to quick start, docs, demo, and contribution sections.

## Repository Intelligence Pass

Before drafting, inspect:

- Package metadata: `package.json`, `pyproject.toml`, `Cargo.toml`, `go.mod`, lockfiles.
- Commands: dev, start, build, test, lint, format, release.
- Repo health: license, contributing guide, code of conduct, changelog, CI workflows.
- Usage evidence: examples, demos, tests, docs, screenshots, fixtures.
- Runtime needs: environment variables, Docker files, service config, auth scopes.
- Visual assets: screenshots, GIFs, logos, diagrams, app previews.

Do not invent support claims. If a command, feature, or platform is inferred rather than verified, phrase it as a placeholder for the maintainer to confirm.

## README Type Matrix

| Type | Optimize for | Sections to prioritize |
| --- | --- | --- |
| CLI | Copy-paste terminal success | Install, Quick Start, Usage, Commands, Examples, Configuration |
| App | Local run and visual confidence | Demo, Screenshots, Quick Start, Configuration, Deployment, Testing |
| Library | Integration confidence | Install, Minimal Example, API Surface, Examples, Versioning |
| Framework/template | Starting a new project | Features, Create/Clone, Customize, Project Structure, Deploy |
| Profile | Personal positioning | Focus, Featured Work, Stack, Stats, Contact |
| Organization | Trust and navigation | Mission, Projects, Contribution Paths, Governance, Support |
| Docs hub | Findability | Contents, Scope, Guides, Examples, Contribution |

## Section Recipes

Overview:

- Explain the outcome in one sentence.
- Add one short paragraph about who it is for.
- Avoid implementation-first language unless the audience is purely technical.

Highlights:

- Use 3-6 bullets.
- Lead with benefits and workflows, not generic adjectives.
- Mention detected repo facts: CI, examples, docs, Docker, package manager.

Quick Start:

- Include only commands needed to reach a working result.
- Put setup and first run in one fenced block.
- If `.env.example` exists, include `cp .env.example .env`.

Usage:

- Show the smallest realistic invocation.
- For libraries, show the minimal import and call.
- For apps, tell the user where to open the local app.

Configuration:

- List environment variables or config files.
- Mention secrets without exposing values.
- Include Docker or deployment notes only when files exist.

Project Structure:

- Include only meaningful paths.
- Avoid listing every file.
- Explain why a user or contributor would open each path.

Contributing:

- Link existing contribution and conduct files.
- Give bug report expectations.
- Mention test commands if detected.

## Visual And Polish Ideas

Borrowed from the `AwesomeREADME` resource categories:

- Use Shields.io, Badgen, Simple Icons, Markdown Badges, and Skill Icons for metadata and tech stacks.
- Use GitHub Stats Extended, streak stats, trophy cards, activity graphs, summary cards, Metrics, or WakaTime only when they support the reader's goal.
- Use Capsule Render, Readme Typing SVG, or generated banners sparingly for profile READMEs and highly visual projects.
- Use Mermaid, Excalidraw, draw.io, PlantUML, Carbon, or Ray.so for architecture, flows, and code visuals.
- Use asciinema, VHS, Kap, ScreenToGif, or compressed GIFs for demos.
- Use GitHub Actions, Blog Post Workflow, Waka Readme, All Contributors, or Readme Scribe for sections that must stay current.

## Quality Checklist

- Start with the reader's goal: what the project does, why it matters, and how to try it.
- Put install, quick start, screenshots, and examples before deep internals.
- Use badges to communicate status, not decorate every line.
- Keep profile READMEs personal but avoid private details and noisy widgets.
- Prefer lightweight images, compressed assets, and stable external services.
- Add alt text for images and meaningful labels for links.
- Test README links, commands, badges, and generated cards after major changes.
- Keep dynamic stats self-hosted when uptime, privacy, or rate limits matter.
