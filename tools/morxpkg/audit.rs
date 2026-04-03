// M0RX Package Security Audit

pub struct AuditResult {
    pub package: String,
    pub version: String,
    pub severity: String,
    pub description: String,
}

pub struct Auditor {
    pub results: Vec<AuditResult>,
}

impl Auditor {
    pub fn new() -> Self {
        Auditor {
            results: Vec::new(),
        }
    }

    pub fn audit_package(&mut self, name: &str, version: &str) {
        // Check known vulnerabilities
        let known_vulns = vec![
            // Example entries
            ("m0rx.old", "0.0.1", "HIGH", "Deprecated version"),
        ];

        for (pkg, ver, sev, desc) in known_vulns {
            if pkg == name && ver == version {
                self.results.push(AuditResult {
                    package: name.to_string(),
                    version: version.to_string(),
                    severity: sev.to_string(),
                    description: desc.to_string(),
                });
            }
        }
    }

    pub fn audit_all(&mut self, packages: &[(String, String)]) {
        for (name, version) in packages {
            self.audit_package(name, version);
        }
    }

    pub fn report(&self) {
        if self.results.is_empty() {
            println!("morxpkg audit: No vulnerabilities found ✓");
            return;
        }
        println!("morxpkg audit: {} vulnerability(s) found", self.results.len());
        for r in &self.results {
            println!(
                "  [{}] {} v{}: {}",
                r.severity, r.package, r.version, r.description
            );
        }
    }

    pub fn has_critical(&self) -> bool {
        self.results.iter().any(|r| r.severity == "CRITICAL")
    }
}
