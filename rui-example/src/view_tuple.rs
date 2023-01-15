pub struct ViewSlice<'a, V> {
    slice: &'a [V],
}

impl<'a, V> ViewSlice<'a, V> {
    pub fn new(slice: &'a [V]) -> Self {
        ViewSlice { slice }
    }
}

impl<'a, V> rui::ViewTuple for ViewSlice<'a, V>
where
    V: rui::View,
{
    fn foreach_view<F: FnMut(&dyn rui::View)>(&self, f: &mut F) {
        for v in self.slice {
            f(v)
        }
    }

    fn len(&self) -> usize {
        self.slice.len()
    }
}
