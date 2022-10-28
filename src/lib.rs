#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

use egui::{Align2, Event, FontId, Id, Key, Rounding, Sense, Ui};
use std::hash::Hash;

mod target;
pub use target::*;
mod either;
pub use either::*;

/// Widget for showing the bind itself
pub struct Bind<'a, B: BindTarget> {
    id: Id,
    value: &'a mut B,
}

impl<'a, B: BindTarget> Bind<'a, B> {
    /// Creates a new bind widget
    pub fn new(id_source: impl Hash, value: &'a mut B) -> Self {
        Self {
            id: Id::new(id_source),
            value,
        }
    }

    /// Shows the bind widget
    pub fn show(self, ui: &mut Ui) -> bool {
        let id = ui.make_persistent_id(self.id);
        let changing = ui.memory().data.get_temp(id).unwrap_or(false);

        let mut size = ui.spacing().interact_size;
        size.x *= 1.25;

        let (r, p) = ui.allocate_painter(size, Sense::click());
        let vis = ui.style().interact_selectable(&r, changing);

        p.rect_filled(r.rect, Rounding::same(4.), vis.bg_fill);

        p.text(
            r.rect.center(),
            Align2::CENTER_CENTER,
            self.value.format(),
            FontId::default(),
            vis.fg_stroke.color,
        );

        if changing {
            let key = ui
                .input()
                .events
                .iter()
                .find(|e| {
                    matches!(
                        e,
                        Event::Key { pressed: true, .. }
                            | Event::PointerButton { pressed: true, .. }
                    )
                })
                .cloned();

            let reset = match key {
                Some(Event::Key {
                    key: Key::Escape, ..
                }) if B::CLEARABLE => {
                    self.value.clear();
                    true
                }
                Some(Event::Key { key, modifiers, .. }) if B::IS_KEY && r.hovered() => {
                    self.value.set_key(key, modifiers);
                    true
                }
                Some(Event::PointerButton {
                    button, modifiers, ..
                }) if B::IS_POINTER && r.hovered() => {
                    self.value.set_pointer(button, modifiers);
                    true
                }
                _ if !r.hovered() => true,
                _ => false,
            };

            if reset {
                ui.memory().data.insert_temp(id, false);
                return true;
            }
        }

        if r.clicked() {
            ui.memory().data.insert_temp(id, true);
        }

        false
    }
}
