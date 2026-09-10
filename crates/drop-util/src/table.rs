use std::{
    marker::PhantomData,
    ops::{Index, IndexMut},
};

#[derive(Debug)]
pub struct TableId<T> {
    id: usize,
    __phantom_data: PhantomData<T>,
}

impl<T> TableId<T> {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            __phantom_data: PhantomData,
        }
    }

    pub fn get(&self) -> usize {
        self.id
    }
}

impl<T> Clone for TableId<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for TableId<T> {}

impl<T> PartialEq for TableId<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Debug)]
pub struct Table<T> {
    items: Vec<T>,
}

impl<T> Table<T> {
    pub fn new() -> Self {
        Table { items: vec![] }
    }

    pub fn insert(&mut self, x: T) -> TableId<T> {
        let id = self.items.len();
        self.items.push(x);
        TableId::new(id)
    }

    pub fn all(&self) -> Vec<TableId<T>> {
        (0..self.items.len()).map(|x| TableId::new(x)).collect()
    }
}

impl<T> Default for Table<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Index<TableId<T>> for Table<T> {
    type Output = T;

    fn index(&self, index: TableId<T>) -> &Self::Output {
        &self.items[index.get()]
    }
}

impl<T> IndexMut<TableId<T>> for Table<T> {
    fn index_mut(&mut self, index: TableId<T>) -> &mut Self::Output {
        &mut self.items[index.get()]
    }
}
