/// A curated list of known ad-network and tracker hostnames to block.
///
/// This list is intentionally kept to a manageable size of well-known,
/// unambiguously-ad/tracking-only domains.  It is the **single source of
/// truth** used by both:
///   • the Rust navigation handler (`is_blocked_url`), and
///   • the JavaScript injection (`adblock_script`).
pub const BLOCKED_DOMAINS: &[&str] = &[
    // Google advertising
    "doubleclick.net",
    "googlesyndication.com",
    "googleadservices.com",
    "adservice.google.com",
    "pagead2.googlesyndication.com",
    // Amazon advertising
    "aax.amazon-adsystem.com",
    "amazon-adsystem.com",
    // Social-media ad networks
    "an.facebook.com",
    "connect.facebook.net",
    "ads.twitter.com",
    "ads.linkedin.com",
    "platform.linkedin.com",
    // Programmatic / RTB networks
    "advertising.com",
    "adnxs.com",
    "adsrvr.org",
    "rubiconproject.com",
    "pubmatic.com",
    "openx.net",
    "casalemedia.com",
    "criteo.com",
    "criteo.net",
    "tapad.com",
    "eyeota.net",
    "bidswitch.net",
    "bidswitch.com",
    "thetradedesk.com",
    "smartadserver.com",
    "moatads.com",
    "moat.com",
    "appnexus.com",
    "contextweb.com",
    "lkqd.net",
    "indexww.com",
    "33across.com",
    "sovrn.com",
    "lijit.com",
    "spotxchange.com",
    "spotx.tv",
    "undertone.com",
    "exponential.com",
    "tribalfusion.com",
    "yieldmanager.com",
    // Native-ad / content-recommendation networks
    "outbrain.com",
    "taboola.com",
    "revcontent.com",
    "zergnet.com",
    // Analytics / behavioural trackers
    "hotjar.com",
    "fullstory.com",
    "mixpanel.com",
    "amplitude.com",
    "segment.com",
    "segment.io",
    "quantserve.com",
    "scorecardresearch.com",
    "chartbeat.com",
    "clicktale.net",
    "crazyegg.com",
    // Social-sharing widgets (tracking component)
    "addthis.com",
    "sharethis.com",
];

/// Returns `true` when the supplied URL string belongs to a blocked domain.
///
/// Both exact matches (`doubleclick.net`) and sub-domain matches
/// (`x.doubleclick.net`, `x.y.doubleclick.net`) are caught.
pub fn is_blocked_url(url: &str) -> bool {
    let hostname = extract_hostname(url);
    BLOCKED_DOMAINS
        .iter()
        .any(|&blocked| matches_hostname(&hostname, blocked))
}

/// Returns `true` iff `hostname` is exactly `blocked` or is a sub-domain of it.
/// No heap allocation is performed.
fn matches_hostname(hostname: &str, blocked: &str) -> bool {
    hostname == blocked
        || hostname
            .strip_suffix(blocked)
            .is_some_and(|prefix| prefix.ends_with('.'))
}

fn extract_hostname(url: &str) -> String {
    // Use the `url` crate for robust parsing (handles IPv6, userinfo, etc.).
    if let Ok(parsed) = url::Url::parse(url) {
        if let Some(host) = parsed.host_str() {
            return host.to_lowercase();
        }
    }
    // Fallback: simple string slicing for scheme-less or malformed inputs.
    let without_scheme = url.find("://").map_or(url, |pos| &url[pos + 3..]);
    let without_path = without_scheme.split('/').next().unwrap_or(without_scheme);
    let without_port = without_path.split(':').next().unwrap_or(without_path);
    without_port.to_lowercase()
}

