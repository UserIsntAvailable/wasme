use eyre::Result;
use once_cell::sync::Lazy;
use serde_json::{json, to_writer_pretty};
use std::{
    fs::{create_dir_all, read_to_string, write, File},
    path::PathBuf,
};
use toml::Value;
use wasm_pack::command::build::{Build, BuildOptions, Target};

static GEN_PATH: Lazy<PathBuf> = Lazy::new(|| PathBuf::from("dist/"));

fn main() -> Result<()> {
    let env: Option<&'static str> = option_env!("WASME_IGNORE_BUILD_SCRIPT");
    let env = env.unwrap_or_else(|| "false");

    if env.to_uppercase() != "TRUE".to_owned() {
        let package: Value = include_str!("Cargo.toml").parse()?;
        let package = &package["package"];

        create_dir_all(GEN_PATH.as_path())?;
        // TODO: Package with `trunk build --release` the `src`.
        //
        // Remember to package it before `wasm-pack`, because `trunk` removes all files in the
        // directory.
        generate_wasm_files(package)?;
        generate_manifest_json(package)?;
    };

    Ok(())
}

fn generate_wasm_files(package: &Value) -> Result<()> {
    let package_name = package["name"]
        .as_str()
        .expect("package.name is set.")
        .to_owned();

    // FIX: Will the `trunk` dist folder work for the html web extension?
    let mut command = Build::try_from_opts(BuildOptions {
        path: Some(PathBuf::from("scripts/")),
        disable_dts: true,
        target: Target::NoModules,
        out_name: Some(package_name.clone()),
        out_dir: format!("../{}", GEN_PATH.to_string_lossy()),
        ..Default::default()
    })
    .expect("wasm-pack command was not able to be parsed.");

    command.run().expect("wasm-pack build did not succeeded.");

    let mut gen_js = GEN_PATH.join(package_name.clone());
    gen_js.set_extension("js");

    let gen_js_contents = read_to_string(&gen_js)?;

    write(
        gen_js,
        format!(
            "(function () {{

{gen_js_contents}
    wasm_bindgen(browser.runtime.getURL('{package_name}_bg.wasm'));
}})()"
        ),
    )?;

    Ok(())
}

fn generate_manifest_json(package: &Value) -> Result<()> {
    let mut manifest_json = json!({
        // Used as default if not provided.
        "manifest_version": 2,

        "name": package["name"],
        "description": package["description"],
        "homepage_url": package["homepage"],
        "version": package["version"],
    });

    if let Some(manifest) = package.get("metadata").and_then(|m| m.get("manifest_json")) {
        let manifest_table = manifest
            .as_table()
            .expect("The `package.metadata.manifest_json` is not well formatted.")
            .iter()
            .map(|(key, value)| {
                (
                    key.to_owned(),
                    serde_json::to_value(value).expect("valid value."),
                )
            });

        manifest_json
            .as_object_mut()
            .expect("valid object.")
            .extend(manifest_table);
    }

    // TODO: include `res` files into `web_accessible_resources`.

    to_writer_pretty(
        File::create(GEN_PATH.join("manifest.json"))?,
        &manifest_json,
    )?;

    Ok(())
}
