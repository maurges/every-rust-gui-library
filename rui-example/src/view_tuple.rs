pub struct ViewVec<V> {
    vec: Vec<V>,
}

impl<V> ViewVec<V> {
    pub fn new(vec: Vec<V>) -> Self {
        ViewVec { vec }
    }
}

impl<V> rui::ViewTuple for ViewVec<V>
where
    V: rui::View,
{
    fn foreach_view<F: FnMut(&dyn rui::View)>(&self, f: &mut F) {
        for v in &self.vec {
            f(v)
        }
    }

    fn len(&self) -> usize {
        self.vec.len()
    }
}
