use std::{collections::BTreeMap, fs, time::Duration};

use anyhow::{Context, Result};
use serde::Deserialize;

const BASE_URL: &str = "https://images.ygoprodeck.com/images/cards_cropped";

#[derive(Deserialize)]
struct Entry {
    image: u64,
}

fn main() -> Result<()> {
    let yaml = fs::read_to_string("data/db.yaml").context("读取 data/db.yaml 失败")?;
    let entries: BTreeMap<String, Entry> =
        serde_yaml_ng::from_str(&yaml).context("解析 data/db.yaml 失败")?;
    fs::create_dir_all("image").context("创建 image 目录失败")?;
    let agent = ureq::Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(30)))
        .build()
        .new_agent();
    for (name, entry) in entries {
        let url = format!("{BASE_URL}/{}.jpg", entry.image);
        let path = format!("image/{}.jpg", entry.image);
        let bytes = agent
            .get(&url)
            .call()
            .with_context(|| format!("下载 {name} 失败：{url}"))?
            .body_mut()
            .read_to_vec()
            .with_context(|| format!("读取 {name} 的图片失败：{url}"))?;
        fs::write(&path, bytes).with_context(|| format!("写入 {path} 失败"))?;
        println!("{name} -> {path}");
    }
    Ok(())
}
