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
    license: Option<String>,
    theme: String,
    template: String,
    screenshot: Option<String>,
    demo: Option<String>,
    docs_url: Option<String>,
    include_stats: bool,
    no_toc: bool,
    profile: bool,
}

#[derive(Default)]
struct Metadata {
    name: Option<String>,
    description: Option<String>,
    package_manager: Option<String>,
    install: Option<String>,
    usage: Option<String>,
    dev: Option<String>,
    build: Option<String>,
    test: Option<String>,
    lint: Option<String>,
    format: Option<String>,
    docs_path: Option<String>,
    examples_path: Option<String>,
    screenshot_path: Option<String>,
    has_env_example: bool,
    has_contributing: bool,
    has_code_of_conduct: bool,
    has_changelog: bool,
    has_ci: bool,
    has_docker: bool,
    has_src: bool,
    has_scripts: bool,
    has_tests: bool,
    has_references: bool,
    has_assets: bool,
    has_license: bool,
    license_label: Option<String>,
    kind: String,
}

fn main() {
    let args = parse_args();
    let mut metadata = detect_metadata(&args.project);
    let (remote_owner, remote_repo) = detect_remote(&args.project);
    let owner = args.owner.clone().or(remote_owner);
    let repo = args.repo.clone().or(remote_repo);

    if let Some(language) = &args.language {
        metadata.kind = infer_kind(&args.project, &metadata, language);
    }

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
        theme: "tokyonight".to_string(),
        template: "auto".to_string(),
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
            "--license" => args.license = Some(next_value(&mut iter, "--license")),
            "--theme" => args.theme = next_value(&mut iter, "--theme"),
            "--template" => args.template = next_value(&mut iter, "--template"),
            "--screenshot" => args.screenshot = Some(next_value(&mut iter, "--screenshot")),
            "--demo" => args.demo = Some(next_value(&mut iter, "--demo")),
            "--docs-url" => args.docs_url = Some(next_value(&mut iter, "--docs-url")),
            "--include-stats" => args.include_stats = true,
            "--no-toc" => args.no_toc = true,
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
  --install TEXT       Install command override
  --usage TEXT         Usage command override
  --license TEXT       License label override
  --template NAME      auto, library, app, cli, docs, profile (default: auto)
  --screenshot PATH    Screenshot, GIF, or image path/URL to feature
  --demo URL           Demo, video, hosted app, or package link
  --docs-url URL       External documentation link
  --theme TEXT         GitHub Stats Extended theme (default: tokyonight)
  --include-stats      Include maintainer stats in project README
  --no-toc             Omit the generated contents block
  --profile            Render a profile README instead of a project README
  --help               Show this help
"
    );
}

fn detect_metadata(project: &Path) -> Metadata {
    let mut metadata = Metadata::default();
    detect_package_metadata(project, &mut metadata);
    detect_repo_files(project, &mut metadata);
    let language = detect_language(project);
    metadata.kind = infer_kind(project, &metadata, &language);
    metadata
}

