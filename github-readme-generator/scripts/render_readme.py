#!/usr/bin/env python3
"""Render a polished GitHub README draft from repo metadata and CLI inputs."""

from __future__ import annotations

import argparse
import json
import re
from collections import Counter
from configparser import ConfigParser
from pathlib import Path
from urllib.parse import quote


EXTENSION_LANGUAGES = {
    ".js": "JavaScript",
    ".jsx": "JavaScript",
    ".ts": "TypeScript",
    ".tsx": "TypeScript",
    ".py": "Python",
    ".go": "Go",
    ".rs": "Rust",
    ".java": "Java",
    ".kt": "Kotlin",
    ".cs": "C#",
    ".php": "PHP",
    ".rb": "Ruby",
    ".swift": "Swift",
    ".dart": "Dart",
    ".vue": "Vue",
    ".svelte": "Svelte",
}


def shield_url(label: str, message: str, color: str = "0969da", logo: str | None = None) -> str:
    url = f"https://img.shields.io/badge/{quote(label)}-{quote(message)}-{color}?style=for-the-badge"
    if logo:
        url += f"&logo={quote(logo)}&logoColor=white"
    return url


def badge_img(label: str, src: str) -> str:
    return f'<img alt="{label}" src="{src}" />'


def github_badge(label: str, path: str, owner: str, repo: str, target: str) -> str:
    image = f"https://img.shields.io/github/{path}/{owner}/{repo}?style=for-the-badge"
    return f'<a href="https://github.com/{owner}/{repo}/{target}">{badge_img(label, image)}</a>'


def read_json(path: Path) -> dict:
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except Exception:
        return {}


def detect_remote(project: Path) -> tuple[str | None, str | None]:
    config_path = project / ".git" / "config"
    if not config_path.exists():
        return None, None
    parser = ConfigParser()
    parser.read(config_path)
    for section in parser.sections():
        if section.startswith('remote "') and parser.has_option(section, "url"):
            url = parser.get(section, "url")
            match = re.search(r"github\.com[:/]([^/]+)/([^/.]+)(?:\.git)?$", url)
            if match:
                return match.group(1), match.group(2)
    return None, None


def detect_metadata(project: Path) -> dict:
    metadata: dict[str, str] = {}
    package_json = project / "package.json"
    if package_json.exists():
        package = read_json(package_json)
        metadata["name"] = package.get("name", "")
        metadata["description"] = package.get("description", "")
        scripts = package.get("scripts", {})
        if "dev" in scripts:
            metadata["usage"] = "npm run dev"
        elif "start" in scripts:
            metadata["usage"] = "npm start"
        metadata["install"] = "npm install"

    pyproject = project / "pyproject.toml"
    if pyproject.exists() and not metadata.get("name"):
        try:
            import tomllib

            data = tomllib.loads(pyproject.read_text(encoding="utf-8"))
            project_data = data.get("project", {})
            metadata["name"] = project_data.get("name", "")
            metadata["description"] = project_data.get("description", "")
            metadata["install"] = "pip install -e ."
            metadata["usage"] = "python -m " + metadata["name"].replace("-", "_")
        except Exception:
            pass

    cargo = project / "Cargo.toml"
    if cargo.exists() and not metadata.get("name"):
        text = cargo.read_text(encoding="utf-8", errors="ignore")
        name = re.search(r'(?m)^name\s*=\s*"([^"]+)"', text)
        description = re.search(r'(?m)^description\s*=\s*"([^"]+)"', text)
        if name:
            metadata["name"] = name.group(1)
            metadata["usage"] = f"cargo run --bin {name.group(1)}"
            metadata["install"] = "cargo build --release"
        if description:
            metadata["description"] = description.group(1)

    return metadata


def detect_language(project: Path) -> str:
    counts: Counter[str] = Counter()
    ignored = {".git", "node_modules", "dist", "build", "target", ".venv", "vendor"}
    for path in project.rglob("*"):
        if any(part in ignored for part in path.parts):
            continue
        if path.is_file() and path.suffix in EXTENSION_LANGUAGES:
            counts[EXTENSION_LANGUAGES[path.suffix]] += 1
    return counts.most_common(1)[0][0] if counts else "Markdown"


def clean_name(value: str) -> str:
    value = value.strip()
    if " " in value or any(character.isupper() for character in value):
        return value
    return value.replace("_", " ").replace("-", " ").title()


