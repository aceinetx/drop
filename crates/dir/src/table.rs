use std::{
    marker::PhantomData,
    ops::{Index, IndexMut},
};

#[derive(Debug)]
pub struct TableId<T> {
    id: usize,
    phantom_data: PhantomData<T>,
}

impl<T> TableId<T> {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            phantom_data: PhantomData::default(),
        }
    }

    pub fn get(&self) -> usize {
        self.id
    }
}

impl<T> Clone for TableId<T> {
    fn clone(&self) -> Self {
        Self::new(self.get())
    }
}

impl<T> Copy for TableId<T> {}

#[derive(Debug)]
pub struct Table<T> {
    instructions: Vec<T>,
}

impl<T> Table<T> {
    pub fn new() -> Self {
        Table {
            instructions: vec![],
        }
    }

    pub fn insert(&mut self, x: T) -> TableId<T> {
        let id = self.instructions.len();
        self.instructions.push(x);
        TableId::new(id)
    }

    pub fn all(&self) -> Vec<TableId<T>> {
        (0..self.instructions.len())
            .map(|x| TableId::new(x))
            .collect()
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
        &self.instructions[index.get()]
    }
}

impl<T> IndexMut<TableId<T>> for Table<T> {
    fn index_mut(&mut self, index: TableId<T>) -> &mut Self::Output {
        &mut self.instructions[index.get()]
    }
}
