use std::collections::{BTreeMap, HashSet};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Default)]
struct Args {
    project: PathBuf,
    output: Option<PathBuf>,
    owner: Option<String>,
    repo: Option<String>,
    name: Option<String>,
    tagline: Option<String>,
    language: Option<String>,
    install: Option<String>,
    usage: Option<String>,
    license: String,
    theme: String,
    include_stats: bool,
    profile: bool,
}

#[derive(Default)]
struct Metadata {
    name: Option<String>,
    description: Option<String>,
    install: Option<String>,
    usage: Option<String>,
}

fn main() {
    let args = parse_args();
    let metadata = detect_metadata(&args.project);
    let (remote_owner, remote_repo) = detect_remote(&args.project);
    let owner = args.owner.clone().or(remote_owner);
    let repo = args.repo.clone().or(remote_repo);

    let content = if args.profile {
        profile_readme(&args, owner.as_deref())
    } else {
        project_readme(&args, &metadata, owner.as_deref(), repo.as_deref())
    };

    if let Some(output) = &args.output {
        if let Some(parent) = output.parent() {
            fs::create_dir_all(parent).expect("failed to create output directory");
        }
        fs::write(output, content).expect("failed to write README");
    } else {
        print!("{content}");
    }
}

fn parse_args() -> Args {
    let mut args = Args {
        project: env::current_dir().expect("failed to read current directory"),
        license: "MIT".to_string(),
        theme: "tokyonight".to_string(),
        ..Args::default()
    };

    let mut iter = env::args().skip(1);
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--help" | "-h" => {
                print_help();
                std::process::exit(0);
            }
            "--project" => args.project = PathBuf::from(next_value(&mut iter, "--project")),
            "--output" => args.output = Some(PathBuf::from(next_value(&mut iter, "--output"))),
            "--owner" => args.owner = Some(next_value(&mut iter, "--owner")),
            "--repo" => args.repo = Some(next_value(&mut iter, "--repo")),
            "--name" => args.name = Some(next_value(&mut iter, "--name")),
            "--tagline" => args.tagline = Some(next_value(&mut iter, "--tagline")),
            "--language" => args.language = Some(next_value(&mut iter, "--language")),
            "--install" => args.install = Some(next_value(&mut iter, "--install")),
            "--usage" => args.usage = Some(next_value(&mut iter, "--usage")),
            "--license" => args.license = next_value(&mut iter, "--license"),
            "--theme" => args.theme = next_value(&mut iter, "--theme"),
            "--include-stats" => args.include_stats = true,
            "--profile" => args.profile = true,
            unknown => {
                eprintln!("unknown argument: {unknown}");
                print_help();
                std::process::exit(2);
            }
        }
    }

    args.project = fs::canonicalize(&args.project).unwrap_or(args.project);
    args
}

fn next_value(iter: &mut impl Iterator<Item = String>, flag: &str) -> String {
    iter.next().unwrap_or_else(|| {
        eprintln!("{flag} requires a value");
        std::process::exit(2);
    })
}

fn print_help() {
    println!(
        "\
Render a polished GitHub README draft from repo metadata and CLI inputs.

Usage:
  render_readme [options]

Options:
  --project PATH       Repository to inspect
  --output PATH        Path to write README.md
  --owner NAME         GitHub owner or profile username
  --repo NAME          GitHub repository name
  --name TEXT          Project, profile, or organization display name
  --tagline TEXT       Short value proposition
  --language TEXT      Primary language or stack label
  --install TEXT       Install command
  --usage TEXT         Usage command
  --license TEXT       License label (default: MIT)
  --theme TEXT         GitHub Stats Extended theme (default: tokyonight)
  --include-stats      Include maintainer stats in project README
  --profile            Render a profile README instead of a project README
  --help               Show this help
"
    );
}

fn detect_metadata(project: &Path) -> Metadata {
    let mut metadata = Metadata::default();

    let package_json = project.join("package.json");
    if package_json.exists() {
        if let Ok(text) = fs::read_to_string(&package_json) {
            metadata.name = json_string(&text, "name");
            metadata.description = json_string(&text, "description");
            metadata.install = Some("npm install".to_string());
            if text.contains("\"dev\"") {
                metadata.usage = Some("npm run dev".to_string());
            } else if text.contains("\"start\"") {
                metadata.usage = Some("npm start".to_string());
            }
        }
    }

    let pyproject = project.join("pyproject.toml");
    if pyproject.exists() && metadata.name.is_none() {
        if let Ok(text) = fs::read_to_string(&pyproject) {
            metadata.name = toml_string(&text, "name");
            metadata.description = toml_string(&text, "description");
            metadata.install = Some("pip install -e .".to_string());
            if let Some(name) = &metadata.name {
                metadata.usage = Some(format!("python -m {}", name.replace('-', "_")));
            }
        }
    }

    let cargo = project.join("Cargo.toml");
    if cargo.exists() && metadata.name.is_none() {
        if let Ok(text) = fs::read_to_string(&cargo) {
            metadata.name = toml_string(&text, "name");
            metadata.description = toml_string(&text, "description");
            metadata.install = Some("cargo build --release".to_string());
            if let Some(name) = &metadata.name {
                metadata.usage = Some(format!("cargo run --bin {name}"));
            }
        }
    }

    metadata
}

