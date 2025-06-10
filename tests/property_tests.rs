use clojure_reader::edn::Edn;
use proptest::prelude::*;
use std::collections::BTreeMap;
use tilde_fish::fish_config;

// Simple property test strategies
prop_compose! {
    fn arb_alias_map()(
        aliases in prop::collection::btree_map(
            "[a-zA-Z][a-zA-Z0-9_-]*",  // Valid alias names
            "[a-zA-Z0-9 ._/-]+",       // Valid command strings
            0..10
        )
    ) -> BTreeMap<Edn<'static>, Edn<'static>> {
        aliases.into_iter()
            .map(|(k, v)| (Edn::Key(Box::leak(k.into_boxed_str())), Edn::Str(Box::leak(v.into_boxed_str()))))
            .collect()
    }
}

prop_compose! {
    fn arb_env_map()(
        env_vars in prop::collection::btree_map(
            "[A-Z][A-Z0-9_]*",         // Valid environment variable names
            "[a-zA-Z0-9 ._/-]+",       // Valid values
            0..10
        )
    ) -> BTreeMap<Edn<'static>, Edn<'static>> {
        env_vars.into_iter()
            .map(|(k, v)| (Edn::Key(Box::leak(k.into_boxed_str())), Edn::Str(Box::leak(v.into_boxed_str()))))
            .collect()
    }
}

prop_compose! {
    fn arb_paths()(
        paths in prop::collection::vec(
            r"/[a-zA-Z0-9/_.-]+",      // Valid Unix paths
            0..10
        )
    ) -> Vec<Edn<'static>> {
        paths.into_iter()
            .map(|p| Edn::Str(Box::leak(p.into_boxed_str())))
            .collect()
    }
}

prop_compose! {
    fn arb_fish_commands()(
        commands in prop::collection::vec(
            r"set -g [a-zA-Z_][a-zA-Z0-9_]* [a-zA-Z0-9]+",  // Valid fish set commands
            0..5
        )
    ) -> Vec<Edn<'static>> {
        commands.into_iter()
            .map(|c| Edn::Str(Box::leak(c.into_boxed_str())))
            .collect()
    }
}

prop_compose! {
    fn arb_config_map()(
        aliases in proptest::option::of(arb_alias_map()),
        env_vars in proptest::option::of(arb_env_map()),
        paths in proptest::option::of(arb_paths()),
        fish_commands in proptest::option::of(arb_fish_commands()),
        greeting in proptest::option::of("[a-zA-Z0-9 !.,]+"),
    ) -> BTreeMap<Edn<'static>, Edn<'static>> {
        let mut config = BTreeMap::new();

        if let Some(aliases) = aliases {
            config.insert(Edn::Key("aliases"), Edn::Map(aliases));
        }

        if let Some(env_vars) = env_vars {
            config.insert(Edn::Key("env"), Edn::Map(env_vars));
        }

        if let Some(paths) = paths {
            config.insert(Edn::Key("paths"), Edn::Vector(paths));
        }

        if let Some(fish_commands) = fish_commands {
            config.insert(Edn::Key("fish"), Edn::Vector(fish_commands));
        }

        if let Some(greeting) = greeting {
            config.insert(Edn::Key("fish-greeting"), Edn::Str(Box::leak(greeting.into_boxed_str())));
        }

        config
    }
}

