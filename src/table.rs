use crate::rows::Row;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};

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
        //  The map of rows sorted by ones
        let mut sorted: HashMap<usize, Vec<&Row>> = HashMap::new();
        for row in &self.rows {
            sorted.entry(row.ones()).or_default().push(row);
        }
        //  A list of all available counts of ones to iterate over
        let counts = {
            let mut c: Vec<&usize> = sorted.keys().collect();
            c.sort_unstable();
            c
        };
        let (rows, used) = counts
            .windows(2)
            .flat_map(|indices| {
                sorted
                    .get(indices[0])
                    .unwrap()
                    .into_par_iter()
                    .flat_map(|&row1| {
                        sorted
                            .get(indices[1])
                            .unwrap()
                            .into_iter()
                            .filter_map(|&row2| row1.combine(row2).map(|row| (row, [row1, row2])))
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>()
            })
            .fold(
                (HashSet::new(), HashSet::new()),
                |(mut rows, mut used), (row, pair)| {
                    rows.insert(row);
                    used.extend(pair);
                    (rows, used)
                },
            );

        self.not_combinable.extend(used.into_iter().cloned());
        self.rows = rows;

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
