#[cfg(target_os = "linux")]
mod imp {
    use std::collections::BTreeSet;
    use std::net::IpAddr;
    use std::process::Command;

    use anyhow::{anyhow, Context, Result};

    pub struct DNSManager;

    impl DNSManager {
        pub fn new() -> DNSManager {
            DNSManager
        }

        pub fn set_dns(
            &mut self,
            interface: &str,
            dns_servers: Vec<String>,
            domain_split: Vec<String>,
        ) -> Result<()> {
            let dns_servers = dns_servers
                .into_iter()
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>();

            if dns_servers.is_empty() {
                return Ok(());
            }

            let route_domains = normalize_route_domains(domain_split);

            Command::new("resolvectl")
                .arg("dns")
                .arg(interface)
                .args(&dns_servers)
                .status()
                .with_context(|| format!("failed to set per-link DNS servers for {interface}"))?
                .success()
                .then_some(())
                .ok_or_else(|| anyhow!("resolvectl dns returned non-zero exit code"))?;

            // Ensure the VPN link is not used as the default DNS route (only for routed domains).
            Command::new("resolvectl")
                .arg("default-route")
                .arg(interface)
                .arg("no")
                .status()
                .with_context(|| format!("failed to disable DNS default-route for {interface}"))?
                .success()
                .then_some(())
                .ok_or_else(|| anyhow!("resolvectl default-route returned non-zero exit code"))?;

            if !route_domains.is_empty() {
                Command::new("resolvectl")
                    .arg("domain")
                    .arg(interface)
                    .args(&route_domains)
                    .status()
                    .with_context(|| format!("failed to set per-link routed domains for {interface}"))?
                    .success()
                    .then_some(())
                    .ok_or_else(|| anyhow!("resolvectl domain returned non-zero exit code"))?;
            }

            // Best effort; ignore failures since this is not required for correctness.
            let _ = Command::new("resolvectl").arg("flush-caches").status();

            log::info!(
                "configured systemd-resolved on {interface}: dns_servers={}, routed_domains={}",
                dns_servers.join(","),
                route_domains.len()
            );
            Ok(())
        }

        pub fn restore_dns(&self, interface: &str) -> Result<()> {
            // If resolvectl is missing or systemd-resolved isn't present, restoring is a no-op.
            match Command::new("resolvectl")
                .arg("revert")
                .arg(interface)
                .status()
            {
                Ok(status) if status.success() => Ok(()),
                Ok(status) => Err(anyhow!(
                    "resolvectl revert returned non-zero exit code: {}",
                    status
                )),
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
                Err(e) => Err(anyhow::Error::new(e).context("failed to run resolvectl revert")),
            }
        }
    }

    fn normalize_route_domains(domains: Vec<String>) -> Vec<String> {
        let mut out = BTreeSet::new();
        for domain in domains {
            if let Some(normalized) = normalize_domain(&domain) {
                out.insert(format!("~{normalized}"));
            }
        }
        out.into_iter().collect()
    }

    fn normalize_domain(domain: &str) -> Option<String> {
        let mut s = domain.trim().trim_end_matches('.').to_ascii_lowercase();
        if s.is_empty() {
            return None;
        }
        if s.starts_with('~') {
            s = s.trim_start_matches('~').to_string();
        }
        if s.contains('/') {
            return None;
        }
        if s.parse::<IpAddr>().is_ok() {
            return None;
        }
        if s.contains('*') {
            s = s.replace('*', "");
        }
        while s.contains("..") {
            s = s.replace("..", ".");
        }
        let s = s.trim_matches('.').to_string();
        (!s.is_empty()).then_some(s)
    }

    #[cfg(test)]
    mod tests {
        use super::{normalize_domain, normalize_route_domains};

        #[test]
        fn normalize_domain_handles_wildcards() {
            assert_eq!(
                normalize_domain("*.Bytedance.NET"),
                Some("bytedance.net".to_string())
            );
            assert_eq!(
                normalize_domain("*.*.google.com"),
                Some("google.com".to_string())
            );
            assert_eq!(normalize_domain("*tiktokv.com"), Some("tiktokv.com".to_string()));
        }

        #[test]
        fn normalize_domain_ignores_ip_and_empty() {
            assert_eq!(normalize_domain(""), None);
            assert_eq!(normalize_domain("   "), None);
            assert_eq!(normalize_domain("10.0.0.1"), None);
        }

        #[test]
        fn normalize_route_domains_dedupes_and_prefixes() {
            let out = normalize_route_domains(vec![
                "bytedance.net".into(),
                "*.bytedance.net".into(),
                "Bytedance.Net".into(),
            ]);
            assert_eq!(out, vec!["~bytedance.net".to_string()]);
        }
    }
}

