<h1 align="center">GitHub README Generator</h1>
<p align="center"><strong>Give any AI coding agent a repeatable way to ship beautiful GitHub READMEs in seconds.</strong></p>
<p align="center">
  <img alt="AI Agents" src="https://img.shields.io/badge/AI%20Agents-Codex%20%7C%20Claude%20Code%20%7C%20OpenCode-7c3aed?style=for-the-badge" />
  <img alt="README Generator" src="https://img.shields.io/badge/README-generator-2ea44f?style=for-the-badge" />
  <img alt="Shields.io" src="https://img.shields.io/badge/Shields.io-badges-111827?style=for-the-badge" />
  <a href="https://github.com/vingaming1113/github-readme-generator-skill/blob/main/LICENSE"><img alt="License" src="https://img.shields.io/github/license/vingaming1113/github-readme-generator-skill?style=for-the-badge" /></a>
</p>

<p align="center">
  <a href="#install">Install</a> ·
  <a href="#what-it-gives-your-agent">Agent Workflow</a> ·
  <a href="#generated-readme-style">Generated Style</a> ·
  <a href="#stats-and-badges">Stats & Badges</a>
</p>

---

## What It Does

`github-readme-generator` is a portable AI-agent skill pack for creating READMEs that look like someone cared: tight positioning, useful badge rows, clean quick starts, project-specific commands, and GitHub Stats Extended cards when they add value.

It works as a Codex skill, a Claude Code instruction pack, an OpenCode/AGENTS.md workflow, or a plain folder any coding agent can read.

## Generated README Style

This repository README is intentionally written in the same style the skill teaches agents to produce:

<table>
  <tr>
    <td><strong>First-screen polish</strong></td>
    <td>Centered title, sharp value proposition, compact badge row, and useful navigation.</td>
  </tr>
  <tr>
    <td><strong>Real adoption path</strong></td>
    <td>Install, generate, customize, and verify without burying the reader in prose.</td>
  </tr>
  <tr>
    <td><strong>Dynamic GitHub visuals</strong></td>
    <td>Shields.io badges and GitHub Stats Extended cards are documented as reusable patterns.</td>
  </tr>
  <tr>
    <td><strong>Agent-ready workflow</strong></td>
    <td>Instructions are short enough to load, specific enough to steer, and portable across agents.</td>
  </tr>
</table>

<p align="center">
  <img alt="GitHub stats" src="https://github-stats-extended.vercel.app/api?username=vingaming1113&show_icons=true&theme=tokyonight" />
</p>

## Install

Use the whole `github-readme-generator/` folder with whichever agent you prefer.

### Codex

Copy the folder into your Codex skills directory:

```bash
cp -R github-readme-generator "${CODEX_HOME:-$HOME/.codex}/skills/"
```

Then ask Codex:

```text
Use $github-readme-generator to create a polished README for this repository.
```

### Claude Code

Copy or reference the folder in your project and ask Claude Code to read:

```text
Use github-readme-generator/CLAUDE.md to create a polished README for this repository.
```

### OpenCode And Other Agents

Point the agent at:

```text
github-readme-generator/AGENTS.md
```

The shared instructions route the agent to the same workflow, references, and draft renderer.

## Generate A Draft

```bash
python3 github-readme-generator/scripts/render_readme.py \
  --project . \
  --owner YOUR_GITHUB_USER \
  --repo YOUR_REPO \
  --output README.md
```

Profile README:

```bash
python3 github-readme-generator/scripts/render_readme.py \
  --profile \
  --owner YOUR_GITHUB_USER \
  --name "Your Name" \
  --output README.md
```

## What It Gives Your Agent

| File | Purpose |
| --- | --- |
| `SKILL.md` | Core README-generation workflow and quality bar |
| `AGENTS.md` | Portable entry point for OpenCode, Cursor, Cline, Aider, Continue, and other agents |
| `CLAUDE.md` | Claude Code entry point |
| `references/badges-and-stats.md` | Shields.io and GitHub Stats Extended URL patterns |
| `scripts/render_readme.py` | Deterministic first-draft renderer |
| `agents/openai.yaml` | OpenAI/Codex UI metadata |

## Stats And Badges

The skill teaches agents to use high-signal visuals instead of badge spam:

```markdown
<p align="center">
  <img alt="Built with Rust" src="https://img.shields.io/badge/built%20with-Rust-b7410e?style=for-the-badge" />
  <a href="https://github.com/OWNER/REPO/stargazers"><img alt="Stars" src="https://img.shields.io/github/stars/OWNER/REPO?style=for-the-badge" /></a>
  <a href="https://github.com/OWNER/REPO/issues"><img alt="Issues" src="https://img.shields.io/github/issues/OWNER/REPO?style=for-the-badge" /></a>
</p>
```

## GitHub Stats Extended

This skill uses `https://github-stats-extended.vercel.app` card URLs for maintained GitHub stats embeds and points agents to the upstream project at `https://github.com/stats-organization/github-stats-extended`.

## Design Principles

- Lead with the result, not the implementation.
- Use badges as metadata, not decoration.
- Make the first screen useful enough that people keep reading.
- Keep instructions portable so any AI coding agent can apply them.

## License

MIT
