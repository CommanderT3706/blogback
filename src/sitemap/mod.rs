use std::fs::File;
use std::io::Write;
use std::path::Path;
use chrono::Utc;
use quick_xml::{Reader, Writer};
use quick_xml::events::Event;
use serde::Serialize;
use crate::db;

#[derive(Serialize, Debug)]
pub struct Url {
    pub loc: String,
    pub lastmod: String,
    pub changefreq: String,
    pub priority: f64,
}

#[derive(Serialize, Debug)]
#[serde(rename = "urlset")]
pub struct UrlSet {
    #[serde(rename = "url")]
    pub url: Vec<Url>
}

pub async fn generate_sitemap() {
    println!("Generating sitemap...");

    let mut urls: Vec<Url> = Vec::new();
    let sitemap_config = crate::config::read_config().site;
    let current_date = Utc::now().format("%Y-%m-%d").to_string();

    // Homepage
    urls.push(Url {
        loc: sitemap_config.homepage,
        lastmod: current_date.clone(),
        changefreq: sitemap_config.default_changefreq.clone(),
        priority: 1.0
    });

    // Posts Page
    urls.push(Url {
        loc: sitemap_config.posts_page,
        lastmod: current_date.clone(),
        changefreq: sitemap_config.default_changefreq.clone(),
        priority: 1.0
    });

    // All static pages
    for page in sitemap_config.static_pages {
        urls.push(Url {
            loc: page,
            lastmod: sitemap_config.static_lastmod.clone(),
            changefreq: sitemap_config.default_changefreq.clone(),
            priority: 1.0
        });
    }

    // All posts
    let posts = db::get_posts().await;
    for post in posts {
        urls.push(Url {
            loc: post.path,
            lastmod: if let Some(date) = post.date {
                date.format("%Y-%m-%d").to_string()
            } else { current_date.clone() },
            changefreq: sitemap_config.default_changefreq.clone(),
            priority: 0.8
        });
    }

    let sitemap = UrlSet { url: urls };

    let mut xml = quick_xml::se::to_string(&sitemap).expect("Failed to serialize XML");
    let xml_declaration = r#"<?xml version="1.0" encoding="UTF-8"?>"#;
    let namespace = r#" xmlns="http://www.sitemaps.org/schemas/sitemap/0.9""#;

    xml.insert_str(0, xml_declaration);
    let xml = xml.replace("<urlset>", &format!("<urlset{}>", namespace));

    println!("Writing file...");

    let mut sitemap_file =
        File::create(Path::join(crate::config::read_config().server.server_root.as_ref(), "sitemap.xml"))
            .expect("Failed to create sitemap.xml file");
    sitemap_file.write_all(xml.as_bytes()).expect("Failed to write to sitemap.xml");

    println!("Done!");
}