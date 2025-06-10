use std::io::Write;
use std::process::{Command, Stdio};

#[test]
fn test_plugin_config_command() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--config"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");

    // Verify the output is valid EDN
    assert!(stdout.contains(":name"));
    assert!(stdout.contains(":files"));
    assert!(stdout.contains(":generators"));
    assert!(stdout.contains(":preambles"));
    assert!(stdout.contains("fish"));

    // Parse it to ensure it's valid EDN
    let parsed = clojure_reader::edn::read_string(&stdout);
    assert!(parsed.is_ok(), "Plugin config should be valid EDN");
}

#[test]
fn test_gen_config_command_simple() {
    let mut child = Command::new("cargo")
        .args(&["run", "--", "--gen-config"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn command");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    stdin
        .write_all(b"{:aliases {:ll \"ls -la\"}}")
        .expect("Failed to write to stdin");
    stdin.flush().expect("Failed to flush stdin");
    drop(stdin); // Close stdin

    let output = child.wait_with_output().expect("Failed to read stdout");
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");
    assert!(stdout.contains("alias ll 'ls -la'"));
}

#[test]
fn test_gen_config_command_aliases() {
    let config = "{:aliases {:ll \"ls -la\" :la \"ls -A\"}}";

    let mut child = Command::new("cargo")
        .args(&["run", "--", "--gen-config"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn command");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    stdin
        .write_all(config.as_bytes())
        .expect("Failed to write to stdin");
    stdin.flush().expect("Failed to flush stdin");
    drop(stdin);

    let output = child.wait_with_output().expect("Failed to read stdout");
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");

    // Verify alias section
    assert!(stdout.contains("# Aliases"));
    assert!(stdout.contains("alias ll 'ls -la'"));
    assert!(stdout.contains("alias la 'ls -A'"));
}

#[test]
fn test_gen_config_command_env_vars() {
    let config = "{:env {:EDITOR \"nvim\" :BROWSER \"firefox\"}}";

    let mut child = Command::new("cargo")
        .args(&["run", "--", "--gen-config"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn command");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    stdin
        .write_all(config.as_bytes())
        .expect("Failed to write to stdin");
    stdin.flush().expect("Failed to flush stdin");
    drop(stdin);

    let output = child.wait_with_output().expect("Failed to read stdout");
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");

    assert!(stdout.contains("# Environment Variables"));
    assert!(stdout.contains("set -gx EDITOR 'nvim'"));
    assert!(stdout.contains("set -gx BROWSER 'firefox'"));
}

#[test]
fn test_gen_config_command_paths() {
    let config = "{:paths [\"/usr/local/bin\" \"~/.local/bin\"]}";

    let mut child = Command::new("cargo")
        .args(&["run", "--", "--gen-config"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn command");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    stdin
        .write_all(config.as_bytes())
        .expect("Failed to write to stdin");
    stdin.flush().expect("Failed to flush stdin");
    drop(stdin);

    let output = child.wait_with_output().expect("Failed to read stdout");
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");

    assert!(stdout.contains("# PATH additions"));
    assert!(stdout.contains("fish_add_path /usr/local/bin"));
    assert!(stdout.contains("fish_add_path ~/.local/bin"));
}

#[test]
fn test_gen_config_command_functions() {
    let config = "{:functions {:mkcd \"mkdir -p $argv[1]; and cd $argv[1]\"}}";

    let mut child = Command::new("cargo")
        .args(&["run", "--", "--gen-config"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn command");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    stdin
        .write_all(config.as_bytes())
        .expect("Failed to write to stdin");
    stdin.flush().expect("Failed to flush stdin");
    drop(stdin);

    let output = child.wait_with_output().expect("Failed to read stdout");
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");

    assert!(stdout.contains("# Functions"));
    assert!(stdout.contains("function mkcd"));
    assert!(stdout.contains("mkdir -p $argv[1]; and cd $argv[1]"));
    assert!(stdout.contains("end"));
}

#[test]
fn test_gen_config_command_fish_greeting() {
    let config = "{:fish-greeting \"Welcome to Fish!\"}";

    let mut child = Command::new("cargo")
        .args(&["run", "--", "--gen-config"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn command");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    stdin
        .write_all(config.as_bytes())
        .expect("Failed to write to stdin");
    stdin.flush().expect("Failed to flush stdin");
    drop(stdin);

    let output = child.wait_with_output().expect("Failed to read stdout");
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");
    assert!(stdout.contains("set fish_greeting 'Welcome to Fish!'"));
}

#[test]
fn test_gen_config_command_abbreviations() {
    let config = "{:abbrs {:gs \"git status\" :gc \"git commit\"}}";

    let mut child = Command::new("cargo")
        .args(&["run", "--", "--gen-config"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn command");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    stdin
        .write_all(config.as_bytes())
        .expect("Failed to write to stdin");
    stdin.flush().expect("Failed to flush stdin");
    drop(stdin);

    let output = child.wait_with_output().expect("Failed to read stdout");
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");

    assert!(stdout.contains("# Abbreviations"));
    assert!(stdout.contains("abbr -a -- gs 'git status'"));
    assert!(stdout.contains("abbr -a -- gc 'git commit'"));
}

#[test]
fn test_invalid_edn_input() {
    let mut child = Command::new("cargo")
        .args(&["run", "--", "--gen-config"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn command");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    stdin
        .write_all(b"{invalid edn")
        .expect("Failed to write to stdin");
    stdin.flush().expect("Failed to flush stdin");
    drop(stdin);

    let output = child.wait_with_output().expect("Failed to read stdout");
    assert!(!output.status.success());

    let stderr = String::from_utf8(output.stderr).expect("Invalid UTF-8");
    assert!(stderr.contains("Failed to parse EDN"));
}

#[test]
fn test_empty_input() {
    let mut child = Command::new("cargo")
        .args(&["run", "--", "--gen-config"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn command");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    stdin.write_all(b"{}").expect("Failed to write to stdin");
    stdin.flush().expect("Failed to flush stdin");
    drop(stdin);

    let output = child.wait_with_output().expect("Failed to read stdout");
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");
    // Empty config should produce minimal output
    assert!(stdout.trim().is_empty() || stdout == "\n");
}

#[test]
fn test_usage_message() {
    let output = Command::new("cargo")
        .args(&["run", "--"])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());

    let stderr = String::from_utf8(output.stderr).expect("Invalid UTF-8");
    assert!(stderr.contains("Usage:"));
    assert!(stderr.contains("--config"));
    assert!(stderr.contains("--gen-config"));
}

#[test]
fn test_prompt_configuration() {
    let config = "{:prompt {:style \"robbyrussell\" :show-git true}}";

    let mut child = Command::new("cargo")
        .args(&["run", "--", "--gen-config"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn command");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    stdin
        .write_all(config.as_bytes())
        .expect("Failed to write to stdin");
    stdin.flush().expect("Failed to flush stdin");
    drop(stdin);

    let output = child.wait_with_output().expect("Failed to read stdout");
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");

    assert!(stdout.contains("# Prompt Configuration"));
    assert!(stdout.contains("set -g theme robbyrussell"));
    assert!(stdout.contains("set -g fish_prompt_show_git true"));
}

#[test]
fn test_boolean_prompt_values() {
    let config_false = "{:prompt {:show-git false}}";

    let mut child = Command::new("cargo")
        .args(&["run", "--", "--gen-config"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn command");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    stdin
        .write_all(config_false.as_bytes())
        .expect("Failed to write to stdin");
    stdin.flush().expect("Failed to flush stdin");
    drop(stdin);

    let output = child.wait_with_output().expect("Failed to read stdout");
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");
    assert!(stdout.contains("set -g fish_prompt_show_git false"));
}

#[test]
fn test_comprehensive_config() {
    let config = r#"{
        :fish-greeting "Hello Fish!"
        :aliases {:ll "ls -la" :la "ls -A"}
        :env {:EDITOR "nvim"}
        :paths ["/usr/local/bin"]
        :functions {:mkcd "mkdir -p $argv[1]; and cd $argv[1]"}
        :fish ["set -g fish_prompt_pwd_dir_length 3"]
        :prompt {:style "robbyrussell" :show-git true}
    }"#;

    let mut child = Command::new("cargo")
        .args(&["run", "--", "--gen-config"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn command");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    stdin
        .write_all(config.as_bytes())
        .expect("Failed to write to stdin");
    stdin.flush().expect("Failed to flush stdin");
    drop(stdin);

    let output = child.wait_with_output().expect("Failed to read stdout");
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");

    // Verify all sections are present
    assert!(stdout.contains("set fish_greeting 'Hello Fish!'"));
    assert!(stdout.contains("# Aliases"));
    assert!(stdout.contains("alias ll 'ls -la'"));
    assert!(stdout.contains("# Environment Variables"));
    assert!(stdout.contains("set -gx EDITOR 'nvim'"));
    assert!(stdout.contains("# PATH additions"));
    assert!(stdout.contains("fish_add_path /usr/local/bin"));
    assert!(stdout.contains("# Functions"));
    assert!(stdout.contains("function mkcd"));
    assert!(stdout.contains("# Custom Fish Commands"));
    assert!(stdout.contains("set -g fish_prompt_pwd_dir_length 3"));
    assert!(stdout.contains("# Prompt Configuration"));
    assert!(stdout.contains("set -g theme robbyrussell"));
}
