use crate::rows::Row;
use std::{
    cell::Cell,
    collections::{HashMap, HashSet},
};

/// A table with a variable number of rows with BITS bits each
#[derive(Debug, Default, Clone)]
pub struct Table {
    rows: HashSet<Row>,
    not_combinable: HashSet<Row>,
}

impl Table {
    pub fn new() -> Self {
        Self::default()
    }

    /// Make one tick (generate the next step table)
    pub fn next_step(&mut self) -> StepResult<'_> {
        let mut todo: HashMap<usize, Vec<(&Row, Cell<bool>)>> = HashMap::new();
        for row in &self.rows {
            todo.entry(row.ones())
                .or_default()
                .push((row, Cell::new(false)));
        }
        let counts = {
            let mut c: Vec<usize> = todo.keys().copied().collect();
            c.sort_unstable();
            c
        };
        let mut new_rows: HashSet<Row> = HashSet::new();
        for count in counts.windows(2) {
            let (now, then) = (count[0], count[1]);
            for (row, checked1) in todo.get(&now).unwrap() {
                for (other, checked2) in todo.get(&then).unwrap() {
                    if let Some(row) = row.combine(other) {
                        checked1.set(true);
                        checked2.set(true);
                        new_rows.insert(row);
                    }
                }
            }
        }
        self.not_combinable.extend(
            todo.into_values()
                .flatten()
                .filter(|(_, checked)| !checked.get())
                .map(|(row, _)| row)
                .cloned(),
        );
        self.rows = new_rows;
        if self.rows.is_empty() {
            StepResult::Done(&self.not_combinable)
        } else {
            StepResult::NotDone
        }
    }

    pub fn insert_row(&mut self, row: Row) {
        if self
            .rows
            .iter()
            .next()
            .is_some_and(|other| row.len() != other.len())
        {
            panic!("The length of the rows have to be equal!");
        }
        self.rows.insert(row);
    }
}

/// The result of a tick (generate next table)
pub enum StepResult<'a> {
    NotDone,
    Done(&'a HashSet<Row>),
}
