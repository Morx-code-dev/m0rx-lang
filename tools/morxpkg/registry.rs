// M0RX Package Registry

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub checksum: String,
    pub dependencies: Vec<String>,
    pub download_url: String,
}

pub struct Registry {
    packages: HashMap<String, Vec<Package>>,
}

impl Registry {
    pub fn new() -> Self {
        let mut reg = Registry {
            packages: HashMap::new(),
        };
        reg.load_defaults();
        reg
    }

    fn load_defaults(&mut self) {
        // Core packages
        let core_pkgs = vec![
            ("m0rx.core", "Core runtime and built-ins"),
            ("m0rx.backend", "High-level backend framework"),
            ("m0rx.ui", "UI/UX and game engine"),
            ("m0rx.ai", "AI/ML framework"),
            ("m0rx.voice", "TTS and STT library"),
            ("m0rx.ds", "Data structures"),
            ("m0rx.algo", "Algorithms"),
            ("m0rx.math", "Advanced math"),
            ("m0rx.str", "String operations"),
            ("m0rx.net", "Networking"),
            ("m0rx.db", "Database drivers"),
            ("m0rx.sec", "Security"),
            ("m0rx.sys", "System calls"),
            ("m0rx.file", "File operations"),
            ("m0rx.async", "Async runtime"),
            ("m0rx.serial", "Serialization"),
            ("m0rx.cloud", "Cloud SDKs"),
            ("m0rx.img", "Image processing"),
            ("m0rx.media", "Media processing"),
            ("m0rx.crypto", "Cryptography"),
            ("m0rx.cache", "Caching"),
            ("m0rx.log", "Logging"),
            ("m0rx.config", "Configuration"),
            ("m0rx.web", "Web scraping"),
            ("m0rx.date", "Date and time"),
            ("m0rx.pdf", "PDF processing"),
            ("m0rx.chart", "Data visualization"),
            ("m0rx.payments", "Payment gateways"),
            ("m0rx.i18n", "Internationalization"),
            ("m0rx.test", "Testing framework"),
        ];

        for (name, desc) in core_pkgs {
            let pkg = Package {
                name: name.to_string(),
                version: "0.1.0".to_string(),
                description: desc.to_string(),
                author: "M0RX Team".to_string(),
                checksum: format!("sha256:{}", name.replace('.', "")),
                dependencies: vec!["m0rx.core".to_string()],
                download_url: format!(
                    "https://pkg.m0rx.dev/{}/0.1.0",
                    name
                ),
            };
            self.packages
                .entry(name.to_string())
                .or_default()
                .push(pkg);
        }
    }

    pub fn search(&self, query: &str) -> Vec<&Package> {
        self.packages
            .values()
            .flatten()
            .filter(|p| {
                p.name.contains(query)
                    || p.description
                        .to_lowercase()
                        .contains(&query.to_lowercase())
            })
            .collect()
    }

    pub fn get(&self, name: &str, version: &str) -> Option<&Package> {
        self.packages.get(name)?.iter().find(|p| p.version == version)
    }

    pub fn list_all(&self) -> Vec<&Package> {
        self.packages.values().flatten().collect()
    }

    pub fn resolve_deps(&self, name: &str) -> Vec<String> {
        let mut deps = Vec::new();
        if let Some(versions) = self.packages.get(name) {
            if let Some(pkg) = versions.first() {
                for dep in &pkg.dependencies {
                    deps.push(dep.clone());
                    let sub = self.resolve_deps(dep);
                    deps.extend(sub);
                }
            }
        }
        deps.dedup();
        deps
    }
}
