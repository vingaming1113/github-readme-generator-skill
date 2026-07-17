# Badges And Stats

Use this reference when a README needs visual polish from Shields.io or GitHub Stats Extended.

## Shields.io

Common GitHub badge patterns:

```markdown
[![Stars](https://img.shields.io/github/stars/OWNER/REPO?style=for-the-badge)](https://github.com/OWNER/REPO/stargazers)
[![Issues](https://img.shields.io/github/issues/OWNER/REPO?style=for-the-badge)](https://github.com/OWNER/REPO/issues)
[![License](https://img.shields.io/github/license/OWNER/REPO?style=for-the-badge)](https://github.com/OWNER/REPO/blob/main/LICENSE)
[![Release](https://img.shields.io/github/v/release/OWNER/REPO?style=for-the-badge)](https://github.com/OWNER/REPO/releases)
[![CI](https://img.shields.io/github/actions/workflow/status/OWNER/REPO/ci.yml?branch=main&style=for-the-badge&label=CI)](https://github.com/OWNER/REPO/actions)
```

Use static badges for technology or status labels:

```markdown
![Python](https://img.shields.io/badge/Python-3.11+-3776AB?style=for-the-badge&logo=python&logoColor=white)
![Status](https://img.shields.io/badge/status-active-success?style=for-the-badge)
```

Guidelines:

- Use `style=for-the-badge` for a bold, consistent header row; use `flat-square` when the repo already has a quieter visual style.
- URL-encode spaces as `%20`, plus signs as `%2B`, hashes as `%23`, and slashes in labels when needed.
- Keep the first badge row to 3-6 badges. Put package ecosystem badges near install instructions when they are secondary.
- Link badges to useful destinations: CI to Actions, release to Releases, license to the license file, coverage to the report.

## GitHub Stats Extended

GitHub Stats Extended is maintained at `https://github.com/stats-organization/github-stats-extended` and exposes public card endpoints from `https://github-stats-extended.vercel.app`.

Stats card:

```markdown
[![GitHub Stats](https://github-stats-extended.vercel.app/api?username=OWNER&show_icons=true&theme=tokyonight)](https://github.com/stats-organization/github-stats-extended)
```

Top languages:

```markdown
[![Top Languages](https://github-stats-extended.vercel.app/api/top-langs/?username=OWNER&layout=compact&theme=tokyonight)](https://github.com/stats-organization/github-stats-extended)
```

Pinned repository:

```markdown
[![Repo Card](https://github-stats-extended.vercel.app/api/pin/?username=OWNER&repo=REPO&theme=tokyonight)](https://github.com/OWNER/REPO)
```

Guidelines:

- Use stats cards mainly for profile READMEs, organization READMEs, or project READMEs where maintainer identity is part of the story.
- For private contributions or organization/private repository activity, note that a self-hosted deployment or generated SVG workflow may be needed so tokens are not exposed.
- Keep all cards on one visual theme. Common themes include `default`, `tokyonight`, `radical`, `dracula`, `nord`, and `gruvbox`.
- If migrating legacy snippets, change the domain from `github-readme-stats.vercel.app` to `github-stats-extended.vercel.app` and keep compatible query parameters.

## Layout Patterns

Project README first screen:

```markdown
<h1 align="center">Project Name</h1>
<p align="center">One sentence that explains the result.</p>
<p align="center">
  BADGES
</p>

## Overview
## Features
## Quick Start
## Usage
## Configuration
## Development
## Contributing
## License
```

Profile README first screen:

```markdown
<h1 align="center">Hi, I'm NAME</h1>
<p align="center">Short positioning statement.</p>

## Focus
## Featured Work
## Stack
## Stats
## Contact
```

Prefer short, useful sections over decorative clutter. A beautiful README is readable first.
