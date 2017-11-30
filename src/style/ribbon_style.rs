use limn::prelude::*;
use limn::resources;
use limn::resources::font::FontDescriptor;
use limn::draw::rect::RectStyle;
use limn::draw::text::TextStyle;
use limn::draw::ellipse::EllipseStyle;

/// Default styles
pub fn add_ribbon_style() {
    let mut res = resources::resources();

    res.font_loader.register_font_data(
        FontDescriptor::from_family("NotoSans-Regular"),
        include_bytes!("../../assets/fonts/Lato/Lato-Regular.ttf").to_vec());

    res.theme.register_type_style(EllipseStyle::default());
    res.theme.register_type_style(RectStyle::default());
    res.theme.register_type_style(TextStyle {
        font: Some(FontDescriptor::from_family("NotoSans-Regular")),
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
        background_color: Some(BLUE_HIGHLIGHT),
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
        background_color: Some(GRAY_80),
        corner_radius: Some(Some(5.0)),
        border: Some(Some((1.0, GRAY_40))),
        ..RectStyle::default()
    });
    res.theme.register_class_prop_style("button_rect", INACTIVE.clone(), RectStyle {
        background_color: Some(GRAY_90),
        border: Some(Some((1.0, GRAY_70))),
        ..RectStyle::default()
    });
    res.theme.register_class_prop_style("button_rect", ACTIVATED_PRESSED.clone(), RectStyle {
        background_color: Some(GRAY_30),
        ..RectStyle::default()
    });
    res.theme.register_class_prop_style("button_rect", ACTIVATED.clone(), RectStyle {
        background_color: Some(GRAY_40),
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
