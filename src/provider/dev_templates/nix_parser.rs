use rnix::Root;
use rnix::ast::{Apply, AttrSet, AttrpathValue, Expr};
use rowan::ast::AstNode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShellAttrs {
    /// Items meant to be env vars, e.g. env = { RUST_SRC_PATH = "..."; }
    pub env: Vec<(String, String)>,
    /// Other non-standard attributes like venvDir, postShellHook
    pub extra_attrs: Vec<(String, String)>,
}

pub fn parse_flake_shell(source: &str) -> ShellAttrs {
    let parse = Root::parse(source);
    let root = parse.tree();

    let mut shell_attrs = ShellAttrs::default();

    // Traverse the AST to find `pkgs.mkShell` or `pkgs.mkShellNoCC` calls
    for node in root.syntax().descendants() {
        if let Some(apply) = Apply::cast(node.clone())
            && is_mk_shell_call(&apply)
            && let Some(Expr::AttrSet(attr_set)) = apply.argument()
        {
            extract_attributes(&attr_set, &mut shell_attrs);
            break; // Just need the first mkShell call in the flake (the default one)
        }
    }

    shell_attrs
}

fn is_mk_shell_call(apply: &Apply) -> bool {
    let Some(lambda) = apply.lambda() else {
        return false;
    };

    let text = lambda.syntax().text().to_string();
    text.contains("mkShell") || text.contains("mkShellNoCC")
}

fn extract_attributes(attr_set: &AttrSet, shell_attrs: &mut ShellAttrs) {
    for node in attr_set.syntax().children() {
        if let Some(attrpath_value) = AttrpathValue::cast(node) {
            let Some(attrpath) = attrpath_value.attrpath() else {
                continue;
            };

            let Some(value) = attrpath_value.value() else {
                continue;
            };

            let attr_name = attrpath.to_string();
            let value_text = value.syntax().text().to_string();

            match attr_name.as_str() {
                // Ignore standard inputs that inputsFrom handles
                "packages" | "buildInputs" | "nativeBuildInputs" | "shellHook" | "inputsFrom" => {
                    continue;
                }

                "env" => {
                    if let Expr::AttrSet(inner_set) = value {
                        for inner_node in inner_set.syntax().children() {
                            if let Some(inner_attr) = AttrpathValue::cast(inner_node)
                                && let (Some(k), Some(v)) =
                                    (inner_attr.attrpath(), inner_attr.value())
                            {
                                shell_attrs
                                    .env
                                    .push((k.to_string(), v.syntax().text().to_string()));
                            }
                        }
                    } else {
                        shell_attrs
                            .extra_attrs
                            .push(("env".to_string(), value_text));
                    }
                }

                _ => {
                    shell_attrs.extra_attrs.push((attr_name, value_text));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_mk_shell_with_env_and_extra_attrs() {
        let source = r#"
            { pkgs }:
            pkgs.mkShell {
              packages = with pkgs; [ rustc cargo ];
              env = {
                RUST_SRC_PATH = "foo";
                MY_VAR = "bar";
              };
              postShellHook = ''
                echo hello
              '';
            }
        "#;

        let attrs = parse_flake_shell(source);

        assert_eq!(
            attrs.env,
            vec![
                ("RUST_SRC_PATH".to_string(), "\"foo\"".to_string()),
                ("MY_VAR".to_string(), "\"bar\"".to_string()),
            ]
        );

        let keys: Vec<&str> = attrs.extra_attrs.iter().map(|(k, _)| k.as_str()).collect();
        assert!(
            keys.contains(&"postShellHook"),
            "expected postShellHook in extra_attrs, got: {keys:?}"
        );
        assert!(
            !keys.contains(&"packages"),
            "packages should be ignored, got: {keys:?}"
        );
    }

    #[test]
    fn parses_mk_shell_no_cc() {
        let source = r#"
            pkgs.mkShellNoCC {
              env = { GREETING = "hi"; };
            }
        "#;

        let attrs = parse_flake_shell(source);
        assert_eq!(
            attrs.env,
            vec![("GREETING".to_string(), "\"hi\"".to_string())]
        );
    }

    #[test]
    fn ignores_known_input_keys() {
        let source = r#"
            pkgs.mkShell {
              packages = [ a ];
              buildInputs = [ b ];
              nativeBuildInputs = [ c ];
              shellHook = "ignored";
              inputsFrom = [ d ];
            }
        "#;

        let attrs = parse_flake_shell(source);
        let keys: Vec<&str> = attrs.extra_attrs.iter().map(|(k, _)| k.as_str()).collect();
        assert!(
            keys.is_empty(),
            "all input-only keys should be dropped, got: {keys:?}"
        );
    }

    #[test]
    fn non_attrset_env_falls_back_to_extra_attrs() {
        let source = r#"
            pkgs.mkShell {
              env = someExpr;
            }
        "#;

        let attrs = parse_flake_shell(source);
        assert!(attrs.env.is_empty());
        assert_eq!(
            attrs.extra_attrs,
            vec![("env".to_string(), "someExpr".to_string())]
        );
    }

    #[test]
    fn source_without_mk_shell_returns_default_attrs() {
        let source = r#"
            { pkgs }:
            pkgs.buildEnv { name = "x"; paths = []; }
        "#;

        let attrs = parse_flake_shell(source);
        assert_eq!(attrs, ShellAttrs::default());
    }

    #[test]
    fn takes_only_the_first_mk_shell() {
        let source = r#"
            pkgs.mkShell { env = { A = "1"; }; };
            pkgs.mkShell { env = { B = "2"; }; };
        "#;

        let attrs = parse_flake_shell(source);
        assert_eq!(attrs.env, vec![("A".to_string(), "\"1\"".to_string())]);
    }

    #[test]
    fn non_mk_shell_apply_is_ignored() {
        let source = r#"
            pkgs.runCommand "x" { env = { FOO = "bar"; }; } ""
        "#;

        let attrs = parse_flake_shell(source);
        assert_eq!(attrs, ShellAttrs::default());
    }
}

#[cfg(test)]
mod serde_tests {
    use super::*;

    #[test]
    fn shell_attrs_roundtrips_through_json() {
        let original = ShellAttrs {
            env: vec![
                ("RUST_SRC_PATH".into(), "\"foo\"".into()),
                ("MY_VAR".into(), "\"bar\"".into()),
            ],
            extra_attrs: vec![("postShellHook".into(), "echo hi".into())],
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: ShellAttrs = serde_json::from_str(&json).unwrap();
        assert_eq!(restored, original);
    }

    #[test]
    fn shell_attrs_default_serializes_to_empty_arrays() {
        let attrs = ShellAttrs::default();
        let json = serde_json::to_string(&attrs).unwrap();
        assert_eq!(json, r#"{"env":[],"extra_attrs":[]}"#);
    }
}