#[cfg(target_os = "macos")]
mod imp {
    use std::collections::HashMap;
    use std::process::Command;

    use anyhow::{Context, Result};

    pub struct DNSManager {
        service_dns: HashMap<String, String>,
        service_dns_search: HashMap<String, String>,
    }

    impl DNSManager {
        pub fn new() -> DNSManager {
            DNSManager {
                service_dns: HashMap::new(),
                service_dns_search: HashMap::new(),
            }
        }

        fn collect_new_service_dns(&mut self) -> Result<()> {
            let output = Command::new("networksetup")
                .arg("-listallnetworkservices")
                .output()
                .context("failed to list network services")?;

            let services = String::from_utf8_lossy(&output.stdout);
            let lines = services.lines();
            // Skip the first line's legend
            for service in lines.skip(1) {
                // Remove leading '*' and trim whitespace
                let service = service.trim_start_matches('*').trim();
                if service.is_empty() {
                    continue;
                }

                // get DNS servers
                let dns_output = Command::new("networksetup")
                    .arg("-getdnsservers")
                    .arg(service)
                    .output()
                    .with_context(|| format!("failed to get dns servers for {service}"))?;
                let dns_response = String::from_utf8_lossy(&dns_output.stdout)
                    .trim()
                    .to_string();
                // if dns config for this service is not empty, output should be ip addresses seperated in lines without space
                // otherwise, output should be "There aren't any DNS Servers set on xxx", use "Empty" instead, which can be recognized in 'networksetup -setdnsservers'
                let dns_response = if dns_response.contains(' ') {
                    "Empty".to_string()
                } else {
                    dns_response
                };

                self.service_dns
                    .insert(service.to_string(), dns_response.clone());

                // get search domain
                let search_output = Command::new("networksetup")
                    .arg("-getsearchdomains")
                    .arg(service)
                    .output()
                    .with_context(|| format!("failed to get search domains for {service}"))?;
                let search_response = String::from_utf8_lossy(&search_output.stdout)
                    .trim()
                    .to_string();
                let search_response = if search_response.contains(' ') {
                    "Empty".to_string()
                } else {
                    search_response
                };

                self.service_dns_search
                    .insert(service.to_string(), search_response.clone());

                log::debug!(
                    "DNS collected for {}, dns servers: {}, search domain: {}",
                    service,
                    dns_response,
                    search_response
                )
            }
            Ok(())
        }

        pub fn set_dns(
            &mut self,
            _interface: &str,
            dns_servers: Vec<String>,
            dns_search: Vec<String>,
        ) -> Result<()> {
            if dns_servers.is_empty() {
                return Ok(());
            }
            self.collect_new_service_dns()?;
            for service in self.service_dns.keys() {
                Command::new("networksetup")
                    .arg("-setdnsservers")
                    .arg(service)
                    .args(&dns_servers)
                    .status()
                    .with_context(|| format!("failed to set dns servers for {service}"))?;

                if !dns_search.is_empty() {
                    Command::new("networksetup")
                        .arg("-setsearchdomains")
                        .arg(service)
                        .args(&dns_search)
                        .status()
                        .with_context(|| format!("failed to set search domains for {service}"))?;
                }
                log::debug!("DNS set for {} with {}", service, dns_servers.join(","));
            }

            Ok(())
        }

        pub fn restore_dns(&self, _interface: &str) -> Result<()> {
            for (service, dns) in &self.service_dns {
                Command::new("networksetup")
                    .arg("-setdnsservers")
                    .arg(service)
                    .args(dns.lines())
                    .status()
                    .with_context(|| format!("failed to reset dns servers for {service}"))?;

                log::debug!("DNS server reset for {} with {}", service, dns);
            }
            for (service, search_domain) in &self.service_dns_search {
                Command::new("networksetup")
                    .arg("-setsearchdomains")
                    .arg(service)
                    .args(search_domain.lines())
                    .status()
                    .with_context(|| format!("failed to reset search domains for {service}"))?;
                log::debug!(
                    "DNS search domain reset for {} with {}",
                    service,
                    search_domain
                )
            }
            log::debug!("DNS reset");
            Ok(())
        }
    }
}

#[cfg(not(any(target_os = "linux", target_os = "macos")))]
mod imp {
    use anyhow::Result;

    pub struct DNSManager;

    impl DNSManager {
        pub fn new() -> DNSManager {
            DNSManager
        }

        pub fn set_dns(
            &mut self,
            _interface: &str,
            _dns_servers: Vec<String>,
            _domain_split: Vec<String>,
        ) -> Result<()> {
            Ok(())
        }

        pub fn restore_dns(&self, _interface: &str) -> Result<()> {
            Ok(())
        }
    }
}

pub use imp::DNSManager;