fn json_string(text: &str, key: &str) -> Option<String> {
    quoted_assignment(text, &format!("\"{key}\""), ':')
}

fn toml_string(text: &str, key: &str) -> Option<String> {
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('#') {
            continue;
        }
        if let Some(value) = trimmed.strip_prefix(&format!("{key} =")) {
            return extract_quoted(value.trim());
        }
    }
    None
}

fn quoted_assignment(text: &str, key: &str, separator: char) -> Option<String> {
    let key_index = text.find(key)?;
    let rest = &text[key_index + key.len()..];
    let separator_index = rest.find(separator)?;
    extract_quoted(rest[separator_index + 1..].trim_start())
}

fn extract_quoted(value: &str) -> Option<String> {
    let mut chars = value.chars();
    if chars.next()? != '"' {
        return None;
    }
    let mut result = String::new();
    let mut escaped = false;
    for ch in chars {
        if escaped {
            result.push(match ch {
                'n' => '\n',
                't' => '\t',
                '"' => '"',
                '\\' => '\\',
                other => other,
            });
            escaped = false;
        } else if ch == '\\' {
            escaped = true;
        } else if ch == '"' {
            return Some(result);
        } else {
            result.push(ch);
        }
    }
    None
}

fn detect_remote(project: &Path) -> (Option<String>, Option<String>) {
    let config = project.join(".git/config");
    let Ok(text) = fs::read_to_string(config) else {
        return (None, None);
    };

    for line in text.lines() {
        let trimmed = line.trim();
        if let Some(url) = trimmed.strip_prefix("url = ") {
            if let Some((owner, repo)) = parse_github_remote(url) {
                return (Some(owner), Some(repo));
            }
        }
    }
    (None, None)
}

fn parse_github_remote(url: &str) -> Option<(String, String)> {
    let marker = "github.com";
    let index = url.find(marker)?;
    let mut rest = &url[index + marker.len()..];
    rest = rest.trim_start_matches([':', '/']);
    let parts: Vec<&str> = rest.split('/').collect();
    if parts.len() < 2 {
        return None;
    }
    let repo = parts[1].trim_end_matches(".git");
    Some((parts[0].to_string(), repo.to_string()))
}

fn detect_language(project: &Path) -> String {
    let mut counts: BTreeMap<&str, usize> = BTreeMap::new();
    walk_languages(project, &mut counts);
    counts
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .map(|(language, _)| language.to_string())
        .unwrap_or_else(|| "Markdown".to_string())
}

fn walk_languages(path: &Path, counts: &mut BTreeMap<&'static str, usize>) {
    let ignored: HashSet<&str> = [".git", "node_modules", "dist", "build", "target", ".venv", "vendor"]
        .iter()
        .copied()
        .collect();

    let Ok(entries) = fs::read_dir(path) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        let file_name = entry.file_name();
        let file_name = file_name.to_string_lossy();
        if ignored.contains(file_name.as_ref()) {
            continue;
        }
        if path.is_dir() {
            walk_languages(&path, counts);
        } else if let Some(language) = language_for_path(&path) {
            *counts.entry(language).or_insert(0) += 1;
        }
    }
}

fn language_for_path(path: &Path) -> Option<&'static str> {
    match path.extension()?.to_string_lossy().as_ref() {
        "js" | "jsx" => Some("JavaScript"),
        "ts" | "tsx" => Some("TypeScript"),
        "py" => Some("Python"),
        "go" => Some("Go"),
        "rs" => Some("Rust"),
        "java" => Some("Java"),
        "kt" => Some("Kotlin"),
        "cs" => Some("C#"),
        "php" => Some("PHP"),
        "rb" => Some("Ruby"),
        "swift" => Some("Swift"),
        "dart" => Some("Dart"),
        "vue" => Some("Vue"),
        "svelte" => Some("Svelte"),
        _ => None,
    }
}

