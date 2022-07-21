use yakui_core::dom::Dom;
use yakui_core::geometry::{Constraints, Vec2};
use yakui_core::layout::LayoutDom;
use yakui_core::widget::Widget;
use yakui_core::Response;

use crate::util::widget_children;

/**
Applies padding around a single child widget.

Responds with [PadResponse].
*/
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct Pad {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

impl Pad {
    pub const ZERO: Pad = Pad::all(0.0);

    pub const fn all(value: f32) -> Self {
        Self {
            left: value,
            right: value,
            top: value,
            bottom: value,
        }
    }

    pub const fn balanced(horizontal: f32, vertical: f32) -> Self {
        Self {
            left: horizontal,
            right: horizontal,
            top: vertical,
            bottom: vertical,
        }
    }

    pub const fn horizontal(value: f32) -> Self {
        Self {
            left: value,
            right: value,
            top: 0.0,
            bottom: 0.0,
        }
    }

    pub const fn vertical(value: f32) -> Self {
        Self {
            left: 0.0,
            right: 0.0,
            top: value,
            bottom: value,
        }
    }

    pub fn show<F: FnOnce()>(self, children: F) -> Response<PadWidget> {
        widget_children::<PadWidget, F>(children, self)
    }
}

#[derive(Debug)]
pub struct PadWidget {
    props: Pad,
}

pub type PadResponse = ();

impl Widget for PadWidget {
    type Props = Pad;
    type Response = PadResponse;

    fn new() -> Self {
        Self { props: Pad::ZERO }
    }

    fn update(&mut self, props: Self::Props) -> Self::Response {
        self.props = props;
    }

    fn layout(&self, dom: &Dom, layout: &mut LayoutDom, input: Constraints) -> Vec2 {
        let node = dom.get_current();

        let mut self_size = Vec2::ZERO;

        let total_padding = Vec2::new(
            self.props.left + self.props.right,
            self.props.top + self.props.bottom,
        );
        let offset = Vec2::new(self.props.left, self.props.top);

        let child_constraints = Constraints {
            min: input.min - total_padding,
            max: input.max - total_padding,
        };

        for &child in &node.children {
            self_size = layout.calculate(dom, child, child_constraints) + total_padding;
            layout.set_pos(child, offset);
        }

        input.constrain(self_size)
    }
}