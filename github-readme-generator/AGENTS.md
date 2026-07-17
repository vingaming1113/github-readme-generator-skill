# GitHub README Generator

Use this folder to create or improve GitHub README.md files for repositories, profile READMEs, organization READMEs, libraries, CLIs, apps, and templates.

Start with `SKILL.md` for the workflow and quality bar. Read `references/readme-playbook.md` for repository-specific strategy, `references/badges-and-stats.md` for Shields.io and GitHub Stats Extended patterns, and `references/resource-catalog.md` when choosing external README polish resources. Use the renderer in `scripts/` when a deterministic first draft is useful, then edit the README for repository-specific accuracy.

Core behavior:

- Inspect the repository before writing.
- Lead with a clear value proposition and fast-start path.
- Use a compact badge row, not a badge wall.
- Prefer concrete commands from the repo over placeholders.
- Add conditional sections from actual repo signals: docs, examples, screenshots, CI, env files, Docker, license, changelog, and contribution files.
- Verify badge URLs, links, commands, and generated stats URLs before finishing.
