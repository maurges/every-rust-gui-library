use std::cell::RefCell;
use kas::model::{SharedData, ListData};

#[derive(Debug)]
pub struct SharedCell<T> {
    pub cell: RefCell<T>,
}

impl<T> SharedCell<T> {
    pub fn new(x: T) -> Self {
        SharedCell {
            cell: RefCell::new(x),
        }
    }
}

impl<T: SharedData> SharedData for SharedCell<T> {
    type Key = T::Key;
    type Item = T::Item;
    type ItemRef<'b> = T::ItemRef<'b> where Self: 'b;

    fn version(&self) -> u64 {
        self.cell.borrow().version()
    }

    fn contains_key(&self, key: &Self::Key) -> bool {
        self.cell.borrow().contains_key(key)
    }

    fn borrow(&self, key: &Self::Key) -> Option<Self::ItemRef<'_>> {
        self.cell.borrow().borrow(key)
    }
}

impl<T: ListData> ListData for SharedCell<T> {
    type KeyIter<'b> = T::KeyIter<'b> where Self: 'b;

    fn len(&self) -> usize {
        self.cell.borrow().len()
    }

    fn make_id(&self, parent: &kas::WidgetId, key: &Self::Key) -> kas::WidgetId {
        self.cell.borrow().make_id(parent, key)
    }

    fn reconstruct_key(&self, parent: &kas::WidgetId, child: &kas::WidgetId) -> Option<Self::Key> {
        self.cell.borrow().reconstruct_key(parent, child)
    }

    fn iter_from(&self, start: usize, limit: usize) -> Self::KeyIter<'_> {
        self.cell.borrow().iter_from(start, limit)
    }
}
