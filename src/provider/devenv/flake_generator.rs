use tracing_attributes::instrument;

#[instrument(skip_all, fields(provider = "devenv", languages = ?languages))]
pub fn generate_devenv_flake(languages: &[String]) -> String {
    let languages_enable_str = languages
        .iter()
        .map(|lang| format!("languages.{}.enable = true;", lang))
        .collect::<Vec<_>>()
        .join("\n                ");

    format!(
        r#"{{
  inputs = {{
    nixpkgs.url = "github:cachix/devenv-nixpkgs/rolling";
    devenv.url = "github:cachix/devenv";
  }};

  nixConfig = {{
    extra-trusted-public-keys = "devenv.cachix.org-1:w1cLUi8dv3hnoSPGAuibQv+f9TZLr6cv/Hm9XgU50cw= cachix.cachix.org-1:eWNHQldwUO7G2VkjpnjDbWwy4KQ/HNxht7H4SSoMckM=";
    extra-substituters = "https://devenv.cachix.org https://cachix.cachix.org";
  }};

  outputs =
    {{ nixpkgs, devenv, ... }}@inputs:
    let
      inherit (nixpkgs) lib;

      allSystems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];

      forAllSystems =
        f: lib.genAttrs allSystems (system: f {{ pkgs = import nixpkgs {{ inherit system; }}; }});
    in
    {{
      devShells = forAllSystems (
        {{ pkgs }}:
        {{
          default = devenv.lib.mkShell {{
            inherit inputs pkgs;
            modules = [
              {{
                {}
              }}
            ];
          }};
        }}
      );
    }};
}}
"#,
        languages_enable_str
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enables_each_requested_language() {
        let flake = generate_devenv_flake(&["rust".into(), "go".into()]);
        assert!(flake.contains("languages.rust.enable = true;"));
        assert!(flake.contains("languages.go.enable = true;"));
    }

    #[test]
    fn empty_languages_produces_valid_flake() {
        let flake = generate_devenv_flake(&[]);
        assert!(flake.contains("devenv.lib.mkShell"));
        assert!(flake.contains("modules = ["));
        assert!(!flake.contains("languages..enable = true;"));
    }

    #[test]
    fn includes_devenv_inputs() {
        let flake = generate_devenv_flake(&["rust".into()]);
        assert!(flake.contains(r#"nixpkgs.url = "github:cachix/devenv-nixpkgs/rolling";"#));
        assert!(flake.contains(r#"devenv.url = "github:cachix/devenv";"#));
    }

    #[test]
    fn declares_default_devshell() {
        let flake = generate_devenv_flake(&[]);
        assert!(flake.contains("devShells = forAllSystems"));
        assert!(flake.contains("default = devenv.lib.mkShell"));
    }

    #[test]
    fn preserves_input_order_in_modules() {
        let flake = generate_devenv_flake(&["a".into(), "b".into(), "c".into()]);
        let a = flake.find("languages.a.enable").unwrap();
        let b = flake.find("languages.b.enable").unwrap();
        let c = flake.find("languages.c.enable").unwrap();
        assert!(a < b && b < c, "languages should appear in input order");
    }
}
