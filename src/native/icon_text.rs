//! Text widget for rendering icons.
//!
//! Nearly a complete copy of the `iced_native::Text` widget, but with the
//! icons font as a default font. Maybe I'll find a better way in the future.
//!
//! //! *This API requires the following crate features to be activated: `icon_text`*
use std::hash::Hash;

use iced_native::{
    alignment::{Horizontal, Vertical},
    Color, Element, Length, Rectangle, Size, Widget,
};

/// Text widget with icon font.
#[allow(missing_debug_implementations)]
pub struct IconText<Renderer: iced_native::text::Renderer> {
    /// The content of the [`IconText`](IconText).
    content: String,
    /// The optional size of the [`IconText`](IconText).
    size: Option<u16>,
    /// The optional color of the [`IconText`](IconText).
    color: Option<Color>,
    /// The optional font of the [`IconText`](IconText).
    font: Option<Renderer::Font>,
    /// The width of the [`IconText`](IconText).
    width: Length,
    /// The height of the [`IconText`](IconText).
    height: Length,
    /// The horizontal alignment of the [`IconText`](IconText).
    horizontal_alignment: Horizontal,
    /// The vertical alignment of the [`IconText`](IconText).
    vertical_alignment: Vertical,
}

impl<Renderer: iced_native::text::Renderer> IconText<Renderer> {
    /// Creates a new [`IconText`](IconText) with the given icon label.
    ///
    /// It expects:
    ///     * the label to be displayed as an icon on the [`IconText`](IconText).
    pub fn new<T: Into<String>>(label: T) -> Self {
        Self {
            content: label.into(),
            size: None,
            color: None,
            font: None,
            width: Length::Shrink,
            height: Length::Shrink,
            horizontal_alignment: Horizontal::Center,
            vertical_alignment: Vertical::Center,
        }
    }

    /// Sets the size of the [`IconText`](IconText).
    pub fn size(mut self, size: u16) -> Self {
        self.size = Some(size);
        self
    }

    /// Sets the [`Color`](iced_native::Color) of the [`IconText`](IconText).
    pub fn color<C: Into<Color>>(mut self, color: C) -> Self {
        self.color = Some(color.into());
        self
    }

    /// Sets the [`Font`](iced_native::Font) of the [`IconText`](IconText).
    pub fn font(mut self, font: impl Into<Renderer::Font>) -> Self {
        self.font = Some(font.into());
        self
    }

    /// Sets the width of the [`IconText`](IconText) boundaries.
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the [`IconText`](IconText) boundaries.
    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    /// Sets the [`Horizontal `](iced_native::Horizontal )
    /// of the [`IconText`](IconText).
    pub fn horizontal_alignment(mut self, alignment: Horizontal) -> Self {
        self.horizontal_alignment = alignment;
        self
    }

    /// Sets the [`Vertical `](iced_native::Vertical )
    /// of the [`IconText`](IconText).
    pub fn vertical_alignment(mut self, alignment: Vertical) -> Self {
        self.vertical_alignment = alignment;
        self
    }
}

impl<Message, Renderer> Widget<Message, Renderer> for IconText<Renderer>
where
    Renderer: iced_native::Renderer + iced_native::text::Renderer,
{
    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        self.height
    }

    fn layout(
        &self,
        renderer: &Renderer,
        limits: &iced_native::layout::Limits,
    ) -> iced_native::layout::Node {
        let limits = limits.width(self.width).height(self.height);

        let size = self.size.unwrap_or_else(|| renderer.default_size());

        let bounds = limits.max();

        let (width, height) = renderer.measure(
            &self.content,
            size,
            self.font.unwrap_or_else(|| renderer.default_font()),
            bounds,
        );

        let size = limits.resolve(Size::new(width, height));

        iced_native::layout::Node::new(size)
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        style: &iced_native::renderer::Style,
        layout: iced_native::Layout<'_>,
        cursor_position: iced_graphics::Point,
        viewport: &iced_graphics::Rectangle,
    ) {
        let bounds = layout.bounds();

        let x = match self.horizontal_alignment {
            Horizontal::Left => bounds.x,
            Horizontal::Center => bounds.center_x(),
            Horizontal::Right => bounds.x + bounds.width,
        };

        let y = match self.vertical_alignment {
            Vertical::Top => bounds.y,
            Vertical::Center => bounds.center_y(),
            Vertical::Bottom => bounds.y + bounds.height,
        };

        renderer.fill_text(iced_native::text::Text {
            content: self.content,
            bounds: Rectangle { x, y, ..bounds },
            size: f32::from(self.size),
            color: self.color.unwrap_or(style.text_color),
            font: self.font.unwrap_or_else(crate::graphics::icons::ICON_FONT),
            horizontal_alignment: self.horizontal_alignment,
            vertical_alignment: self.vertical_alignment,
        })
    }

    fn hash_layout(&self, state: &mut iced_native::Hasher) {
        #[allow(clippy::missing_docs_in_private_items)]
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);

        self.content.hash(state);
        self.size.hash(state);
        self.width.hash(state);
        self.height.hash(state);
    }
}

impl<'a, Message, Renderer> From<IconText<Renderer>> for Element<'a, Message, Renderer>
where
    Renderer: iced_native::Renderer + iced_native::text::Renderer + 'a,
{
    fn from(icon: IconText<Renderer>) -> Element<'a, Message, Renderer> {
        Element::new(icon)
    }
}

impl<Renderer: iced_native::text::Renderer> Clone for IconText<Renderer> {
    fn clone(&self) -> Self {
        Self {
            content: self.content.clone(),
            size: self.size,
            color: self.color,
            font: self.font,
            width: self.width,
            height: self.height,
            horizontal_alignment: self.horizontal_alignment,
            vertical_alignment: self.vertical_alignment,
        }
    }
}
