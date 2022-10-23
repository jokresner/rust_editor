mod document;
#[warn(clippy::all, clippy::pedantic, clippy::restriction)]
#[allow(clippy::else_if_without_else, clippy::missing_docs_in_private_items)]
mod editor;
mod row;
mod terminal;

pub use document::Document;
pub use editor::Position;
pub use row::Row;
pub use terminal::Terminal;

use editor::Editor;

fn main() {
    Editor::default().run();
}