def project_readme(args: argparse.Namespace, meta: dict, owner: str | None, repo: str | None) -> str:
    name = args.name or meta.get("name") or (repo or "Project")
    title = clean_name(name)
    tagline = args.tagline or meta.get("description") or "A polished, useful project built for fast adoption."
    language = args.language or detect_language(args.project)
    install = args.install or meta.get("install") or "# Add install command"
    usage = args.usage or meta.get("usage") or "# Add usage command"

    badges = [badge_img(language, shield_url("built with", language, "2ea44f"))]
    if owner and repo:
        badges.extend(
            [
                github_badge("Stars", "stars", owner, repo, "stargazers"),
                github_badge("Issues", "issues", owner, repo, "issues"),
                github_badge("License", "license", owner, repo, "blob/main/LICENSE"),
            ]
        )
    else:
        badges.append(badge_img("Status", shield_url("status", "active", "success")))

    stats = ""
    if args.include_stats and owner:
        stats = f"""
## Stats

<p align="center">
  <img alt="GitHub stats" src="https://github-stats-extended.vercel.app/api?username={owner}&show_icons=true&theme={args.theme}" />
</p>
"""

    return f"""<h1 align="center">{title}</h1>
<p align="center">{tagline}</p>
<p align="center">
  {" ".join(badges)}
</p>

## Overview

{tagline}

## Features

- Fast setup with clear defaults
- Focused workflow for real projects
- Clean GitHub-native documentation

## Quick Start

```bash
{install}
{usage}
```

## Usage

```bash
{usage}
```

## Configuration

Document required environment variables, config files, and service credentials here. Never commit secrets.

## Development

```bash
{install}
# Add test, lint, and formatting commands
```
{stats}
## Contributing

Issues and pull requests are welcome. Please include clear reproduction steps for bugs and concise context for feature requests.

## License

{args.license}
"""


def profile_readme(args: argparse.Namespace, owner: str | None) -> str:
    username = owner or args.owner or "USERNAME"
    name = args.name or username
    tagline = args.tagline or "Building useful software and sharing what I learn."
    theme = args.theme
    return f"""<h1 align="center">Hi, I'm {name}</h1>
<p align="center">{tagline}</p>

## Focus

- Building reliable software with clear user value
- Turning rough ideas into polished tools
- Writing documentation that helps people move quickly

## Stack

<p>
  <img alt="GitHub" src="{shield_url('GitHub', username, '181717', 'github')}" />
  <img alt="Open Source" src="{shield_url('open source', 'builder', '2ea44f')}" />
</p>

## Featured Work

- Add featured repositories here with one-line outcomes.
- Include demos, screenshots, or package links where useful.

## Stats

<p align="center">
  <img alt="GitHub stats" src="https://github-stats-extended.vercel.app/api?username={username}&show_icons=true&theme={theme}" />
  <img alt="Top languages" src="https://github-stats-extended.vercel.app/api/top-langs/?username={username}&layout=compact&theme={theme}" />
</p>

## Contact

- GitHub: [@{username}](https://github.com/{username})
"""


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--project", type=Path, default=Path.cwd(), help="Repository to inspect")
    parser.add_argument("--output", type=Path, help="Path to write README.md")
    parser.add_argument("--owner", help="GitHub owner or profile username")
    parser.add_argument("--repo", help="GitHub repository name")
    parser.add_argument("--name", help="Project, profile, or organization display name")
    parser.add_argument("--tagline", help="Short value proposition")
    parser.add_argument("--language", help="Primary language or stack label")
    parser.add_argument("--install", help="Install command")
    parser.add_argument("--usage", help="Usage command")
    parser.add_argument("--license", default="MIT", help="License label")
    parser.add_argument("--theme", default="tokyonight", help="GitHub Stats Extended theme")
    parser.add_argument("--include-stats", action="store_true", help="Include maintainer stats in project README")
    parser.add_argument("--profile", action="store_true", help="Render a profile README instead of a project README")
    return parser.parse_args()


def main() -> None:
    args = parse_args()
    args.project = args.project.resolve()
    meta = detect_metadata(args.project)
    remote_owner, remote_repo = detect_remote(args.project)
    owner = args.owner or remote_owner
    repo = args.repo or remote_repo

    content = profile_readme(args, owner) if args.profile else project_readme(args, meta, owner, repo)
    if args.output:
        args.output.parent.mkdir(parents=True, exist_ok=True)
        args.output.write_text(content, encoding="utf-8")
    else:
        print(content)


if __name__ == "__main__":
    main()
