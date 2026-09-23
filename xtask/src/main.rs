use std::{
    env,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

const REQUIRED_FILES: &[&str] = &[
    ".cargo/config.toml",
    ".github/workflows/validation.yml",
    ".gitignore",
    "AGENTS.md",
    "ARCHITECTURE.md",
    "BOOTSTRAP.md",
    "Cargo.lock",
    "Cargo.toml",
    "LICENSE",
    "LICENSING.md",
    "README.md",
    "TESTING.md",
    "rust-toolchain.toml",
    "spec/README.md",
    "spec/semantic-model.md",
    "src/common.rs",
    "src/directed.rs",
    "src/lib.rs",
    "src/symmetric.rs",
    "tests/r0_conformance.rs",
    "xtask/Cargo.toml",
    "xtask/src/main.rs",
];

fn main() {
    let result = match env::args().nth(1).as_deref() {
        Some("validate") => validate(),
        _ => Err("usage: cargo xtask validate".to_owned()),
    };

    if let Err(error) = result {
        eprintln!("validation failed: {error}");
        std::process::exit(1);
    }
}

fn validate() -> Result<(), String> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask manifest must have a workspace root")
        .to_path_buf();

    validate_required_files(&root)?;
    validate_product_identity(&root)?;
    validate_normative_spec(&root)?;

    let initial_state = git_status(&root)?;
    if !initial_state.is_empty() {
        return Err(format!(
            "repository must be clean before validation:\n{initial_state}"
        ));
    }

    run(&root, "cargo", &["fmt", "--all", "--", "--check"])?;
    run(&root, "cargo", &["test", "--workspace", "--locked"])?;
    run(
        &root,
        "cargo",
        &[
            "clippy",
            "--workspace",
            "--all-targets",
            "--locked",
            "--",
            "-D",
            "warnings",
        ],
    )?;
    run_with_env(
        &root,
        "cargo",
        &["doc", "--workspace", "--no-deps", "--locked"],
        &[("RUSTDOCFLAGS", "-D warnings")],
    )?;
    run(&root, "git", &["diff", "--check"])?;
    run(&root, "git", &["diff", "--cached", "--check"])?;

    let final_state = git_status(&root)?;
    if final_state != initial_state {
        return Err(format!(
            "validation changed repository state:\nbefore:\n{initial_state}after:\n{final_state}"
        ));
    }

    Ok(())
}

fn validate_product_identity(root: &Path) -> Result<(), String> {
    let cargo_toml = read_text(root, "Cargo.toml")?;
    require_text(&cargo_toml, "Cargo.toml", "name = \"runen-graph\"")?;
    require_text(&cargo_toml, "Cargo.toml", "edition = \"2024\"")?;
    require_text(&cargo_toml, "Cargo.toml", "license.workspace = true")?;
    require_text(
        &cargo_toml,
        "Cargo.toml",
        "repository = \"https://github.com/dornglut/runen-graph\"",
    )?;
    require_text(&cargo_toml, "Cargo.toml", "publish = false")?;
    require_text(&cargo_toml, "Cargo.toml", "license = \"GPL-3.0-only\"")?;
    reject_text(&cargo_toml, "Cargo.toml", "rust-version")?;

    let license = read_text(root, "LICENSE")?;
    require_text(&license, "LICENSE", "GNU GENERAL PUBLIC LICENSE")?;
    require_text(&license, "LICENSE", "Version 3, 29 June 2007")?;
    reject_text(&license, "LICENSE", "Apache License")?;

    let licensing = read_text(root, "LICENSING.md")?;
    require_text(&licensing, "LICENSING.md", "GPL-3.0-only")?;
    require_text(&licensing, "LICENSING.md", "Apache-2.0")?;
    require_text(&licensing, "LICENSING.md", "historical")?;

    for relative_path in [
        "README.md",
        "AGENTS.md",
        "ARCHITECTURE.md",
        "TESTING.md",
        ".github/workflows/validation.yml",
        "src/lib.rs",
    ] {
        let text = read_text(root, relative_path)?;
        reject_text(&text, relative_path, "rust-framework-template")?;
        reject_text(&text, relative_path, "Rust Framework Template")?;
    }

    let workflow = read_text(root, ".github/workflows/validation.yml")?;
    require_text(
        &workflow,
        ".github/workflows/validation.yml",
        "name: RunenGraph Validation",
    )?;
    require_text(
        &workflow,
        ".github/workflows/validation.yml",
        "name: Validate RunenGraph",
    )?;

    Ok(())
}