fn project_readme(args: &Args, meta: &Metadata, owner: Option<&str>, repo: Option<&str>) -> String {
    let name = args
        .name
        .as_deref()
        .or(meta.name.as_deref())
        .or(repo)
        .unwrap_or("Project");
    let title = clean_name(name);
    let tagline = args
        .tagline
        .as_deref()
        .or(meta.description.as_deref())
        .unwrap_or("A polished, useful project built for fast adoption.");
    let language = args
        .language
        .clone()
        .unwrap_or_else(|| detect_language(&args.project));
    let install = args
        .install
        .as_deref()
        .or(meta.install.as_deref())
        .unwrap_or("# Add install command");
    let usage = args
        .usage
        .as_deref()
        .or(meta.usage.as_deref())
        .unwrap_or("# Add usage command");

    let mut badges = vec![badge_img(&language, &shield_url("built with", &language, "2ea44f", None))];
    if let (Some(owner), Some(repo)) = (owner, repo) {
        badges.push(github_badge("Stars", "stars", owner, repo, "stargazers"));
        badges.push(github_badge("Issues", "issues", owner, repo, "issues"));
        badges.push(github_badge("License", "license", owner, repo, "blob/main/LICENSE"));
    } else {
        badges.push(badge_img("Status", &shield_url("status", "active", "success", None)));
    }

    let stats = if args.include_stats {
        owner.map(|owner| {
            format!(
                "\n## Stats\n\n<p align=\"center\">\n  <img alt=\"GitHub stats\" src=\"https://github-stats-extended.vercel.app/api?username={owner}&show_icons=true&theme={}\" />\n</p>\n",
                args.theme
            )
        })
        .unwrap_or_default()
    } else {
        String::new()
    };

    format!(
        "\
<h1 align=\"center\">{title}</h1>
<p align=\"center\">{tagline}</p>
<p align=\"center\">
  {}
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

{}
",
        badges.join(" "),
        args.license
    )
}

fn profile_readme(args: &Args, owner: Option<&str>) -> String {
    let username = owner.or(args.owner.as_deref()).unwrap_or("USERNAME");
    let name = args.name.as_deref().unwrap_or(username);
    let tagline = args
        .tagline
        .as_deref()
        .unwrap_or("Building useful software and sharing what I learn.");

    format!(
        "\
<h1 align=\"center\">Hi, I'm {name}</h1>
<p align=\"center\">{tagline}</p>

## Focus

- Building reliable software with clear user value
- Turning rough ideas into polished tools
- Writing documentation that helps people move quickly

## Stack

<p>
  <img alt=\"GitHub\" src=\"{}\" />
  <img alt=\"Open Source\" src=\"{}\" />
</p>

## Featured Work

- Add featured repositories here with one-line outcomes.
- Include demos, screenshots, or package links where useful.

## Stats

<p align=\"center\">
  <img alt=\"GitHub stats\" src=\"https://github-stats-extended.vercel.app/api?username={username}&show_icons=true&theme={}\" />
  <img alt=\"Top languages\" src=\"https://github-stats-extended.vercel.app/api/top-langs/?username={username}&layout=compact&theme={}\" />
</p>

## Contact

- GitHub: [@{username}](https://github.com/{username})
",
        shield_url("GitHub", username, "181717", Some("github")),
        shield_url("open source", "builder", "2ea44f", None),
        args.theme,
        args.theme
    )
}

fn clean_name(value: &str) -> String {
    let value = value.trim();
    if value.contains(' ') || value.chars().any(|ch| ch.is_uppercase()) {
        return value.to_string();
    }
    value.replace(['_', '-'], " ").to_title_case()
}

trait TitleCase {
    fn to_title_case(&self) -> String;
}

impl TitleCase for str {
    fn to_title_case(&self) -> String {
        self.split_whitespace()
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                    None => String::new(),
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
}

fn shield_url(label: &str, message: &str, color: &str, logo: Option<&str>) -> String {
    let mut url = format!(
        "https://img.shields.io/badge/{}-{}-{}?style=for-the-badge",
        url_encode(label),
        url_encode(message),
        color
    );
    if let Some(logo) = logo {
        url.push_str("&logo=");
        url.push_str(&url_encode(logo));
        url.push_str("&logoColor=white");
    }
    url
}

fn badge_img(label: &str, src: &str) -> String {
    format!("<img alt=\"{}\" src=\"{}\" />", escape_html(label), escape_html(src))
}

fn github_badge(label: &str, path: &str, owner: &str, repo: &str, target: &str) -> String {
    let image = format!("https://img.shields.io/github/{path}/{owner}/{repo}?style=for-the-badge");
    format!(
        "<a href=\"https://github.com/{owner}/{repo}/{target}\">{}</a>",
        badge_img(label, &image)
    )
}

fn url_encode(value: &str) -> String {
    let mut encoded = String::new();
    for byte in value.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => encoded.push(byte as char),
            _ => encoded.push_str(&format!("%{byte:02X}")),
        }
    }
    encoded
}

fn escape_html(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}
