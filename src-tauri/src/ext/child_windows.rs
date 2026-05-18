use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

const CHILD_WINDOW_LABEL: &str = "child";

/// Opens (or focuses) an in-app child webview for a full URL.
pub fn open_child_window(app: &AppHandle, url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let target = url.parse::<url::Url>()?;

    // Create the window if missing; otherwise just navigate.
    if let Some(win) = app.get_webview_window(CHILD_WINDOW_LABEL) {
        win.navigate(target)?;
        return Ok(());
    }

    let mut window_builder =
        WebviewWindowBuilder::new(app, CHILD_WINDOW_LABEL, WebviewUrl::External(target))
            .title("Rule34Video")
            .initialization_script(crate::ext::navigation::build_init_script())
            .inner_size(1000.0, 700.0)
            .resizable(true);
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    {
        window_builder = window_builder.center();
    }
    window_builder.build()?;

    Ok(())
}