fn validate_normative_spec(root: &Path) -> Result<(), String> {
    let index = read_text(root, "spec/README.md")?;
    require_text(
        &index,
        "spec/README.md",
        "sole normative authority for RunenGraph",
    )?;
    require_text(&index, "spec/README.md", "MUST")?;
    require_text(&index, "spec/README.md", "Open")?;
    require_text(&index, "spec/README.md", "Deferred")?;

    let model = read_text(root, "spec/semantic-model.md")?;
    for requirement in [
        "RG-ID-001",
        "RG-MEM-001",
        "RG-REL-001",
        "RG-DIR-001",
        "RG-SYM-001",
        "RG-SELF-001",
        "RG-MUT-003",
        "RG-OBS-001",
    ] {
        require_text(&model, "spec/semantic-model.md", requirement)?;
    }

    for relative_path in ["spec/README.md", "spec/semantic-model.md"] {
        let text = read_text(root, relative_path)?;
        reject_text(&text, relative_path, "http://")?;
        reject_text(&text, relative_path, "https://")?;
        reject_text(&text, relative_path, "../")?;
    }

    Ok(())
}

fn validate_required_files(root: &Path) -> Result<(), String> {
    for relative_path in REQUIRED_FILES {
        let path = root.join(relative_path);
        if !path.is_file() {
            return Err(format!("required file is missing: {relative_path}"));
        }
    }

    Ok(())
}

fn read_text(root: &Path, relative_path: &str) -> Result<String, String> {
    std::fs::read_to_string(root.join(relative_path))
        .map_err(|error| format!("failed to read {relative_path}: {error}"))
}

fn require_text(text: &str, relative_path: &str, expected: &str) -> Result<(), String> {
    if text.contains(expected) {
        Ok(())
    } else {
        Err(format!(
            "{relative_path} is missing required text: {expected}"
        ))
    }
}

fn reject_text(text: &str, relative_path: &str, forbidden: &str) -> Result<(), String> {
    if text.contains(forbidden) {
        Err(format!(
            "{relative_path} contains forbidden text: {forbidden}"
        ))
    } else {
        Ok(())
    }
}

fn git_status(root: &Path) -> Result<String, String> {
    output(
        root,
        "git",
        &["status", "--porcelain", "--untracked-files=all"],
    )
}

fn run(root: &Path, program: &str, args: &[&str]) -> Result<(), String> {
    run_with_env(root, program, args, &[])
}

fn run_with_env(
    root: &Path,
    program: &str,
    args: &[&str],
    environment: &[(&str, &str)],
) -> Result<(), String> {
    let mut command = Command::new(program);
    command
        .args(args)
        .current_dir(root)
        .envs(environment.iter().copied())
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    let status = command
        .status()
        .map_err(|error| format!("failed to execute {program}: {error}"))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("{program} {} exited with {status}", args.join(" ")))
    }
}

fn output(root: &Path, program: &str, args: &[&str]) -> Result<String, String> {
    let result = Command::new(program)
        .args(args)
        .current_dir(root)
        .output()
        .map_err(|error| format!("failed to execute {program}: {error}"))?;

    if !result.status.success() {
        return Err(format!(
            "{program} {} exited with {}:\n{}",
            args.join(" "),
            result.status,
            String::from_utf8_lossy(&result.stderr)
        ));
    }

    String::from_utf8(result.stdout)
        .map_err(|error| format!("{program} produced invalid UTF-8: {error}"))
}