fn detect_package_metadata(project: &Path, metadata: &mut Metadata) {
    let package_json = project.join("package.json");
    if package_json.exists() {
        if let Ok(text) = fs::read_to_string(&package_json) {
            metadata.name = json_string(&text, "name");
            metadata.description = json_string(&text, "description");
            metadata.package_manager = Some(detect_js_package_manager(project));
            metadata.install = Some(match metadata.package_manager.as_deref() {
                Some("bun") => "bun install".to_string(),
                Some("pnpm") => "pnpm install".to_string(),
                Some("yarn") => "yarn install".to_string(),
                _ => "npm install".to_string(),
            });
            metadata.dev = script_command(&metadata.package_manager, "dev", &text);
            metadata.build = script_command(&metadata.package_manager, "build", &text);
            metadata.test = script_command(&metadata.package_manager, "test", &text);
            metadata.lint = script_command(&metadata.package_manager, "lint", &text);
            metadata.format = script_command(&metadata.package_manager, "format", &text);
            metadata.usage = metadata.dev.clone().or_else(|| script_command(&metadata.package_manager, "start", &text));
        }
    }

    let pyproject = project.join("pyproject.toml");
    if pyproject.exists() && metadata.name.is_none() {
        if let Ok(text) = fs::read_to_string(&pyproject) {
            metadata.name = toml_string(&text, "name");
            metadata.description = toml_string(&text, "description");
            metadata.package_manager = Some(if project.join("uv.lock").exists() { "uv" } else { "pip" }.to_string());
            metadata.install = Some(if project.join("uv.lock").exists() {
                "uv sync".to_string()
            } else {
                "pip install -e .".to_string()
            });
            if let Some(name) = &metadata.name {
                metadata.usage = Some(format!("python -m {}", name.replace('-', "_")));
            }
            metadata.test = if text.contains("pytest") { Some("pytest".to_string()) } else { None };
        }
    }

    let cargo = project.join("Cargo.toml");
    if cargo.exists() && metadata.name.is_none() {
        if let Ok(text) = fs::read_to_string(&cargo) {
            metadata.name = toml_string(&text, "name");
            metadata.description = toml_string(&text, "description");
            metadata.package_manager = Some("cargo".to_string());
            metadata.install = Some("cargo build --release".to_string());
            metadata.build = Some("cargo build --release".to_string());
            metadata.test = Some("cargo test".to_string());
            metadata.format = Some("cargo fmt".to_string());
            if let Some(name) = &metadata.name {
                metadata.usage = Some(format!("cargo run --bin {name}"));
            }
        }
    }

    if project.join("go.mod").exists() && metadata.name.is_none() {
        metadata.package_manager = Some("go".to_string());
        metadata.install = Some("go mod download".to_string());
        metadata.build = Some("go build ./...".to_string());
        metadata.test = Some("go test ./...".to_string());
        metadata.usage = Some("go run .".to_string());
    }
}

fn detect_repo_files(project: &Path, metadata: &mut Metadata) {
    metadata.docs_path = first_existing(project, &["docs", "documentation", "site"]);
    metadata.examples_path = first_existing(project, &["examples", "example", "demo", "demos"]);
    metadata.screenshot_path = first_existing(project, &[
        "assets/screenshot.png",
        "assets/demo.gif",
        "docs/screenshot.png",
        "media/demo.gif",
        "screenshot.png",
        "demo.gif",
    ]);
    metadata.has_env_example = any_exists(project, &[".env.example", ".env.sample", "example.env"]);
    metadata.has_contributing = any_exists(project, &["CONTRIBUTING.md", ".github/CONTRIBUTING.md"]);
    metadata.has_code_of_conduct = any_exists(project, &["CODE_OF_CONDUCT.md", ".github/CODE_OF_CONDUCT.md"]);
    metadata.has_changelog = any_exists(project, &["CHANGELOG.md", "changelog.md", "CHANGES.md"]);
    metadata.has_ci = dir_has_files(&project.join(".github/workflows"));
    metadata.has_docker = any_exists(project, &["Dockerfile", "docker-compose.yml", "compose.yml"]);
    metadata.has_src = project.join("src").exists();
    metadata.has_scripts = project.join("scripts").exists();
    metadata.has_tests = any_exists(project, &["test", "tests", "__tests__", "spec"]);
    metadata.has_references = any_exists(project, &["references", "reference"]);
    metadata.has_assets = any_exists(project, &["assets", "public", "static", "media"]);
    metadata.has_license = any_exists(project, &["LICENSE", "LICENSE.md", "COPYING"]);
    metadata.license_label = if metadata.has_license { detect_license(project) } else { None };
}

fn detect_js_package_manager(project: &Path) -> String {
    if project.join("bun.lock").exists() || project.join("bun.lockb").exists() {
        "bun".to_string()
    } else if project.join("pnpm-lock.yaml").exists() {
        "pnpm".to_string()
    } else if project.join("yarn.lock").exists() {
        "yarn".to_string()
    } else {
        "npm".to_string()
    }
}

fn script_command(manager: &Option<String>, script: &str, package_json: &str) -> Option<String> {
    if !package_json.contains(&format!("\"{script}\"")) {
        return None;
    }
    Some(match manager.as_deref() {
        Some("bun") => format!("bun run {script}"),
        Some("pnpm") => format!("pnpm {script}"),
        Some("yarn") => format!("yarn {script}"),
        _ => format!("npm run {script}"),
    })
}

fn first_existing(project: &Path, candidates: &[&str]) -> Option<String> {
    candidates
        .iter()
        .find(|candidate| project.join(candidate).exists())
        .map(|candidate| candidate.to_string())
}

