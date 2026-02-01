//! Fast Rust dev loop: doctor/init/watch + cargo wrappers
//!
//! # Examples
//!
//! ```no_run
//! use cargo_fastdev::{cmd_doctor, cmd_init, cmd_watch};
//!
//! // Check toolchain
//! cmd_doctor(None).unwrap();
//!
//! // Generate config
//! cmd_init(true, false, false, false).unwrap();
//!
//! // Watch for changes
//! cmd_watch("check".to_string(), vec![]).unwrap();
//! ```

use anyhow::{Context, Result};
use notify::{EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

/// Doctor output format
#[derive(Debug, Clone, Serialize)]
pub struct DoctorOutput {
    pub toolchain: ToolchainStatus,
    pub suggestions: Vec<String>,
}

/// Toolchain detection status
#[derive(Debug, Clone, Serialize)]
pub struct ToolchainStatus {
    pub sccache: bool,
    pub mold: bool,
    pub clang: bool,
}

/// Initialize new configuration
pub fn cmd_init(print: bool, write: bool, use_sccache: bool, use_mold: bool) -> Result<()> {
    let config = generate_config(use_sccache, use_mold)?;

    if print || !write {
        println!("{}", config);
        return Ok(());
    }

    if write {
        let cargo_dir = PathBuf::from(".cargo");
        fs::create_dir_all(&cargo_dir)?;
        let config_path = cargo_dir.join("config.toml");

        if config_path.exists() {
            return Err(anyhow::anyhow!(
                "config.toml already exists. Use --print to see what would be written."
            ));
        }

        fs::write(&config_path, config)?;
        println!("Wrote {}", config_path.display());
    }

    Ok(())
}

/// Run doctor checks
pub fn cmd_doctor(format: Option<String>) -> Result<()> {
    let toolchain = detect_toolchain();
    let mut suggestions = Vec::new();

    if !toolchain.sccache {
        suggestions.push(
            "Install sccache for faster incremental builds: https://github.com/mozilla/sccache"
                .to_string(),
        );
    }
    if !toolchain.mold {
        suggestions.push(
            "Consider using mold linker for faster linking: https://github.com/rui314/mold"
                .to_string(),
        );
    }
    if !toolchain.clang {
        suggestions.push("Clang can provide better diagnostics than GCC".to_string());
    }

    let output = DoctorOutput {
        toolchain,
        suggestions,
    };

    if format.as_deref() == Some("json") {
        println!("{}", serde_json::to_string_pretty(&output)?);
    } else {
        println!("Toolchain Status:");
        println!(
            "  sccache: {}",
            if output.toolchain.sccache {
                "✓"
            } else {
                "✗"
            }
        );
        println!("  mold: {}", if output.toolchain.mold { "✓" } else { "✗" });
        println!(
            "  clang: {}",
            if output.toolchain.clang { "✓" } else { "✗" }
        );

        if !output.suggestions.is_empty() {
            println!("\nSuggestions:");
            for s in &output.suggestions {
                println!("  - {}", s);
            }
        }
    }

    Ok(())
}

/// Watch for file changes and re-run cargo command
pub fn cmd_watch(command: String, args: Vec<String>) -> Result<()> {
    let (tx, rx) = mpsc::channel();

    let mut watcher: RecommendedWatcher =
        Watcher::new(tx, notify::Config::default()).context("failed to create file watcher")?;

    watcher.watch(Path::new("."), RecursiveMode::Recursive)?;

    println!("Watching for changes...");
    run_cargo(&command, &args)?;

    loop {
        match rx.recv() {
            Ok(event) => {
                if let Ok(event) = event {
                    if matches!(
                        event.kind,
                        EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_)
                    ) {
                        // Debounce rapid changes
                        thread::sleep(Duration::from_millis(100));
                        run_cargo(&command, &args)?;
                    }
                }
            }
            Err(e) => {
                eprintln!("Watch error: {:?}", e);
                break;
            }
        }
    }

    Ok(())
}

/// Run cargo check with default options
pub fn cmd_check(args: Vec<String>) -> Result<()> {
    run_cargo("check", &args)
}

/// Run cargo test with default options
pub fn cmd_test(args: Vec<String>) -> Result<()> {
    run_cargo("test", &args)
}

/// Run cargo run with default options
pub fn cmd_run(args: Vec<String>) -> Result<()> {
    run_cargo("run", &args)
}

fn detect_toolchain() -> ToolchainStatus {
    ToolchainStatus {
        sccache: Command::new("sccache").arg("--version").output().is_ok(),
        mold: Command::new("mold").arg("--version").output().is_ok(),
        clang: Command::new("clang").arg("--version").output().is_ok(),
    }
}

fn generate_config(use_sccache: bool, use_mold: bool) -> Result<String> {
    let mut lines = vec![
        "# Generated by cargo-fastdev".to_string(),
        "".to_string(),
        "[build]".to_string(),
    ];

    if use_sccache {
        lines.push("# Use sccache for faster incremental builds".to_string());
        lines.push("rustflags = [\"-C\", \"link-arg=-fuse-ld=lld\"]".to_string());
    }

    if use_mold {
        lines.push("# Use mold linker for faster linking".to_string());
        lines.push("rustflags = [\"-C\", \"link-arg=-fuse-ld=mold\"]".to_string());
    }

    lines.push("".to_string());
    lines.push("# Optimize dependencies".to_string());
    lines.push("[profile.dev.package.\"*\"]".to_string());
    lines.push("opt-level = 1".to_string());
    lines.push("".to_string());
    lines.push("# Keep debug info for workspace".to_string());
    lines.push("[profile.dev]".to_string());
    lines.push("debug = true".to_string());

    Ok(lines.join("\n"))
}

fn run_cargo(command: &str, args: &[String]) -> Result<()> {
    let mut cmd = Command::new("cargo");
    cmd.arg(command);
    cmd.args(args);

    let status = cmd
        .status()
        .context(format!("failed to run cargo {}", command))?;

    if !status.success() {
        anyhow::bail!(
            "cargo {} failed with exit code: {:?}",
            command,
            status.code()
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_config_empty() {
        let config = generate_config(false, false).unwrap();
        assert!(config.contains("opt-level = 1"));
        assert!(config.contains("debug = true"));
    }

    #[test]
    fn test_generate_config_with_sccache() {
        let config = generate_config(true, false).unwrap();
        assert!(config.contains("sccache"));
        assert!(config.contains("link-arg=-fuse-ld=lld"));
    }

    #[test]
    fn test_generate_config_with_mold() {
        let config = generate_config(false, true).unwrap();
        assert!(config.contains("mold"));
        assert!(config.contains("link-arg=-fuse-ld=mold"));
    }

    #[test]
    fn test_generate_config_with_both() {
        let config = generate_config(true, true).unwrap();
        assert!(config.contains("opt-level = 1"));
    }

    #[test]
    fn test_doctor_output_json() {
        let output = DoctorOutput {
            toolchain: ToolchainStatus {
                sccache: true,
                mold: false,
                clang: true,
            },
            suggestions: vec!["Install mold".to_string()],
        };

        let json = serde_json::to_string(&output).unwrap();
        assert!(json.contains("\"sccache\":true"));
        assert!(json.contains("\"mold\":false"));
    }
}
