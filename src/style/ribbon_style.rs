use limn::prelude::*;
use limn::resources;
use limn::resources::font::FontDescriptor;
use limn::draw::rect::RectStyle;
use limn::draw::text::TextStyle;
use limn::draw::ellipse::EllipseStyle;
use limn::color::Color;

// -- Colors

// Main colors

/// Color for the main "blue" button in the top left (dark blue)
pub const COL_W_PRIMARY_BUTTON_BLUE: Color = Color(0x2F61CBFF);
/// Selection color for toggled on, activated or selected items (light blue)
pub const COL_W_SELECTION_BLUE: Color = Color(0xC8E4FFFF);
/// Outline color for toggled on, activated or selected items (regular blue)
pub const COL_W_SELECTION_OUTLINE_BLUE: Color = Color(0xA8D2FFFF);
/// Color for a selected item in the explorer sidebar (grey)
pub const COL_W_SELECTED_GREY_SIDEBAR: Color = Color(0xD8D8D8FF);
/// Unfocused outline of the main path view (light grey)
/// Also used for the unfocused searchbar, search icon, etc.
pub const COL_W_UNFOCUSED_OUTLINE_GREY: Color = Color(0xDCDCDCFF);

// Button colors

/// Fill color of a button
pub const COL_W_BUTTON_FILL_GREY: Color = Color(0xDDDDDDFF);
/// Outline color for buttons
pub const COL_W_BUTTON_OUTLINE_GREY: Color = Color(0x707070FF);

// Scrollbar colors

/// Background color of the search bar
pub const COL_W_SCROLLBAR_LIGHT_GREY: Color = Color(0xF0F0F0FF);
/// Main color of the draggable handle
pub const COL_W_SCROLLBAR_GRIP_GREY: Color = Color(0xCCCCCCFF);
/// Color of the arrows. May be useful to do these via icons or gl textures
pub const COL_W_SCROLLBAR_ARROWS_GREY: Color = Color(0x666666FF);

// -- Measurements / paddings, margins, etc.

/// The height of a regular button
pub const MEAS_DEFAULT_BUTTON_HEIGHT_NOBORDER: u32 = 19;
/// Outline width of a button
pub const MEAS_DEFAULT_BUTTON_OUTLINE_WIDTH: u32 = 1;
/// The checkbox width and height
pub const MEAS_DEFAULT_CHECKBOX_HEIGHT_WIDTH_NOBORDER: u32 = 11;
/// Outline width of a checkbox
pub const MEAS_DEFAULT_CHECKBOX_OUTLINE_WIDTH: u32 = 1;
/// The margin from the outer checkbox border to the first character
/// of the text belonging to the checkbox
///
/// Texts on checkboxes are top-aligned, not centered - meaning if
/// checkboxes are 13 px (11 px + 2 x 1px for border) and the text is
/// 10px high, there is a bottom gap (below the text) of 3px.
pub const MEAS_DEFAULT_CHECKBOX_MARGIN_RIGHT: u32 = 11;

// TODO: The layout seems too small

/// Default styles
pub fn add_ribbon_style() {
    let mut res = resources::resources();

    res.font_loader.register_font_data(
        FontDescriptor::from_family("Lato-Regular"),
        include_bytes!("../../assets/fonts/Lato/Lato-Regular.ttf").to_vec());

    res.theme.register_type_style(EllipseStyle::default());
    res.theme.register_type_style(RectStyle::default());
    res.theme.register_type_style(TextStyle {
        font: Some(FontDescriptor::from_family("Lato-Regular")),
        font_size: Some(10.0),
        text_color: Some(BLACK),
        background_color: Some(TRANSPARENT),
        wrap: Some(Wrap::Whitespace),
        align: Some(Align::Start),
        ..TextStyle::default()
    });
    res.theme.register_class_prop_style("static_text", INACTIVE.clone(), TextStyle {
        text_color: Some(GRAY_50),
        ..TextStyle::default()
    });
    res.theme.register_class_style("list_item_rect", RectStyle {
        background_color: Some(GRAY_30),
        ..RectStyle::default()
    });
    res.theme.register_class_prop_style("list_item_rect", SELECTED.clone(), RectStyle {
        background_color: Some(COL_W_SELECTION_BLUE),
        corner_radius: Some(Some(0.0)), // limn bug
        border: Some(Some((1.0, COL_W_SELECTION_OUTLINE_BLUE))),
        ..RectStyle::default()
    });
    res.theme.register_class_prop_style("list_item_rect", MOUSEOVER.clone(), RectStyle {
        background_color: Some(GRAY_60),
        ..RectStyle::default()
    });
    res.theme.register_class_style("list_item_text", TextStyle {
        text_color: Some(WHITE),
        ..TextStyle::default()
    });
    res.theme.register_class_style("button_rect", RectStyle {
        background_color: Some(COL_W_BUTTON_FILL_GREY),
        corner_radius: Some(Some(0.0)), // limn bug
        border: Some(Some((1.0, COL_W_SELECTION_OUTLINE_BLUE))),
        ..RectStyle::default()
    });
    res.theme.register_class_prop_style("button_rect", INACTIVE.clone(), RectStyle {
        background_color: Some(GRAY_90),
        border: Some(Some((1.0, COL_W_BUTTON_FILL_GREY))),
        ..RectStyle::default()
    });
    res.theme.register_class_prop_style("button_rect", ACTIVATED_PRESSED.clone(), RectStyle {
        background_color: Some(COL_W_SELECTION_BLUE),
        corner_radius: Some(Some(0.0)), // limn bug
        border: Some(Some((1.0, COL_W_SELECTION_OUTLINE_BLUE))),
        ..RectStyle::default()
    });
    res.theme.register_class_prop_style("button_rect", ACTIVATED.clone(), RectStyle {
        background_color: Some(COL_W_SELECTION_BLUE),
        corner_radius: Some(Some(0.0)), // limn bug
        border: Some(Some((1.0, COL_W_SELECTION_OUTLINE_BLUE))),
        ..RectStyle::default()
    });
    res.theme.register_class_prop_style("button_rect", PRESSED.clone(), RectStyle {
        background_color: Some(GRAY_60),
        ..RectStyle::default()
    });
    res.theme.register_class_prop_style("button_rect", MOUSEOVER.clone(), RectStyle {
        background_color: Some(GRAY_90),
        ..RectStyle::default()
    });
}
