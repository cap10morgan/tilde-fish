use clojure_reader::edn::Edn;
use criterion::{Criterion, black_box, criterion_group, criterion_main};
use std::collections::BTreeMap;
use tilde_fish::{fish_config, plugin_config};

fn create_small_config() -> Edn<'static> {
    let mut aliases = BTreeMap::new();
    aliases.insert(Edn::Key("ll"), Edn::Str("ls -la"));
    aliases.insert(Edn::Key("la"), Edn::Str("ls -A"));

    let mut config_map = BTreeMap::new();
    config_map.insert(Edn::Key("aliases"), Edn::Map(aliases));
    config_map.insert(Edn::Key("fish-greeting"), Edn::Str("Hello Fish!"));

    Edn::Map(config_map)
}

fn create_medium_config() -> Edn<'static> {
    let mut aliases = BTreeMap::new();
    for i in 0..20 {
        aliases.insert(
            Edn::Key(Box::leak(format!("alias{}", i).into_boxed_str())),
            Edn::Str(Box::leak(format!("command{} -flag", i).into_boxed_str())),
        );
    }

    let mut env_vars = BTreeMap::new();
    for i in 0..10 {
        env_vars.insert(
            Edn::Key(Box::leak(format!("VAR{}", i).into_boxed_str())),
            Edn::Str(Box::leak(format!("value{}", i).into_boxed_str())),
        );
    }

    let mut paths = Vec::new();
    for i in 0..5 {
        paths.push(Edn::Str(Box::leak(
            format!("/path/to/bin{}", i).into_boxed_str(),
        )));
    }

    let mut functions = BTreeMap::new();
    functions.insert(
        Edn::Key("mkcd"),
        Edn::Str("mkdir -p $argv[1]; and cd $argv[1]"),
    );
    functions.insert(
        Edn::Key("extract"),
        Edn::Str("switch $argv[1]\\ncase '*.tar.gz'\\n    tar -xzf $argv[1]\\ncase '*.zip'\\n    unzip $argv[1]\\nend")
    );

    let mut config_map = BTreeMap::new();
    config_map.insert(Edn::Key("aliases"), Edn::Map(aliases));
    config_map.insert(Edn::Key("env"), Edn::Map(env_vars));
    config_map.insert(Edn::Key("paths"), Edn::Vector(paths));
    config_map.insert(Edn::Key("functions"), Edn::Map(functions));
    config_map.insert(Edn::Key("fish-greeting"), Edn::Str("Welcome to Fish!"));

    Edn::Map(config_map)
}

fn create_large_config() -> Edn<'static> {
    let mut aliases = BTreeMap::new();
    for i in 0..100 {
        aliases.insert(
            Edn::Key(Box::leak(format!("alias{}", i).into_boxed_str())),
            Edn::Str(Box::leak(
                format!("command{} --verbose --flag{}", i, i).into_boxed_str(),
            )),
        );
    }

    let mut abbrs = BTreeMap::new();
    for i in 0..50 {
        abbrs.insert(
            Edn::Key(Box::leak(format!("a{}", i).into_boxed_str())),
            Edn::Str(Box::leak(format!("git command{}", i).into_boxed_str())),
        );
    }

    let mut env_vars = BTreeMap::new();
    for i in 0..30 {
        env_vars.insert(
            Edn::Key(Box::leak(format!("ENV_VAR_{}", i).into_boxed_str())),
            Edn::Str(Box::leak(
                format!(
                    "very_long_environment_variable_value_{}_with_special_chars",
                    i
                )
                .into_boxed_str(),
            )),
        );
    }

    let mut paths = Vec::new();
    for i in 0..20 {
        paths.push(Edn::Str(Box::leak(
            format!("/very/long/path/to/some/binary/directory/number/{}", i).into_boxed_str(),
        )));
    }

    let mut functions = BTreeMap::new();
    for i in 0..15 {
        functions.insert(
            Edn::Key(Box::leak(format!("func{}", i).into_boxed_str())),
            Edn::Str(Box::leak(format!("function body for function {}\\nwith multiple lines\\nand complex logic\\nif test $argv[1]\\n    echo 'Processing {}'\\nelse\\n    echo 'Default action'\\nend", i, i).into_boxed_str()))
        );
    }

    let mut fish_commands = Vec::new();
    for i in 0..10 {
        fish_commands.push(Edn::Str(Box::leak(
            format!("set -g fish_config_option_{} value{}", i, i).into_boxed_str(),
        )));
    }

    let mut snippets = BTreeMap::new();
    for i in 0..5 {
        snippets.insert(
            Edn::Key(Box::leak(format!("snippet/custom{}", i).into_boxed_str())),
            Edn::Str(Box::leak(format!("# Custom snippet {}\\necho 'Snippet {} executed'\\nset -g snippet_{}_loaded true", i, i, i).into_boxed_str()))
        );
    }

    let mut preambles = BTreeMap::new();
    preambles.insert(
        Edn::Key("tilde/all"),
        Edn::Str("# This is a very long preamble comment\\n# Generated automatically by tilde-fish\\n# Please do not modify this file manually\\n# Last updated: $(date)\\n")
    );

    let mut prompt_config = BTreeMap::new();
    prompt_config.insert(Edn::Key("style"), Edn::Str("robbyrussell"));
    prompt_config.insert(Edn::Key("show-git"), Edn::Bool(true));

    let mut config_map = BTreeMap::new();
    config_map.insert(Edn::Key("preambles"), Edn::Map(preambles));
    config_map.insert(
        Edn::Key("fish-greeting"),
        Edn::Str("Welcome to the most comprehensive Fish shell configuration!"),
    );
    config_map.insert(Edn::Key("abbrs"), Edn::Map(abbrs));
    config_map.insert(Edn::Key("aliases"), Edn::Map(aliases));
    config_map.insert(Edn::Key("env"), Edn::Map(env_vars));
    config_map.insert(Edn::Key("paths"), Edn::Vector(paths));
    config_map.insert(Edn::Key("functions"), Edn::Map(functions));
    config_map.insert(Edn::Key("fish"), Edn::Vector(fish_commands));
    config_map.insert(Edn::Key("prompt"), Edn::Map(prompt_config));

    // Add snippet entries to the main config map
    for (key, value) in snippets {
        config_map.insert(key, value);
    }

    Edn::Map(config_map)
}