fn any_exists(project: &Path, candidates: &[&str]) -> bool {
    candidates.iter().any(|candidate| project.join(candidate).exists())
}

fn dir_has_files(path: &Path) -> bool {
    fs::read_dir(path).map(|mut entries| entries.next().is_some()).unwrap_or(false)
}

fn detect_license(project: &Path) -> Option<String> {
    let license_path = ["LICENSE", "LICENSE.md", "COPYING"]
        .iter()
        .map(|file| project.join(file))
        .find(|path| path.exists())?;
    let text = fs::read_to_string(license_path).ok()?.to_lowercase();
    if text.contains("mit license") {
        Some("MIT".to_string())
    } else if text.contains("apache license") {
        Some("Apache-2.0".to_string())
    } else if text.contains("gnu general public license") {
        Some("GPL".to_string())
    } else if text.contains("bsd") {
        Some("BSD".to_string())
    } else {
        Some("See LICENSE".to_string())
    }
}

fn infer_kind(project: &Path, metadata: &Metadata, language: &str) -> String {
    if project.join("README.md").exists() && project.file_name().and_then(|name| name.to_str()).is_some() {
        // Keep falling through; project READMEs should still infer by code shape.
    }
    if project.join("package.json").exists() {
        if let Ok(text) = fs::read_to_string(project.join("package.json")) {
            if text.contains("\"bin\"") {
                return "cli".to_string();
            }
            if text.contains("next") || text.contains("vite") || text.contains("react") || text.contains("svelte") || text.contains("vue") {
                return "app".to_string();
            }
        }
    }
    if project.join("src/main.rs").exists() || project.join("cmd").exists() {
        return "cli".to_string();
    }
    if metadata.docs_path.is_some() && language == "Markdown" {
        return "docs".to_string();
    }
    if matches!(language, "Rust" | "Go" | "Python") && project.join("src").exists() {
        return "library".to_string();
    }
    "project".to_string()
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
    let language = args
        .language
        .clone()
        .unwrap_or_else(|| detect_language(&args.project));
    let kind = if args.template == "auto" { meta.kind.as_str() } else { args.template.as_str() };
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
        .unwrap_or_else(|| default_tagline(kind));
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
    let license = args
        .license
        .as_deref()
        .or(meta.license_label.as_deref())
        .unwrap_or("MIT");

    let mut sections = Vec::new();
    sections.push(hero(&title, tagline, &language, owner, repo, args, meta));
    if !args.no_toc {
        sections.push(contents(kind, args, meta));
    }
    sections.push(overview(kind, tagline));
    sections.push(highlights(kind, &language, meta));
    if let Some(media) = args.screenshot.as_deref().or(meta.screenshot_path.as_deref()) {
        sections.push(showcase(media, args.demo.as_deref()));
    } else if let Some(demo) = args.demo.as_deref() {
        sections.push(format!("## Demo\n\nTry it here: [{demo}]({demo})\n"));
    }
    sections.push(quick_start(install, usage, meta));
    sections.push(usage_section(kind, usage));
    sections.push(commands_section(meta));
    if meta.has_env_example || meta.has_docker || kind == "app" {
        sections.push(configuration_section(meta));
    }
    sections.push(project_structure_section(meta));
    if let Some(docs) = args.docs_url.as_deref().or(meta.docs_path.as_deref()) {
        sections.push(format!("## Documentation\n\nRead the full documentation in [{docs}]({docs}).\n"));
    }
    if let Some(examples) = &meta.examples_path {
        sections.push(format!("## Examples\n\nExplore working examples in [`{examples}`]({examples}).\n"));
    }
    if args.include_stats {
        if let Some(owner) = owner {
            sections.push(stats_section(owner, &args.theme));
        }
    }
    sections.push(community_section(meta));
    sections.push(quality_section(kind));
    sections.push(license_section(license, meta));
    sections.join("\n")
}

