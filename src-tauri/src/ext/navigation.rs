use super::adblock;
#[cfg(not(any(target_os = "android", target_os = "ios")))]
use super::context_menu;

/// Navigation + link handling.
///
/// This module owns the injected JS init script, including:
/// - `window.open()` interception
/// - anchor click interception (target="_blank" / modifiers)
/// - routing internal `rule34video.net` links to the in-app child window command
///   (via `open_child_window_cmd`)
/// - desktop fallback context menu support for unhandled right-clicks
///   so website-built menus can continue to work.
pub fn build_init_script() -> String {
    format!(
        "{}\n\n{}\n\n{}\n\n{}",
        init_script(),
        build_context_menu_script(),
        main_child_bridge_script(),
        adblock::adblock_script(),
    )
}

#[cfg(not(any(target_os = "android", target_os = "ios")))]
fn build_context_menu_script() -> String {
    context_menu::build_context_menu_script()
}

#[cfg(any(target_os = "android", target_os = "ios"))]
fn build_context_menu_script() -> String {
    String::new()
}

fn init_script() -> &'static str {
    r#"
(function () {
    'use strict';

    const SITE_HOSTNAME = 'rule34video.net';

    function isInternal(url) {
        try {
            const parsed = new URL(url, window.location.href);
            return (
                parsed.hostname === SITE_HOSTNAME ||
                parsed.hostname.endsWith('.' + SITE_HOSTNAME)
            );
        } catch (_) {
            return false;
        }
    }

    function openExternal(url) {
        window.__TAURI_INTERNALS__.invoke('plugin:opener|open_url', { url: url });
    }

    function openInternal(url) {
        // Command registered on the Rust side: open_child_window_cmd
        window.__TAURI_INTERNALS__.invoke('open_child_window_cmd', { url: url });
    }

    const _open = window.open.bind(window);
    window.open = function (url, target, features) {
        if (!url) return _open(url, target, features);

        let resolved;
        try { resolved = new URL(url, window.location.href).href; }
        catch (_) { return _open(url, target, features); }

        if (isInternal(resolved)) {
            openInternal(resolved);
            return null;
        }

        openExternal(resolved);
        return null;
    };

    window.addEventListener('click', function (e) {
        if (e.defaultPrevented || e.button !== 0) return;

        const anchor = e.composedPath().find(function (el) {
            return el instanceof Node && el.nodeName &&
                   el.nodeName.toUpperCase() === 'A';
        });
        if (!anchor || !anchor.href) return;

        if (anchor.hasAttribute('download')) return;

        const opensNew = anchor.target === '_blank' || e.ctrlKey || e.metaKey || e.shiftKey;
        if (!opensNew) return;

        let proto;
        try { proto = new URL(anchor.href).protocol; } catch (_) { return; }
        if (!['http:', 'https:', 'mailto:', 'tel:'].includes(proto)) return;

        e.preventDefault();

        if (isInternal(anchor.href)) {
            openInternal(anchor.href);
        } else {
            openExternal(anchor.href);
        }
    }, true);
}());
"#
}

/// Bridge for child windows to communicate back to the main window.
///
/// The site’s attachments manager is expected to call `window.opener.postMessage(...)`.
/// Since we use a managed child webview (not a real `window.open` opener relationship),
/// we polyfill `postMessage` so those calls are forwarded to Rust.
/// Rust then emits a Tauri event that main listens to.
fn main_child_bridge_script() -> String {
    r#"
(function () {
    'use strict';

    const EVENT_NAME = 'rule34video:child-post-message';

    function safeStringify(value) {
        try { return JSON.stringify(value); } catch (_) { return String(value); }
    }

    // ----------------------- Child-side forwarding --------------------------
    // If the site tries: window.opener.postMessage(payload, '*')
    // then we intercept and forward via Tauri invoke to the Rust command.
    try {
        if (window.opener && typeof window.opener === 'object') {
            const opener = window.opener;

            // Only patch once per page.
            if (!opener.__rule34videoPatchedPostMessage) {
                const original = opener.postMessage && opener.postMessage.bind(opener);

                opener.__rule34videoPatchedPostMessage = true;
                opener.postMessage = function(message, targetOrigin) {
                    console.log('[vc-bridge] opener.postMessage intercepted', { targetOrigin: targetOrigin });
                    // Forward to Rust.
                    // We pass the message as string to avoid structured clone issues.
                    try {
                        const data = safeStringify(message);
                        const origin = (typeof targetOrigin === 'string') ? targetOrigin : '*';
                        console.log('[vc-bridge] invoking child_post_message_cmd', { data_len: data?.length, origin: origin });
                        window.__TAURI_INTERNALS__.invoke('child_post_message_cmd', { data: data, origin: origin });
                    } catch (e) {
                        console.warn('[vc-bridge] interception forward failed, falling back to original', e);
                        // Best-effort: fall back to original.
                        if (original) original(message, targetOrigin);
                    }
                };
            }
        }
    } catch (_) {
        // no-op
    }

    // ----------------------- Main-side receiving -----------------------------
    // Listen for Rust-emitted payloads.
    // Consumers (attachments manager) can hook this event handler.
    function onChildPayload(e) {
        console.log('[vc-bridge] main received tauri event payload', e);
        const detail = e && e.detail ? e.detail : e;
        console.log('[vc-bridge] main dispatching DOM event detail', detail);
        window.dispatchEvent(new CustomEvent('rule34video:child-post-message:received', { detail: detail }));
    }

    // Main-side receiving:
    // With `app.withGlobalTauri=true`, events are accessible at window.__TAURI__.event
    // (see Tauri v2 docs).
    try {
        if (window.__TAURI__ && window.__TAURI__.event && typeof window.__TAURI__.event.listen === 'function') {
            window.__TAURI__.event.listen(EVENT_NAME, (event) => {
                // `event.payload` contains the Rust-emitted JSON payload.
                onChildPayload(event);
            }).catch(() => {});
        }
    } catch (_) {}

}());
"#
    .to_string()
}