fn create_empty_config() -> Edn<'static> {
    Edn::Map(BTreeMap::new())
}

fn benchmark_plugin_config(c: &mut Criterion) {
    c.bench_function("plugin_config", |b| b.iter(|| black_box(plugin_config())));
}

fn benchmark_fish_config_empty(c: &mut Criterion) {
    let config = create_empty_config();
    c.bench_function("fish_config_empty", |b| {
        b.iter(|| black_box(fish_config(config.clone())))
    });
}

fn benchmark_fish_config_small(c: &mut Criterion) {
    let config = create_small_config();
    c.bench_function("fish_config_small", |b| {
        b.iter(|| black_box(fish_config(config.clone())))
    });
}

fn benchmark_fish_config_medium(c: &mut Criterion) {
    let config = create_medium_config();
    c.bench_function("fish_config_medium", |b| {
        b.iter(|| black_box(fish_config(config.clone())))
    });
}

fn benchmark_fish_config_large(c: &mut Criterion) {
    let config = create_large_config();
    c.bench_function("fish_config_large", |b| {
        b.iter(|| black_box(fish_config(config.clone())))
    });
}

fn benchmark_config_parsing_and_generation(c: &mut Criterion) {
    let config_edn = r#"{:aliases {:ll "ls -la" :la "ls -A" :grep "grep --color=auto"}
                        :env {:EDITOR "nvim" :BROWSER "firefox"}
                        :paths ["/usr/local/bin" "~/.local/bin"]
                        :functions {:mkcd "mkdir -p $argv[1]; and cd $argv[1]"}
                        :fish ["set -g fish_prompt_pwd_dir_length 3"]
                        :prompt {:style "robbyrussell" :show-git true}}"#;

    c.bench_function("parse_and_generate", |b| {
        b.iter(|| {
            let parsed = clojure_reader::edn::read_string(config_edn).unwrap();
            black_box(fish_config(parsed))
        })
    });
}

fn benchmark_multiline_functions(c: &mut Criterion) {
    let mut functions = BTreeMap::new();
    functions.insert(
        Edn::Key("complex_function"),
        Edn::Str("if test (count $argv) -eq 0\\n    echo 'No arguments provided'\\n    return 1\\nend\\n\\nfor arg in $argv\\n    switch $arg\\n        case '*.tar.gz'\\n            tar -xzf $arg\\n        case '*.zip'\\n            unzip $arg\\n        case '*.tar.bz2'\\n            tar -xjf $arg\\n        case '*'\\n            echo 'Unknown file type: $arg'\\n    end\\nend\\n\\necho 'Processing complete'")
    );

    let mut config_map = BTreeMap::new();
    config_map.insert(Edn::Key("functions"), Edn::Map(functions));
    let config = Edn::Map(config_map);

    c.bench_function("multiline_functions", |b| {
        b.iter(|| black_box(fish_config(config.clone())))
    });
}

fn benchmark_many_aliases(c: &mut Criterion) {
    let mut aliases = BTreeMap::new();
    for i in 0..1000 {
        aliases.insert(
            Edn::Key(Box::leak(format!("alias_{:04}", i).into_boxed_str())),
            Edn::Str(Box::leak(
                format!("command_{} --flag-{} --option={}", i, i, i).into_boxed_str(),
            )),
        );
    }

    let mut config_map = BTreeMap::new();
    config_map.insert(Edn::Key("aliases"), Edn::Map(aliases));
    let config = Edn::Map(config_map);

    c.bench_function("many_aliases_1000", |b| {
        b.iter(|| black_box(fish_config(config.clone())))
    });
}

fn benchmark_string_concatenation(c: &mut Criterion) {
    // Benchmark the string building performance
    let config = create_large_config();

    c.bench_function("string_building_large", |b| {
        b.iter(|| {
            let result = black_box(fish_config(config.clone()));
            // Force evaluation of the entire string
            black_box(result.len());
        })
    });
}

criterion_group!(
    benches,
    benchmark_plugin_config,
    benchmark_fish_config_empty,
    benchmark_fish_config_small,
    benchmark_fish_config_medium,
    benchmark_fish_config_large,
    benchmark_config_parsing_and_generation,
    benchmark_multiline_functions,
    benchmark_many_aliases,
    benchmark_string_concatenation
);

criterion_main!(benches);