fn hero(
    title: &str,
    tagline: &str,
    language: &str,
    owner: Option<&str>,
    repo: Option<&str>,
    args: &Args,
    meta: &Metadata,
) -> String {
    let mut badges = vec![badge_img(language, &shield_url("built with", language, language_color(language), language_logo(language)))];
    if let (Some(owner), Some(repo)) = (owner, repo) {
        badges.push(github_badge("Stars", "stars", owner, repo, "stargazers"));
        badges.push(github_badge("Issues", "issues", owner, repo, "issues"));
        if meta.has_ci {
            badges.push(badge_img("CI", &format!("https://img.shields.io/github/actions/workflow/status/{owner}/{repo}/ci.yml?branch=main&style=for-the-badge&label=CI")));
        }
        badges.push(github_badge("License", "license", owner, repo, "blob/main/LICENSE"));
    } else {
        badges.push(badge_img("Status", &shield_url("status", "active", "success", None)));
    }
    if meta.has_docker {
        badges.push(badge_img("Docker", &shield_url("docker", "ready", "2496ed", Some("docker"))));
    }

    let demo_link = args
        .demo
        .as_deref()
        .map(|demo| format!("  <a href=\"{demo}\">Demo</a> ·\n"))
        .unwrap_or_default();
    let docs_link = args
        .docs_url
        .as_deref()
        .or(meta.docs_path.as_deref())
        .map(|docs| format!("  <a href=\"{docs}\">Docs</a> ·\n"))
        .unwrap_or_default();

    format!(
        "\
<h1 align=\"center\">{title}</h1>
<p align=\"center\"><strong>{tagline}</strong></p>
<p align=\"center\">
  {}
</p>

<p align=\"center\">
{demo_link}{docs_link}  <a href=\"#quick-start\">Quick Start</a> ·
  <a href=\"#usage\">Usage</a> ·
  <a href=\"#contributing\">Contributing</a>
</p>
",
        badges.join(" ")
    )
}

fn contents(kind: &str, args: &Args, meta: &Metadata) -> String {
    let mut links = vec![
        "Overview",
        "Highlights",
        "Quick Start",
        "Usage",
        "Commands",
    ];
    if args.screenshot.is_some() || meta.screenshot_path.is_some() || args.demo.is_some() {
        links.insert(2, "Demo");
    }
    if meta.has_env_example || meta.has_docker || kind == "app" {
        links.push("Configuration");
    }
    links.push("Project Structure");
    if args.docs_url.is_some() || meta.docs_path.is_some() {
        links.push("Documentation");
    }
    if meta.examples_path.is_some() {
        links.push("Examples");
    }
    if args.include_stats {
        links.push("Stats");
    }
    links.extend(["Contributing", "Quality Checklist", "License"]);

    let items = links
        .into_iter()
        .map(|link| format!("- [{}](#{})", link, anchor(link)))
        .collect::<Vec<_>>()
        .join("\n");
    format!("## Contents\n\n{items}\n")
}

fn overview(kind: &str, tagline: &str) -> String {
    format!(
        "\
## Overview

{tagline}

This README is organized for the fastest path from first impression to working result: understand the project, install it, run it, and know where to go next.

{}
",
        match kind {
            "cli" => "The command-line workflow is kept close to the top so users can copy, run, and verify the tool quickly.",
            "app" => "The application workflow emphasizes local setup, configuration, demo media, and deployment-ready commands.",
            "library" => "The library workflow separates installation, minimal usage, and development commands so integration is straightforward.",
            "docs" => "The documentation workflow prioritizes navigation, scope, examples, and contribution paths.",
            _ => "The project workflow keeps useful details visible without turning the README into a wall of text.",
        }
    )
}

fn highlights(kind: &str, language: &str, meta: &Metadata) -> String {
    let mut bullets = Vec::new();
    bullets.push(match kind {
        "cli" => "Copy-pasteable command workflow for fast terminal use".to_string(),
        "app" => "Local development path with install, run, and configuration steps".to_string(),
        "library" => "Integration-focused structure with setup and usage examples".to_string(),
        "docs" => "Documentation-first structure with clear navigation".to_string(),
        _ => "Reader-first structure that answers what, why, and how quickly".to_string(),
    });
    bullets.push(format!("{language} project signals detected from the repository"));
    if let Some(manager) = &meta.package_manager {
        bullets.push(format!("Uses `{manager}` workflow conventions"));
    }
    if meta.has_ci {
        bullets.push("Continuous integration metadata detected".to_string());
    }
    if meta.docs_path.is_some() {
        bullets.push("Dedicated documentation path detected".to_string());
    }
    if meta.examples_path.is_some() {
        bullets.push("Examples directory detected for deeper learning".to_string());
    }

    format!(
        "## Highlights\n\n{}\n",
        bullets
            .into_iter()
            .map(|bullet| format!("- {bullet}"))
            .collect::<Vec<_>>()
            .join("\n")
    )
}

