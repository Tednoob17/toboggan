mod vterm;

use std::cell::RefCell;
use std::rc::Rc;

use futures::channel::mpsc;
use futures::{SinkExt, StreamExt};
use gloo::console::{error, info};
use gloo::net::websocket::Message;
use gloo::net::websocket::futures::WebSocket;
use toboggan_core::TerminalConfig;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use wasm_bindgen_futures::spawn_local;
use web_sys::{Element, HtmlCanvasElement, HtmlElement, KeyboardEvent};

use self::vterm::VirtualTerminal;
use crate::components::WasmElement;
use crate::{create_and_append_element, create_shadow_root_with_style, dom_try};

const CSS: &str = include_str!("style.css");
const DEFAULT_COLS: u16 = 80;
const DEFAULT_ROWS: u16 = 24;
const DEFAULT_FONT_SIZE: f64 = 14.0;
const FONT_SIZE_STEP: f64 = 2.0;
const FONT_SIZE_MIN: f64 = 8.0;
const FONT_SIZE_MAX: f64 = 32.0;

#[derive(Debug, Default)]
pub struct TobogganTerminalElement {
    container: Option<Element>,
}

impl TobogganTerminalElement {
    pub fn start_terminal(&self, config: &TerminalConfig, api_base_url: &str) {
        let Some(container) = &self.container else {
            return;
        };

        let document = gloo::utils::document();
        let is_light = config.theme == "light";

        // Window container with theme class
        let window_class = if is_light {
            "terminal-window terminal-light"
        } else {
            "terminal-window terminal-dark"
        };
        let window_el = create_element_with_class(&document, "div", window_class);

        // Title bar
        let titlebar = create_element_with_class(&document, "div", "terminal-titlebar");

        // Traffic light buttons
        let buttons = create_element_with_class(&document, "div", "terminal-buttons");
        for class in [
            "terminal-btn terminal-btn-close",
            "terminal-btn terminal-btn-minimize",
            "terminal-btn terminal-btn-maximize",
        ] {
            let btn = create_element_with_class(&document, "div", class);
            let _ = buttons.append_child(&btn);
        }
        let _ = titlebar.append_child(&buttons);

        // Title text (show cwd basename or cmd)
        let title_text = create_element_with_class(&document, "span", "terminal-title");
        let title = config
            .cmd
            .as_deref()
            .or_else(|| {
                config
                    .cwd
                    .rsplit('/')
                    .find(|segment| !segment.is_empty())
            })
            .unwrap_or(&config.cwd);
        title_text.set_text_content(Some(title));
        let _ = titlebar.append_child(&title_text);
        let _ = window_el.append_child(&titlebar);

        // Terminal body (canvas container)
        let body = create_element_with_class(&document, "div", "terminal-body");

        let Ok(canvas) = document.create_element("canvas") else {
            error!("Failed to create canvas element");
            return;
        };
        let Ok(canvas) = canvas.dyn_into::<HtmlCanvasElement>() else {
            error!("Failed to cast to HtmlCanvasElement");
            return;
        };
        canvas.set_class_name("terminal-canvas");
        canvas.set_attribute("tabindex", "0").unwrap_throw();

        let _ = body.append_child(&canvas);
        let _ = window_el.append_child(&body);
        let _ = container.append_child(&window_el);

        canvas.focus().unwrap_throw();

        let ws_url = build_terminal_ws_url(api_base_url, config);
        let theme = config.theme.clone();

        info!("Starting terminal session:", &ws_url);

        spawn_local(async move {
            run_terminal_session(canvas, &ws_url, &theme).await;
        });
    }

    pub fn stop_terminal(&self) {
        if let Some(container) = &self.container {
            container.set_inner_html("");
        }
    }
}

impl WasmElement for TobogganTerminalElement {
    fn render(&mut self, host: &HtmlElement) {
        let root = dom_try!(
            create_shadow_root_with_style(host, CSS),
            "create shadow root"
        );

        let container: Element = dom_try!(
            create_and_append_element(&root, "div"),
            "create terminal container"
        );

        self.container = Some(container);
    }
}

fn create_element_with_class(document: &web_sys::Document, tag: &str, class: &str) -> Element {
    let Ok(el) = document.create_element(tag) else {
        return document
            .create_element("div")
            .unwrap_or_else(|_| unreachable!("could not create div element"));
    };
    el.set_class_name(class);
    el
}

/// Message from keyboard handler to terminal session
enum KeyAction {
    Input(String),
    FontIncrease,
    FontDecrease,
}

