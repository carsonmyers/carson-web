use std::fs::File;
use std::io::prelude::*;

use anyhow::Result;
use toml::Table;

fn main() -> Result<()> {
    let theme = {
        let mut file = File::open("Theme.toml")?;
        let mut theme_config = String::new();
        file.read_to_string(&mut theme_config)?;

        theme_config.parse::<Table>()?
    };

    generate_theme(theme)?;

    Ok(())
}

fn generate_theme(config: Table) -> Result<()> {
    let mut output = File::create("styles/theme.scss")?;
    writeln!(&mut output, "@use 'sass:color';")?;

    let steps = config
        .get("color-steps")
        .expect("color steps")
        .as_integer()
        .expect("color steps");
    assert!(steps > 0, "steps must be a positive integer");
    assert!(steps <= 100, "steps must be at most 100");

    let colors = config
        .get("colors")
        .expect("colors table")
        .as_table()
        .expect("colors table");

    for (name, color) in colors {
        let color = color.as_str().expect("color value");
        writeln!(&mut output, "{}", generate_colors(color, name, steps))?;
    }

    writeln!(&mut output, "{}", generate_colors("#CCCCCC", "gray", steps))?;

    Ok(())
}

fn generate_colors(color: &str, name: &str, steps: i64) -> String {
    let interval = 100 / steps;
    (0..=steps)
        .into_iter()
        .map(|v| v * interval)
        .flat_map(|v| {
            [
                format!("${name}-{v}: color.change({color}, $lightness: {v}%);"),
                format!(".fg-{name}-{v} {{ color: ${name}-{v}; }}"),
                format!(".bg-{name}-{v} {{ background-color: ${name}-{v}; }}"),
            ]
        })
        .collect::<Vec<_>>()
        .join("\n")
}
