use fltk::group::Group;
use fltk::prelude::{WidgetBase, WidgetExt, GroupExt};

pub struct ColumnLayout {
    parent: Group,
}

fltk::widget_extends!(ColumnLayout, Group, parent);

impl Default for ColumnLayout {
    fn default() -> Self {
        ColumnLayout::new(0, 0, 1, 1)
    }
}

impl ColumnLayout {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        let r = ColumnLayout {
            parent: Group::new(x, y, w, h, None),
        };
        r
    }

    pub fn add<T, P>(&mut self, x: &mut T)
    where
        T: core::ops::DerefMut<Target = P>,
        P: WidgetExt,
    {
        eprintln!("original size: {} x {}", self.width(), self.height());
        x.set_pos(self.x(), self.y() + self.height());
        let new_width = core::cmp::max(self.width(), x.width());
        let new_height = self.height() + x.height();
        eprintln!("resizing to {}, {}", new_width, new_height);
        self.parent.resize(
            self.x(),
            self.y(),
            new_width,
            new_height,
        );
        eprintln!("after resize: {} x {}", self.width(), self.height());
        self.parent.add(x.deref());
        eprintln!("after adding: {} x {}", self.width(), self.height());
        self.redraw();
    }
}
