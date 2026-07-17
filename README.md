<h1 align="center">GitHub README Generator Skill</h1>
<p align="center">A Codex skill for generating beautiful GitHub READMEs in seconds.</p>
<p align="center">
  <img alt="Skill" src="https://img.shields.io/badge/Codex-skill-0969da?style=for-the-badge" />
  <img alt="README" src="https://img.shields.io/badge/README-generator-2ea44f?style=for-the-badge" />
  <a href="https://github.com/vingaming1113/github-readme-generator-skill/blob/main/LICENSE"><img alt="License" src="https://img.shields.io/github/license/vingaming1113/github-readme-generator-skill?style=for-the-badge" /></a>
</p>

## Overview

`github-readme-generator` helps AI agents create polished project, profile, and organization READMEs with clear structure, useful Shields.io badges, and GitHub Stats Extended cards.

The skill includes:

- A concise agent workflow in `SKILL.md`
- Badge and stats patterns for Shields.io and GitHub Stats Extended
- A deterministic README draft renderer at `scripts/render_readme.py`
- Codex UI metadata in `agents/openai.yaml`

## Quick Start

Copy the skill folder into your Codex skills directory:

```bash
cp -R github-readme-generator "${CODEX_HOME:-$HOME/.codex}/skills/"
```

Generate a README draft:

```bash
python3 github-readme-generator/scripts/render_readme.py \
  --project . \
  --owner YOUR_GITHUB_USER \
  --repo YOUR_REPO \
  --output README.md
```

## GitHub Stats Extended

This skill uses `https://github-stats-extended.vercel.app` card URLs for maintained GitHub stats embeds and points agents to the upstream project at `https://github.com/stats-organization/github-stats-extended`.

## License

MIT
