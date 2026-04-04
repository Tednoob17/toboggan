use web_sys::HtmlElement;

mod footer;
pub use self::footer::*;

mod slide;
pub use self::slide::*;

mod terminal;
pub use self::terminal::*;

mod toast;
pub use self::toast::*;

pub(crate) trait WasmElement {
    fn render(&mut self, host: &HtmlElement);
}
