use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::io::{Error, ErrorKind, Result};

const SOURCE_MAP_DELIMETER: &str = ";";

static ASSETS_MAP: Lazy<HashMap<&str, &str>> = Lazy::new(|| create_assets_map());

pub fn get_asset_path(asset: &str) -> Result<&str> {
    ASSETS_MAP.get(asset).map(|path| *path).ok_or(Error::new(
        ErrorKind::NotFound,
        format!("Asset path {} not found in the source map", asset),
    ))
}

fn create_assets_map() -> HashMap<&'static str, &'static str> {
    let source_map_content = include_str!("../../dist/source_map.json");
    let source_map: HashMap<&str, &str> = source_map_content
        .lines()
        .filter_map(|line| {
            let mut parts = line.splitn(2, SOURCE_MAP_DELIMETER);
            Some((parts.next()?, parts.next()?))
        })
        .collect();
    source_map
}