#[allow(clippy::await_holding_refcell_ref)] // Safe: single-threaded WASM
async fn run_terminal_session(canvas: HtmlCanvasElement, ws_url: &str, theme: &str) {
    let ws = match WebSocket::open(ws_url) {
        Ok(ws) => ws,
        Err(err) => {
            error!("Failed to connect to terminal:", err.to_string());
            return;
        }
    };

    let (ws_write, mut ws_read) = ws.split();
    let font_size = Rc::new(RefCell::new(DEFAULT_FONT_SIZE));
    let vterm = VirtualTerminal::new(DEFAULT_COLS, DEFAULT_ROWS, theme);

    vterm.render_to_canvas(&canvas, *font_size.borrow());

    // Set up keyboard input → channel
    let (tx_key, rx_key) = mpsc::unbounded::<KeyAction>();
    setup_keyboard_handler(&canvas, tx_key);

    // Forward keyboard input to WebSocket / handle font resize
    let ws_write = Rc::new(RefCell::new(ws_write));
    let ws_write_kbd = Rc::clone(&ws_write);
    let font_size_kbd = Rc::clone(&font_size);
    let canvas_kbd = canvas.clone();
    let vterm_rc = Rc::new(RefCell::new(vterm));
    let vterm_kbd = Rc::clone(&vterm_rc);

    spawn_local(async move {
        let mut rx_key = rx_key;
        while let Some(action) = rx_key.next().await {
            match action {
                KeyAction::Input(input) => {
                    let send_result =
                        ws_write_kbd.borrow_mut().send(Message::Text(input)).await;
                    if send_result.is_err() {
                        break;
                    }
                }
                KeyAction::FontIncrease => {
                    let mut size = font_size_kbd.borrow_mut();
                    *size = (*size + FONT_SIZE_STEP).min(FONT_SIZE_MAX);
                    vterm_kbd.borrow().render_to_canvas(&canvas_kbd, *size);
                }
                KeyAction::FontDecrease => {
                    let mut size = font_size_kbd.borrow_mut();
                    *size = (*size - FONT_SIZE_STEP).max(FONT_SIZE_MIN);
                    vterm_kbd.borrow().render_to_canvas(&canvas_kbd, *size);
                }
            }
        }
    });

    // Read terminal output from server
    while let Some(msg) = ws_read.next().await {
        match msg {
            Ok(Message::Bytes(data)) => {
                vterm_rc.borrow_mut().process(&data);
                vterm_rc
                    .borrow()
                    .render_to_canvas(&canvas, *font_size.borrow());
            }
            Ok(Message::Text(text)) => {
                vterm_rc.borrow_mut().process(text.as_bytes());
                vterm_rc
                    .borrow()
                    .render_to_canvas(&canvas, *font_size.borrow());
            }
            Err(err) => {
                error!("Terminal WebSocket error:", err.to_string());
                break;
            }
        }
    }

    info!("Terminal session ended");
    let _ = ws_write.borrow_mut().close().await;
}

fn setup_keyboard_handler(canvas: &HtmlCanvasElement, tx: mpsc::UnboundedSender<KeyAction>) {
    let closure = Closure::<dyn FnMut(_)>::new(move |event: KeyboardEvent| {
        let key = event.key();
        let meta = event.meta_key();

        // Cmd+/Cmd- for font size (don't send to terminal)
        if meta && (key == "=" || key == "+") {
            event.prevent_default();
            let _ = tx.unbounded_send(KeyAction::FontIncrease);
            return;
        }
        if meta && key == "-" {
            event.prevent_default();
            let _ = tx.unbounded_send(KeyAction::FontDecrease);
            return;
        }

        event.prevent_default();
        event.stop_propagation();

        let input = translate_key(&event);
        if !input.is_empty() {
            let _ = tx.unbounded_send(KeyAction::Input(input));
        }
    });

    canvas
        .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())
        .unwrap_throw();
    closure.forget();
}

fn translate_key(event: &KeyboardEvent) -> String {
    let key = event.key();
    let ctrl = event.ctrl_key();

    // Control key combinations (Ctrl only, not Cmd)
    if ctrl {
        return match key.as_str() {
            "c" => "\x03".to_string(),
            "d" => "\x04".to_string(),
            "z" => "\x1a".to_string(),
            "l" => "\x0c".to_string(),
            "a" => "\x01".to_string(),
            "e" => "\x05".to_string(),
            "u" => "\x15".to_string(),
            "k" => "\x0b".to_string(),
            "w" => "\x17".to_string(),
            "r" => "\x12".to_string(),
            _ => String::new(),
        };
    }

    // Special keys
    match key.as_str() {
        "Enter" => "\r".to_string(),
        "Backspace" => "\x7f".to_string(),
        "Tab" => "\t".to_string(),
        "Escape" => "\x1b".to_string(),
        "ArrowUp" => "\x1b[A".to_string(),
        "ArrowDown" => "\x1b[B".to_string(),
        "ArrowRight" => "\x1b[C".to_string(),
        "ArrowLeft" => "\x1b[D".to_string(),
        "Home" => "\x1b[H".to_string(),
        "End" => "\x1b[F".to_string(),
        "Delete" => "\x1b[3~".to_string(),
        "PageUp" => "\x1b[5~".to_string(),
        "PageDown" => "\x1b[6~".to_string(),
        // Single printable character
        ch if ch.len() == 1 => ch.to_string(),
        // Ignore modifier-only keys, etc.
        _ => String::new(),
    }
}

fn build_terminal_ws_url(api_base_url: &str, config: &TerminalConfig) -> String {
    let ws_base = api_base_url
        .replace("https://", "wss://")
        .replace("http://", "ws://");

    let encoded_cwd = String::from(js_sys::encode_uri_component(&config.cwd));
    let mut url = format!(
        "{ws_base}/api/terminal?cwd={encoded_cwd}&cols={DEFAULT_COLS}&rows={DEFAULT_ROWS}",
    );

    if let Some(cmd) = &config.cmd {
        url.push_str("&cmd=");
        url.push_str(&String::from(js_sys::encode_uri_component(cmd)));
    }

    url
}
