use std::collections::HashMap;
use std::fmt::Write;
use tracing_attributes::instrument;

use super::nix_parser::ShellAttrs;

/// Generates a dev-templates flake.nix that combines multiple language shells.
///
/// # Parameters
/// - `languages`: Deduplicated language names in requested order
/// - `parsed_attrs`: Shell attributes for each language at the same index
///
/// # Important
/// The two slices must have the same length, and `parsed_attrs[i]` must
/// correspond to `languages[i]`. This is enforced by the caller in `mod.rs`.
#[instrument(skip_all, fields(provider = "dev_templates", language_count = %languages.len()))]
pub fn generate_dev_templates_flake(languages: &[String], parsed_attrs: &[ShellAttrs]) -> String {
    let input_names: Vec<String> = languages
        .iter()
        .map(|lang| format!("dev-templates_{}", lang))
        .collect();

    let inputs_entries: Vec<String> = languages
        .iter()
        .map(|lang| {
            format!(
                "    dev-templates_{}.url = \"github:the-nix-way/dev-templates?dir={}\";\n    dev-templates_{}.inputs.nixpkgs.follows = \"nixpkgs\";",
                lang, lang, lang
            )
        })
        .collect();

    let outputs_inputs = input_names.join(", ");

    let shells_entries: Vec<String> = languages
        .iter()
        .map(|lang| {
            format!(
                "            {} = dev-templates_{}.devShells.${{system}}.default;",
                lang, lang
            )
        })
        .collect();

    // Group by attribute names to avoid duplicate keys in Nix
    let mut extra_attrs_map: HashMap<String, Vec<String>> = HashMap::new();
    let mut env_map: HashMap<String, String> = HashMap::new();

    // Make precedence explicit: process attributes by requested language order.
    for (i, lang) in languages.iter().enumerate() {
        let Some(attrs) = parsed_attrs.get(i) else {
            continue;
        };

        for (k, _) in &attrs.extra_attrs {
            let expr = format!("shells.\"{}\".{}", lang, k);
            extra_attrs_map.entry(k.clone()).or_default().push(expr);
        }

        for (k, _) in &attrs.env {
            let expr = format!("shells.\"{}\".{}", lang, k);
            // Keep last language in request order on conflict.
            env_map.insert(k.clone(), expr);
        }
    }

    let mut extra_attrs_str = String::new();

    let mut extra_attr_keys: Vec<&String> = extra_attrs_map.keys().collect();
    extra_attr_keys.sort_unstable();
    for key in extra_attr_keys {
        let exprs = &extra_attrs_map[key];
        if key == "postShellHook" || key == "shellHook" || key == "preHook" {
            writeln!(
                extra_attrs_str,
                "            {} = {};",
                key,
                exprs.join(" + \"\\n\" + ")
            )
            .expect("writing to String cannot fail");
        } else if let Some(expr) = exprs.first() {
            // For other attributes, keep the first language in request order.
            writeln!(extra_attrs_str, "            {} = {};", key, expr)
                .expect("writing to String cannot fail");
        }
    }

    if !env_map.is_empty() {
        extra_attrs_str.push_str("            env = {\n");
        let mut env_keys: Vec<&String> = env_map.keys().collect();
        env_keys.sort_unstable();
        for key in env_keys {
            writeln!(extra_attrs_str, "              {} = {};", key, env_map[key])
                .expect("writing to String cannot fail");
        }
        extra_attrs_str.push_str("            };\n");
    }

    format!(
        r#"{{
  inputs = {{
    nixpkgs.url = "https://flakehub.com/f/NixOS/nixpkgs/0.1";
{}
  }};

  outputs =
    {{ nixpkgs, {}, ... }}:
    let
      allSystems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];

      forAllSystems =
        f: nixpkgs.lib.genAttrs allSystems (system: f {{ pkgs = import nixpkgs {{ inherit system; }}; }});
    in
    {{
      devShells = forAllSystems (
        {{ pkgs }}:
        let
          system = pkgs.stdenv.hostPlatform.system;
          shells = {{
{}
          }};
          inputsFrom = builtins.attrValues shells;
        in
        {{
          default = pkgs.mkShellNoCC {{
            inherit inputsFrom;
{}          }};
        }}
      );
    }};
}}
"#,
        inputs_entries.join("\n"),
        outputs_inputs,
        shells_entries.join("\n"),
        extra_attrs_str
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn emits_input_per_language() {
        let flake = generate_dev_templates_flake(&["rust".into()], &[ShellAttrs::default()]);
        assert!(
            flake.contains(
                r#"dev-templates_rust.url = "github:the-nix-way/dev-templates?dir=rust";"#
            )
        );
        assert!(flake.contains(r#"dev-templates_rust.inputs.nixpkgs.follows = "nixpkgs";"#));
        assert!(flake.contains("rust, ... }:"));
    }

    #[test]
    fn emits_shell_binding_per_language() {
        let flake = generate_dev_templates_flake(
            &["rust".into(), "go".into()],
            &[ShellAttrs::default(), ShellAttrs::default()],
        );
        assert!(flake.contains(r#"rust = dev-templates_rust.devShells.${system}.default;"#));
        assert!(flake.contains(r#"go = dev-templates_go.devShells.${system}.default;"#));
    }

    #[test]
    fn uses_mk_shell_no_cc() {
        let flake = generate_dev_templates_flake(&["rust".into()], &[ShellAttrs::default()]);
        assert!(flake.contains("default = pkgs.mkShellNoCC"));
    }

    #[test]
    fn concatenates_shell_hooks_across_languages() {
        let attrs_a = ShellAttrs {
            extra_attrs: vec![("postShellHook".to_string(), "echo a".to_string())],
            ..ShellAttrs::default()
        };
        let attrs_b = ShellAttrs {
            extra_attrs: vec![("postShellHook".to_string(), "echo b".to_string())],
            ..ShellAttrs::default()
        };
        let flake =
            generate_dev_templates_flake(&["rust".into(), "go".into()], &[attrs_a, attrs_b]);
        assert!(
            flake.contains(
                r#"postShellHook = shells."rust".postShellHook + "\n" + shells."go".postShellHook;"#
            ),
            "missing hook concat: {flake}"
        );
    }

    #[test]
    fn env_keys_are_merged_with_last_writer_wins() {
        let attrs_a = ShellAttrs {
            env: vec![("FOO".to_string(), "a".to_string())],
            ..ShellAttrs::default()
        };
        let attrs_b = ShellAttrs {
            env: vec![("FOO".to_string(), "b".to_string())],
            ..ShellAttrs::default()
        };
        let flake =
            generate_dev_templates_flake(&["rust".into(), "go".into()], &[attrs_a, attrs_b]);
        assert!(flake.contains(r#"FOO = shells."go".FOO;"#), "{flake}");
        assert!(!flake.contains(r#"FOO = shells."rust".FOO;"#));
    }

    #[test]
    fn extra_attrs_take_first_language_for_non_hook_keys() {
        let attrs_a = ShellAttrs {
            extra_attrs: vec![("venvDir".to_string(), "a".to_string())],
            ..ShellAttrs::default()
        };
        let attrs_b = ShellAttrs {
            extra_attrs: vec![("venvDir".to_string(), "b".to_string())],
            ..ShellAttrs::default()
        };
        let flake =
            generate_dev_templates_flake(&["rust".into(), "go".into()], &[attrs_a, attrs_b]);
        assert!(
            flake.contains(r#"venvDir = shells."rust".venvDir;"#),
            "{flake}"
        );
    }

    #[test]
    fn empty_languages_produces_usable_flake() {
        let flake = generate_dev_templates_flake(&[], &[]);
        assert!(flake.contains("devShells = forAllSystems"));
        assert!(flake.contains("default = pkgs.mkShellNoCC"));
    }
}
