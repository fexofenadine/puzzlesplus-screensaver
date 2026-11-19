//a windows screensaver (.scr) is a program windows starts with one of three switches
//  /s       show it fullscreen
//  /c       its settings (also with no switch, or /c:<window>)
//  /p <hwnd> draw it inside the little monitor of windows' screen saver settings
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{AppHandle, WebviewUrl, WebviewWindow, WebviewWindowBuilder};

enum Mode {
    Show,
    Settings,
    Preview(isize),
    Nothing,
}

//windows writes the switches as /s, /S, -s, /c:1234 or /p 1234
fn mode() -> Mode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let first = args.first().map(|arg| arg.to_lowercase()).unwrap_or_default();
    let flag = first.trim_start_matches(['/', '-']);
    let (key, rest) = match flag.split_once(':') {
        Some((key, rest)) => (key.to_string(), Some(rest.to_string())),
        None => (flag.to_string(), None),
    };
    match key.as_str() {
        "s" => Mode::Show,
        "p" | "l" => rest
            .or_else(|| args.get(1).cloned())
            .and_then(|hwnd| hwnd.trim().parse().ok())
            .map(Mode::Preview)
            .unwrap_or(Mode::Nothing),
        _ => Mode::Settings,
    }
}

//the page asks to end: a key, a click or a moved mouse, or the settings' close
#[tauri::command]
fn quit(app: AppHandle) {
    app.exit(0)
}

//the settings' preview button: the screensaver fullscreen, as its own run
#[tauri::command]
fn preview() {
    if let Ok(me) = std::env::current_exe() {
        let _ = std::process::Command::new(me).arg("/s").spawn();
    }
}

fn page<'a>(app: &'a AppHandle, label: &str, path: &str) -> WebviewWindowBuilder<'a, tauri::Wry, AppHandle> {
    WebviewWindowBuilder::new(app, label, WebviewUrl::App(path.into()))
}

//the window becomes a child of the settings' little monitor and fills it
#[cfg(windows)]
fn embed(window: &WebviewWindow, parent: isize) -> tauri::Result<()> {
    use windows_sys::Win32::Foundation::{HWND, POINT, RECT};
    use windows_sys::Win32::Graphics::Gdi::ClientToScreen;
    use windows_sys::Win32::UI::WindowsAndMessaging::*;
    let hwnd = window.hwnd()?.0 as HWND;
    let parent = parent as HWND;
    unsafe {
        let mut rect: RECT = std::mem::zeroed();
        GetClientRect(parent, &mut rect);
        SetParent(hwnd, parent);
        let style = GetWindowLongPtrW(hwnd, GWL_STYLE);
        let style = (style & !((WS_POPUP | WS_CAPTION | WS_THICKFRAME) as isize)) | WS_CHILD as isize;
        SetWindowLongPtrW(hwnd, GWL_STYLE, style);
        let flags = SWP_NOZORDER | SWP_NOACTIVATE | SWP_FRAMECHANGED | SWP_SHOWWINDOW;
        let (width, height) = (rect.right - rect.left, rect.bottom - rect.top);
        SetWindowPos(hwnd, std::ptr::null_mut(), 0, 0, width, height, flags);
        //the window keeps a frame round its page, so it grows past the monitor by that much
        let mut client: RECT = std::mem::zeroed();
        let mut outer: RECT = std::mem::zeroed();
        let mut origin = POINT { x: 0, y: 0 };
        GetClientRect(hwnd, &mut client);
        GetWindowRect(hwnd, &mut outer);
        ClientToScreen(hwnd, &mut origin);
        let (left, top) = (origin.x - outer.left, origin.y - outer.top);
        let extra_w = (outer.right - outer.left) - client.right;
        let extra_h = (outer.bottom - outer.top) - client.bottom;
        SetWindowPos(hwnd, std::ptr::null_mut(), -left, -top, width + extra_w, height + extra_h, flags);
    }
    Ok(())
}

//the preview ends with the dialog that holds it
#[cfg(windows)]
fn follow(app: AppHandle, parent: isize) {
    use windows_sys::Win32::UI::WindowsAndMessaging::IsWindow;
    std::thread::spawn(move || loop {
        std::thread::sleep(std::time::Duration::from_millis(500));
        if unsafe { IsWindow(parent as _) } == 0 {
            app.exit(0);
            break;
        }
    });
}

fn main() {
    let mode = mode();
    if let Mode::Nothing = mode {
        return;
    }
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![quit, preview])
        .setup(move |app| {
            let app = app.handle();
            match mode {
                Mode::Show => {
                    page(app, "show", "screensaver/index.html")
                        .title("Puzzles Plus Screensaver")
                        .fullscreen(true)
                        .decorations(false)
                        .always_on_top(true)
                        .skip_taskbar(true)
                        .focused(true)
                        .build()?;
                }
                Mode::Settings => {
                    page(app, "settings", "screensaver/settings.html")
                        .title("Puzzles Plus Screensaver")
                        .inner_size(620.0, 820.0)
                        .center()
                        .build()?;
                }
                Mode::Preview(parent) => {
                    let window = page(app, "preview", "screensaver/index.html?preview")
                        .decorations(false)
                        .skip_taskbar(true)
                        .focused(false)
                        .visible(false)
                        .build()?;
                    #[cfg(windows)]
                    {
                        embed(&window, parent)?;
                        follow(app.clone(), parent);
                    }
                    window.show()?;
                }
                Mode::Nothing => {}
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("the screensaver could not start");
}