fn showcase(media: &str, demo: Option<&str>) -> String {
    let alt = if media.ends_with(".gif") { "Demo GIF" } else { "Project screenshot" };
    let demo_line = demo
        .map(|url| format!("\n\nTry the live demo: [{url}]({url})"))
        .unwrap_or_default();
    format!(
        "\
## Demo

<p align=\"center\">
  <img src=\"{media}\" alt=\"{alt}\" width=\"860\">
</p>{demo_line}
"
    )
}

fn quick_start(install: &str, usage: &str, meta: &Metadata) -> String {
    let env_line = if meta.has_env_example { "cp .env.example .env\n" } else { "" };
    format!(
        "\
## Quick Start

```bash
{install}
{env_line}{usage}
```

The quick start intentionally includes only the commands needed to reach a working result.
"
    )
}

fn usage_section(kind: &str, usage: &str) -> String {
    let guidance = match kind {
        "cli" => "Replace the arguments with your own input and add the most common flags here.",
        "app" => "Open the local URL printed by the dev server and verify the primary workflow.",
        "library" => "Show the smallest real integration example here, then link to advanced examples below.",
        _ => "Adapt this example to the main workflow your users should try first.",
    };
    format!(
        "\
## Usage

```bash
{usage}
```

{guidance}
"
    )
}

fn commands_section(meta: &Metadata) -> String {
    let mut rows = Vec::new();
    if let Some(command) = &meta.install {
        rows.push(("Install dependencies", command.as_str()));
    }
    if let Some(command) = &meta.dev {
        rows.push(("Start development", command.as_str()));
    }
    if let Some(command) = &meta.build {
        rows.push(("Build", command.as_str()));
    }
    if let Some(command) = &meta.test {
        rows.push(("Test", command.as_str()));
    }
    if let Some(command) = &meta.lint {
        rows.push(("Lint", command.as_str()));
    }
    if let Some(command) = &meta.format {
        rows.push(("Format", command.as_str()));
    }

    if rows.is_empty() {
        return "## Commands\n\n| Task | Command |\n| --- | --- |\n| Install | `# Add install command` |\n| Run | `# Add run command` |\n| Test | `# Add test command` |\n".to_string();
    }

    let body = rows
        .into_iter()
        .map(|(task, command)| format!("| {task} | `{command}` |"))
        .collect::<Vec<_>>()
        .join("\n");
    format!("## Commands\n\n| Task | Command |\n| --- | --- |\n{body}\n")
}

fn configuration_section(meta: &Metadata) -> String {
    let mut lines = Vec::new();
    if meta.has_env_example {
        lines.push("- Copy `.env.example` to `.env` and fill in required values.");
    }
    if meta.has_docker {
        lines.push("- Docker files are present; document image names, ports, volumes, and required environment variables.");
    }
    if lines.is_empty() {
        lines.push("- Document environment variables, service credentials, config files, and deployment settings here.");
    }
    lines.push("- Never commit secrets. Use local `.env` files or your deployment platform's secret manager.");
    format!("## Configuration\n\n{}\n", lines.join("\n"))
}

fn project_structure_section(meta: &Metadata) -> String {
    let mut rows = Vec::new();
    if meta.docs_path.is_some() {
        rows.push(("docs/", "Long-form documentation and guides"));
    }
    if meta.examples_path.is_some() {
        rows.push(("examples/", "Runnable examples and demos"));
    }
    if meta.has_ci {
        rows.push((".github/workflows/", "CI and automation workflows"));
    }
    if meta.has_docker {
        rows.push(("Dockerfile", "Container build instructions"));
    }
    if meta.has_src {
        rows.push(("src/", "Core source code"));
    }
    if meta.has_scripts {
        rows.push(("scripts/", "Automation and command-line helpers"));
    }
    if meta.has_tests {
        rows.push(("tests/", "Automated tests and fixtures"));
    }
    if meta.has_references {
        rows.push(("references/", "Detailed reference material loaded when needed"));
    }
    if meta.has_assets {
        rows.push(("assets/", "Images, screenshots, icons, or static assets"));
    }
    rows.push(("README.md", "Project entry point and usage guide"));

    let body = rows
        .into_iter()
        .map(|(path, description)| format!("| `{path}` | {description} |"))
        .collect::<Vec<_>>()
        .join("\n");
    format!("## Project Structure\n\n| Path | Purpose |\n| --- | --- |\n{body}\n")
}

