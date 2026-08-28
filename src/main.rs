use std::{env, fs, path::Path, process};

#[derive(Debug, PartialEq, Eq)]
struct Finding {
    line: usize,
    code: &'static str,
    message: String,
}

fn finding(line: usize, code: &'static str, message: impl Into<String>) -> Finding {
    Finding {
        line,
        code,
        message: message.into(),
    }
}

fn scan_requirements(input: &str) -> Vec<Finding> {
    let mut findings = Vec::new();

    for (index, raw) in input.lines().enumerate() {
        let line_no = index + 1;
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with("--") {
            continue;
        }
        if line.starts_with("-e ")
            || line.starts_with("git+")
            || line.starts_with("http://")
            || line.starts_with("https://")
        {
            findings.push(finding(
                line_no,
                "direct-source",
                "dependency is installed from a direct URL/VCS source",
            ));
            continue;
        }

        let has_constraint = ["==", ">=", "<=", "~=", "!=", ">", "<"]
            .iter()
            .any(|operator| line.contains(operator));
        if !has_constraint {
            findings.push(finding(
                line_no,
                "unconstrained-version",
                format!("'{line}' has no explicit version constraint"),
            ));
        }
    }

    findings
}

fn scan_cargo_toml(input: &str) -> Vec<Finding> {
    let mut findings = Vec::new();
    let mut dependency_section = false;

    for (index, raw) in input.lines().enumerate() {
        let line_no = index + 1;
        let line = raw.trim();
        if line.starts_with('[') && line.ends_with(']') {
            let section = line
                .trim_matches(|ch| ch == '[' || ch == ']')
                .to_ascii_lowercase();
            dependency_section = section.contains("dependencies");
            continue;
        }
        if !dependency_section || line.is_empty() || line.starts_with('#') {
            continue;
        }

        let Some((name, value)) = line.split_once('=') else {
            continue;
        };
        let name = name.trim();
        let value = value.trim();
        let normalized = value.replace(' ', "").to_ascii_lowercase();

        if value.trim_matches(|ch| ch == '"' || ch == '\'') == "*"
            || normalized.contains("version=\"*\"")
        {
            findings.push(finding(
                line_no,
                "wildcard-version",
                format!("'{name}' accepts any published version"),
            ));
        }
        if normalized.contains("git=") || normalized.contains("path=") {
            findings.push(finding(
                line_no,
                "direct-source",
                format!("'{name}' uses a git/path dependency source"),
            ));
        }
        if normalized.contains("http://") || normalized.contains("https://") {
            findings.push(finding(
                line_no,
                "direct-url",
                format!("'{name}' references a direct URL"),
            ));
        }
    }

    findings
}

fn scan_package_json(input: &str) -> Vec<Finding> {
    let mut findings = Vec::new();
    let mut dependency_block = false;

    for (index, raw) in input.lines().enumerate() {
        let line_no = index + 1;
        let line = raw.trim();
        if !dependency_block {
            let starts_block = [
                "\"dependencies\"",
                "\"devDependencies\"",
                "\"peerDependencies\"",
                "\"optionalDependencies\"",
            ]
            .iter()
            .any(|key| line.starts_with(key));
            if starts_block && line.contains('{') {
                dependency_block = true;
            }
            continue;
        }

        if line.starts_with('}') {
            dependency_block = false;
            continue;
        }

        let Some((name, value)) = line.split_once(':') else {
            continue;
        };
        let name = name.trim().trim_matches('"');
        let value = value.trim().trim_end_matches(',').trim().trim_matches('"');
        let lower = value.to_ascii_lowercase();

        if value == "*" || lower == "latest" {
            findings.push(finding(
                line_no,
                "floating-version",
                format!("'{name}' uses the floating version '{value}'"),
            ));
        }
        if lower.starts_with("git+")
            || lower.starts_with("http://")
            || lower.starts_with("https://")
            || lower.starts_with("github:")
            || lower.starts_with("file:")
        {
            findings.push(finding(
                line_no,
                "direct-source",
                format!("'{name}' is resolved from a direct/non-registry source"),
            ));
        }
    }

    findings
}

fn scan_manifest(path: &Path, input: &str) -> Result<Vec<Finding>, String> {
    let name = path
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or_else(|| "manifest path has no UTF-8 file name".to_owned())?;

    match name {
        "Cargo.toml" => Ok(scan_cargo_toml(input)),
        "package.json" => Ok(scan_package_json(input)),
        "requirements.txt" => Ok(scan_requirements(input)),
        _ => Err(format!(
            "unsupported manifest '{name}'; expected Cargo.toml, package.json, or requirements.txt"
        )),
    }
}

fn help() {
    println!(
        "DepGuard 0.1.0-dev\n\nUSAGE:\n  depguard scan <MANIFEST>\n\nSUPPORTED:\n  Cargo.toml\n  package.json\n  requirements.txt\n\nThe current preview performs local manifest heuristics only. It does not claim that a package is safe, malicious, real, or hallucinated."
    );
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() || matches!(args[0].as_str(), "--help" | "-h") {
        help();
        return;
    }
    if matches!(args[0].as_str(), "--version" | "-V") {
        println!("depguard 0.1.0-dev");
        return;
    }
    if args.len() != 2 || args[0] != "scan" {
        eprintln!("depguard: expected 'scan <MANIFEST>'");
        process::exit(2);
    }

    let path = Path::new(&args[1]);
    let input = match fs::read_to_string(path) {
        Ok(input) => input,
        Err(error) => {
            eprintln!("depguard: failed to read '{}': {error}", path.display());
            process::exit(2);
        }
    };

    let findings = match scan_manifest(path, &input) {
        Ok(findings) => findings,
        Err(error) => {
            eprintln!("depguard: {error}");
            process::exit(2);
        }
    };

    if findings.is_empty() {
        println!("OK: no current DepGuard review signal matched");
        return;
    }

    for item in &findings {
        println!("{}:{}: {}", item.line, item.code, item.message);
    }
    println!("FOUND: {} dependency review signal(s)", findings.len());
    process::exit(3);
}

#[cfg(test)]
mod tests {
    use super::{scan_cargo_toml, scan_package_json, scan_requirements};

    #[test]
    fn requirements_flags_unconstrained_dependency() {
        let findings = scan_requirements("requests\nflask==3.0.0\n");
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].code, "unconstrained-version");
    }

    #[test]
    fn cargo_flags_path_dependency() {
        let findings = scan_cargo_toml("[dependencies]\nlocal = { path = \"../local\" }\n");
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].code, "direct-source");
    }

    #[test]
    fn npm_flags_latest() {
        let findings =
            scan_package_json("{\n  \"dependencies\": {\n    \"left-pad\": \"latest\"\n  }\n}\n");
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].code, "floating-version");
    }

    #[test]
    fn pinned_examples_are_clean() {
        assert!(scan_requirements("requests==2.32.0\n").is_empty());
        assert!(scan_cargo_toml("[dependencies]\nserde = \"1\"\n").is_empty());
    }
}