proptest! {
    #[test]
    fn test_fish_config_never_panics(config_map in arb_config_map()) {
        let config = Edn::Map(config_map);
        let _result = fish_config(config);
        // Test passes if no panic occurs
    }

    #[test]
    fn test_fish_config_produces_valid_output(config_map in arb_config_map()) {
        let config = Edn::Map(config_map);
        let result = fish_config(config);

        // Output should not contain null bytes
        prop_assert!(!result.contains('\0'));

        // Should be valid UTF-8 (already guaranteed by String type)
        // Each line should contain valid characters
        for line in result.lines() {
            let valid_chars = " -_=+[]{}()'\".,/:!@#$%^&*";
            prop_assert!(line.chars().all(|c| c.is_ascii() || c.is_alphanumeric() || valid_chars.contains(c)));
        }
    }

    #[test]
    fn test_aliases_section_consistency(aliases in arb_alias_map()) {
        let mut config_map = BTreeMap::new();
        config_map.insert(Edn::Key("aliases"), Edn::Map(aliases.clone()));

        let config = Edn::Map(config_map);
        let result = fish_config(config);

        if !aliases.is_empty() {
            prop_assert!(result.contains("# Aliases"));

            // Each alias should appear in the output
            for (key, value) in aliases.iter() {
                if let (Edn::Key(alias_name), Edn::Str(alias_command)) = (key, value) {
                    let expected = format!("alias {} '{}'", alias_name, alias_command);
                    prop_assert!(result.contains(&expected));
                }
            }
        }
    }

    #[test]
    fn test_env_vars_section_consistency(env_vars in arb_env_map()) {
        let mut config_map = BTreeMap::new();
        config_map.insert(Edn::Key("env"), Edn::Map(env_vars.clone()));

        let config = Edn::Map(config_map);
        let result = fish_config(config);

        if !env_vars.is_empty() {
            prop_assert!(result.contains("# Environment Variables"));

            // Each environment variable should appear in the output
            for (key, value) in env_vars.iter() {
                if let (Edn::Key(var_name), Edn::Str(var_value)) = (key, value) {
                    let expected = format!("set -gx {} '{}'", var_name, var_value);
                    prop_assert!(result.contains(&expected));
                }
            }
        }
    }

    #[test]
    fn test_paths_section_consistency(paths in arb_paths()) {
        let mut config_map = BTreeMap::new();
        config_map.insert(Edn::Key("paths"), Edn::Vector(paths.clone()));

        let config = Edn::Map(config_map);
        let result = fish_config(config);

        if !paths.is_empty() {
            prop_assert!(result.contains("# PATH additions"));

            // Each path should appear in the output
            for path in paths.iter() {
                if let Edn::Str(path_str) = path {
                    let expected = format!("fish_add_path {}", path_str);
                    prop_assert!(result.contains(&expected));
                }
            }
        }
    }

    #[test]
    fn test_fish_commands_section_consistency(commands in arb_fish_commands()) {
        let mut config_map = BTreeMap::new();
        config_map.insert(Edn::Key("fish"), Edn::Vector(commands.clone()));

        let config = Edn::Map(config_map);
        let result = fish_config(config);

        if !commands.is_empty() {
            prop_assert!(result.contains("# Custom Fish Commands"));

            // Each command should appear in the output
            for command in commands.iter() {
                if let Edn::Str(cmd_str) = command {
                    prop_assert!(result.contains(cmd_str));
                }
            }
        }
    }

    #[test]
    fn test_greeting_consistency(greeting in proptest::option::of("[a-zA-Z0-9 !.,]+")) {
        let mut config_map = BTreeMap::new();
        if let Some(greeting_str) = greeting {
            config_map.insert(Edn::Key("fish-greeting"), Edn::Str(Box::leak(greeting_str.clone().into_boxed_str())));

            let config = Edn::Map(config_map);
            let result = fish_config(config);

            let expected = format!("set fish_greeting '{}'", greeting_str);
            prop_assert!(result.contains(&expected));
        }
    }

    #[test]
    fn test_output_sections_order_consistency(config_map in arb_config_map()) {
        let config = Edn::Map(config_map);
        let result = fish_config(config);

        // Find positions of section headers (if they exist)
        let greeting_pos = result.find("set fish_greeting");
        let aliases_pos = result.find("# Aliases");
        let env_pos = result.find("# Environment Variables");
        let paths_pos = result.find("# PATH additions");
        let functions_pos = result.find("# Functions");
        let fish_pos = result.find("# Custom Fish Commands");
        let prompt_pos = result.find("# Prompt Configuration");

        // Verify ordering when sections exist
        if let (Some(g), Some(a)) = (greeting_pos, aliases_pos) {
            prop_assert!(g < a, "Greeting should come before aliases");
        }
        if let (Some(a), Some(e)) = (aliases_pos, env_pos) {
            prop_assert!(a < e, "Aliases should come before environment variables");
        }
        if let (Some(e), Some(p)) = (env_pos, paths_pos) {
            prop_assert!(e < p, "Environment variables should come before paths");
        }
        if let (Some(p), Some(f)) = (paths_pos, functions_pos) {
            prop_assert!(p < f, "Paths should come before functions");
        }
        if let (Some(f), Some(c)) = (functions_pos, fish_pos) {
            prop_assert!(f < c, "Functions should come before fish commands");
        }
        if let (Some(c), Some(pr)) = (fish_pos, prompt_pos) {
            prop_assert!(c < pr, "Fish commands should come before prompt config");
        }
    }

    #[test]
    fn test_empty_collections_handled_gracefully(
        _dummy in Just(0) // Just a placeholder to make this a property test
    ) {
        let mut config_map = BTreeMap::new();
        config_map.insert(Edn::Key("aliases"), Edn::Map(BTreeMap::new()));
        config_map.insert(Edn::Key("env"), Edn::Map(BTreeMap::new()));
        config_map.insert(Edn::Key("paths"), Edn::Vector(Vec::new()));
        config_map.insert(Edn::Key("fish"), Edn::Vector(Vec::new()));

        let config = Edn::Map(config_map);
        let result = fish_config(config);

        // Should still contain section headers even for empty collections
        prop_assert!(result.contains("# Aliases"));
        prop_assert!(result.contains("# Environment Variables"));
        prop_assert!(result.contains("# PATH additions"));
        prop_assert!(result.contains("# Custom Fish Commands"));
    }
}

// Additional property tests for edge cases
proptest! {
    #[test]
    fn test_special_characters_in_strings(
        s in r"[a-zA-Z0-9 ._-]+"
    ) {
        let mut aliases = BTreeMap::new();
        let key = format!("test_alias_{}", s.len());
        aliases.insert(
            Edn::Key(Box::leak(key.into_boxed_str())),
            Edn::Str(Box::leak(s.into_boxed_str()))
        );

        let mut config_map = BTreeMap::new();
        config_map.insert(Edn::Key("aliases"), Edn::Map(aliases));

        let config = Edn::Map(config_map);
        let result = fish_config(config);

        // Should not panic and should produce valid output
        prop_assert!(!result.is_empty());
        prop_assert!(result.contains("alias"));
    }

    #[test]
    fn test_large_configurations(
        large_aliases in prop::collection::btree_map(
            "[a-zA-Z][a-zA-Z0-9_]*",
            "[a-zA-Z0-9 ._/-]+",
            20..50
        )
    ) {
        let aliases_map: BTreeMap<Edn, Edn> = large_aliases.into_iter()
            .map(|(k, v)| (Edn::Key(Box::leak(k.into_boxed_str())), Edn::Str(Box::leak(v.into_boxed_str()))))
            .collect();

        let mut config_map = BTreeMap::new();
        config_map.insert(Edn::Key("aliases"), Edn::Map(aliases_map));

        let config = Edn::Map(config_map);
        let result = fish_config(config);

        // Should handle large configurations without issues
        prop_assert!(!result.is_empty());
        prop_assert!(result.contains("# Aliases"));

        // Should contain at least some of the aliases
        let alias_count = result.matches("alias").count();
        prop_assert!(alias_count >= 20);
    }
}