fn stats_section(owner: &str, theme: &str) -> String {
    format!(
        "\
## Stats

<p align=\"center\">
  <img alt=\"GitHub stats\" src=\"https://github-stats-extended.vercel.app/api?username={owner}&show_icons=true&theme={theme}\" />
  <img alt=\"Top languages\" src=\"https://github-stats-extended.vercel.app/api/top-langs/?username={owner}&layout=compact&theme={theme}\" />
</p>
"
    )
}

fn community_section(meta: &Metadata) -> String {
    let mut lines = vec!["Issues and pull requests are welcome. Keep reports focused and include reproduction steps for bugs.".to_string()];
    if meta.has_contributing {
        lines.push("Read [`CONTRIBUTING.md`](CONTRIBUTING.md) before opening larger changes.".to_string());
    }
    if meta.has_code_of_conduct {
        lines.push("This project has a code of conduct; participate respectfully.".to_string());
    }
    if meta.has_changelog {
        lines.push("Release notes are tracked in [`CHANGELOG.md`](CHANGELOG.md).".to_string());
    }
    format!("## Contributing\n\n{}\n", lines.join("\n\n"))
}

fn quality_section(kind: &str) -> String {
    let first = match kind {
        "cli" => "Run the command with the documented example input.",
        "app" => "Start the app locally and capture or update the screenshot/GIF when UI changes.",
        "library" => "Run the minimal usage example against a clean checkout.",
        _ => "Confirm the quick-start path works from a clean checkout.",
    };
    format!(
        "\
## Quality Checklist

- {first}
- Test every command copied into this README.
- Verify links, badges, screenshots, and dynamic cards after major changes.
- Keep the first screen focused on what the project does, why it matters, and how to try it.
"
    )
}

fn license_section(license: &str, meta: &Metadata) -> String {
    if meta.has_license {
        format!("## License\n\n{license}. See [`LICENSE`](LICENSE) for details.\n")
    } else {
        format!("## License\n\n{license}. Add a `LICENSE` file before publishing if this project is open source.\n")
    }
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
<p align=\"center\"><strong>{tagline}</strong></p>

<p align=\"center\">
  <img alt=\"GitHub\" src=\"{}\" />
  <img alt=\"Open Source\" src=\"{}\" />
</p>

## Focus

- Building reliable software with clear user value
- Turning rough ideas into polished tools
- Writing documentation that helps people move quickly

## Featured Work

| Project | Why it matters |
| --- | --- |
| [Project One](https://github.com/{username}) | Add the outcome, not just the tech stack. |
| [Project Two](https://github.com/{username}) | Link to a demo, package, or case study when possible. |

## Stack

Use this section for the tools you want people to associate with your work. Keep it current and avoid listing every tool you have ever tried.

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

fn default_tagline(kind: &str) -> &'static str {
    match kind {
        "cli" => "A focused command-line tool built for fast, reliable workflows.",
        "app" => "A polished application with a clear setup path and practical defaults.",
        "library" => "A developer-friendly library with clean setup and usage examples.",
        "docs" => "A documentation hub designed for fast navigation and contribution.",
        _ => "A polished, useful project built for fast adoption.",
    }
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

fn language_color(language: &str) -> &'static str {
    match language {
        "Rust" => "b7410e",
        "TypeScript" => "3178c6",
        "JavaScript" => "f7df1e",
        "Python" => "3776ab",
        "Go" => "00add8",
        "Java" => "f89820",
        _ => "2ea44f",
    }
}

fn language_logo(language: &str) -> Option<&'static str> {
    match language {
        "Rust" => Some("rust"),
        "TypeScript" => Some("typescript"),
        "JavaScript" => Some("javascript"),
        "Python" => Some("python"),
        "Go" => Some("go"),
        "Java" => Some("openjdk"),
        _ => None,
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

fn anchor(value: &str) -> String {
    value
        .to_lowercase()
        .chars()
        .filter_map(|ch| {
            if ch.is_ascii_alphanumeric() {
                Some(ch)
            } else if ch.is_whitespace() || ch == '-' {
                Some('-')
            } else {
                None
            }
        })
        .collect::<String>()
}
