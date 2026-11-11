//! Compares the three sorted renderings of a configuration map with their
//! previous forms, which looked every value up again by key.

use super::{render_config_as_json, render_config_as_table, serialize_config_toml};
use std::collections::HashMap;

fn previous_serialize_config_toml(map: &HashMap<String, String>) -> String {
    let mut keys: Vec<&String> = map.keys().collect();
    keys.sort();
    let mut out = String::new();
    let mut current_section = String::new();
    for key in keys {
        let parts: Vec<&str> = key.splitn(2, '.').collect();
        if parts.len() == 2 {
            let section = parts[0];
            let field = parts[1];
            if section != current_section {
                if !current_section.is_empty() {
                    out.push('\n');
                }
                out.push_str(&format!("[{}]\n", section));
                current_section = section.to_string();
            }
            let val = map.get(key).expect("key is from map.keys()");
            out.push_str(&format!("{} = \"{}\"\n", field, val));
        } else {
            out.push_str(&format!(
                "{} = \"{}\"\n",
                key,
                map.get(key).expect("key is from map.keys()")
            ));
        }
    }
    out
}

fn previous_render_config_as_table(map: &HashMap<String, String>) -> String {
    let mut keys: Vec<&String> = map.keys().collect();
    keys.sort();
    let max_key_len = keys.iter().map(|k| k.len()).max().unwrap_or(10);
    let mut out = String::new();
    out.push_str(&format!(
        "{:<width$}  {}\n",
        "KEY",
        "VALUE",
        width = max_key_len
    ));
    out.push_str(&"-".repeat(max_key_len + 20));
    out.push('\n');
    for key in keys {
        let val = map.get(key).expect("key is from map.keys()");
        out.push_str(&format!("{:<width$}  {}\n", key, val, width = max_key_len));
    }
    out
}

fn previous_render_config_as_json(map: &HashMap<String, String>) -> String {
    let mut keys: Vec<&String> = map.keys().collect();
    keys.sort();
    let mut out = String::from("{\n");
    for (i, key) in keys.iter().enumerate() {
        let val = map.get(*key).expect("key is from map.keys()");
        let comma = if i + 1 < map.len() { "," } else { "" };
        out.push_str(&format!("  \"{}\": \"{}\"{}\n", key, val, comma));
    }
    out.push('}');
    out
}

fn sample_maps() -> Vec<HashMap<String, String>> {
    let entries = [
        ("version", "1"),
        ("build.jobs", "4"),
        ("build.target", "native"),
        ("lint.level", "warn"),
        ("codegen.backend", "zig"),
        ("name", "demo"),
        ("a.b.c", "nested"),
    ];
    (0..=entries.len())
        .map(|n| {
            entries[..n]
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect()
        })
        .collect()
}

#[test]
fn renderings_match_their_previous_forms() {
    for map in sample_maps() {
        assert_eq!(
            serialize_config_toml(&map),
            previous_serialize_config_toml(&map)
        );
        assert_eq!(
            render_config_as_table(&map),
            previous_render_config_as_table(&map)
        );
        assert_eq!(
            render_config_as_json(&map),
            previous_render_config_as_json(&map)
        );
    }
}
