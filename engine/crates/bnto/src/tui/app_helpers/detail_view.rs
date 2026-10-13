// Detail-screen view updates — viewport scroll, embedded picker, focus.

use super::super::app::AppModel;
use super::super::screens::detail::DetailFocus;
use super::super::screens::detail_scroll::DetailScrollMsg;
use super::super::screens::picker::{PickerMessage, update as picker_update};

/// Scroll the detail screen viewport.
pub(crate) fn handle_detail_scroll(model: AppModel, msg: DetailScrollMsg) -> AppModel {
    let detail = model.detail.map(|mut d| {
        match msg {
            DetailScrollMsg::ScrollDown => d.viewport.scroll_down(),
            DetailScrollMsg::ScrollUp => d.viewport.scroll_up(),
            DetailScrollMsg::PageDown => d.viewport.page_down(),
            DetailScrollMsg::PageUp => d.viewport.page_up(),
            DetailScrollMsg::GoToTop => d.viewport.go_to_top(),
            DetailScrollMsg::GoToBottom => d.viewport.go_to_bottom(),
        }
        d
    });
    AppModel { detail, ..model }
}

/// Forward a message to the detail screen's embedded picker.
pub(crate) fn handle_detail_picker(model: AppModel, msg: PickerMessage) -> AppModel {
    let detail = model.detail.map(|mut d| {
        if let Some(picker) = d.input_picker.take() {
            d.input_picker = Some(picker_update(picker, msg));
        }
        d
    });
    AppModel { detail, ..model }
}

/// Move detail focus to the given section.
pub(crate) fn handle_detail_focus(model: AppModel, focus: DetailFocus) -> AppModel {
    let detail = model.detail.map(|mut d| {
        d.focus = focus;
        d
    });
    AppModel { detail, ..model }
}