/// Returns the JavaScript snippet that should be injected into every page.
///
/// Pulling the domain list from the same Rust constant ensures the Rust
/// navigation-level block and the JS in-page block always stay in sync.
pub fn adblock_script() -> String {
    // Build the JS set literal from BLOCKED_DOMAINS
    let domains_js = BLOCKED_DOMAINS
        .iter()
        .map(|d| format!("        \"{d}\""))
        .collect::<Vec<_>>()
        .join(",\n");

    format!(
        r#"(function () {{
    'use strict';

    // ---------------------------------------------------------------------------
    // Domain blocklist (generated from the same list used by the Rust backend)
    // ---------------------------------------------------------------------------
    const BLOCKED_DOMAINS = new Set([
{domains_js}
    ]);

    function isBlocked(url) {{
        if (!url) return false;
        try {{
            const hostname = new URL(url, window.location.href).hostname.toLowerCase();
            for (const domain of BLOCKED_DOMAINS) {{
                if (hostname === domain || hostname.endsWith('.' + domain)) {{
                    return true;
                }}
            }}
        }} catch (_) {{}}
        return false;
    }}

    // ---------------------------------------------------------------------------
    // Block fetch() requests to ad domains
    // ---------------------------------------------------------------------------
    const _fetch = window.fetch.bind(window);
    window.fetch = function (resource, init) {{
        const url = resource instanceof Request ? resource.url
                  : typeof resource === 'string'  ? resource
                  : String(resource);
        if (isBlocked(url)) {{
            return Promise.reject(new TypeError('Request blocked by adblock'));
        }}
        return _fetch(resource, init);
    }};

    // ---------------------------------------------------------------------------
    // Block XMLHttpRequest to ad domains
    // ---------------------------------------------------------------------------
    const _xhrOpen = XMLHttpRequest.prototype.open;
    XMLHttpRequest.prototype.open = function (method, url) {{
        this._adblockBlocked = isBlocked(String(url));
        return _xhrOpen.apply(this, arguments);
    }};
    const _xhrSend = XMLHttpRequest.prototype.send;
    XMLHttpRequest.prototype.send = function () {{
        if (this._adblockBlocked) return;
        return _xhrSend.apply(this, arguments);
    }};

    // ---------------------------------------------------------------------------
    // Block <script>, <iframe>, and <img> elements whose src points to ad domains
    // ---------------------------------------------------------------------------
    const _createElement = document.createElement.bind(document);
    const INTERCEPTED_TAGS = new Set(['script', 'iframe', 'img']);

    document.createElement = function (tag) {{
        const el = _createElement.apply(document, arguments);
        if (typeof tag !== 'string' || !INTERCEPTED_TAGS.has(tag.toLowerCase())) {{
            return el;
        }}
        const proto = Object.getPrototypeOf(el);
        const srcDesc = Object.getOwnPropertyDescriptor(proto, 'src');
        if (!srcDesc) return el;
        Object.defineProperty(el, 'src', {{
            get() {{
                return srcDesc.get ? srcDesc.get.call(this) : undefined;
            }},
            set(val) {{
                if (!isBlocked(String(val))) {{
                    if (srcDesc.set) srcDesc.set.call(this, val);
                }}
            }},
            configurable: true,
        }});
        return el;
    }};

    // ---------------------------------------------------------------------------
    // CSS-based element hiding for common ad containers
    // ---------------------------------------------------------------------------
    const AD_SELECTORS = [
        'ins.adsbygoogle',
        '.adsbygoogle',
        '[id^="ad-container"]',
        '[id^="ads-container"]',
        '[class*="banner-ad"]',
        '[class*="ad-banner"]',
        '[class*="google-ad"]',
        '[class*="adsense"]',
        '[id*="google_ads"]',
        'iframe[src*="doubleclick.net"]',
        'iframe[src*="googlesyndication.com"]',
        'iframe[src*="amazon-adsystem.com"]',
    ].join(', ');

    function injectHidingStyle() {{
        const style = _createElement('style');
        style.id = '__adblock_hide__';
        style.textContent = AD_SELECTORS + ' {{ display: none !important; }}';
        (document.head || document.documentElement).appendChild(style);
    }}

    if (document.readyState === 'loading') {{
        document.addEventListener('DOMContentLoaded', injectHidingStyle, {{ once: true }});
    }} else {{
        injectHidingStyle();
    }}
}}());
"#
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blocks_exact_domain() {
        assert!(is_blocked_url("https://doubleclick.net/ad"));
    }

    #[test]
    fn blocks_subdomain() {
        assert!(is_blocked_url(
            "https://pagead2.googlesyndication.com/pagead/js/adsbygoogle.js"
        ));
    }

    #[test]
    fn blocks_multi_level_subdomain() {
        assert!(is_blocked_url("https://ads.eu.doubleclick.net/ad?id=1"));
    }

    #[test]
    fn allows_site_domain() {
        assert!(!is_blocked_url("https://rule34video.com//page"));
    }

    #[test]
    fn allows_unrelated_domain() {
        assert!(!is_blocked_url("https://example.com/file.js"));
    }

    #[test]
    fn adblock_script_contains_domains() {
        let script = adblock_script();
        assert!(script.contains("doubleclick.net"));
        assert!(script.contains("taboola.com"));
    }
}
