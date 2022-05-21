mod args;
mod patch;

use {
    crate::patch::*,
    anyhow::*,
    std::{fs, path::Path},
    yaml_rust::*,
};

fn parse(path: &Path) -> Result<Vec<Yaml>> {
    let s = fs::read_to_string(path)?;
    Ok(YamlLoader::load_from_str(&s)?)
}

fn main() -> Result<()> {
    let args: args::Args = argh::from_env();
    if args.version {
        println!("SafeCloset {}", env!("CARGO_PKG_VERSION"),);
        return Ok(());
    }
    let mut base = parse(&args.base)?;
    if base.len() > 1 {
        bail!("unexpected length of base");
    }
    let mut base = base.drain(..).next().unwrap();
    let patch = parse(&args.patch)?;
    apply_patch(&mut base, patch)?;
    let mut out = String::new();
    let mut writer = YamlEmitter::new(&mut out);
    writer.dump(&base)?;
    let out = fix_tilde_err(out);
    println!("{}", out);
    Ok(())
}

// yaml 在输出为字符串时，空字符串值会被替换为波浪号 ~
// 现象类似 https://github.com/dtolnay/serde-yaml/issues/87
fn fix_tilde_err(s: String) -> String {
    use regex::Regex;
    let s_n = s.replace(": ~\n", ": \n");
    let re = Regex::new(r": ~$").unwrap();
    re.replace_all(&s_n, ":\n").to_string()
}
