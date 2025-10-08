/// Warning: this calculates the smaller possible function, not the smallest possible overall...
/// You can take the result of this program and minimize it further.
/// Also: with many bits (> 8) the calculation takes multiple seconds. It's not very optimized (yet).
use std::{
    collections::{HashMap, HashSet},
    fmt::{Debug, Display},
    ops::{Index, IndexMut},
    time::Instant,
};

fn main() {
    /*
     * This represents the truth table:
     * x3 x2 x1 x0  y
     *  0  0  0  0  0
     *  0  0  0  1  0
     *  0  0  1  0  1
     *  0  0  1  1  1
     *  0  1  0  0  0
     *  0  1  0  1  0
     *  0  1  1  0  1
     *  0  1  1  1  1
     *  1  0  0  0  0
     *  1  0  0  1  0
     *  1  0  1  0  1
     *  1  0  1  1  1
     *  1  1  0  0  0
     *  1  1  0  1  0
     *  1  1  1  0  1
     *  1  1  1  1  0
     * TABLE contains the column of y in ascending order
     */
    const TABLE: &[u8] = &[0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 0];
    const BITS: usize = TABLE.len().ilog2() as usize;
    assert!(2usize.pow(BITS as u32) == TABLE.len());

    let mut table = make_table::<BITS>(TABLE);

    println!("Start:");
    println!("{table}");
    println!();

    let start = Instant::now();
    let rows = loop {
        match table.tick() {
            TickResult::Done(rows) => {
                break rows;
            }
            TickResult::NotDone(table_) => table = table_,
        }
    };

    let end = start.elapsed();
    println!("Done:");
    for row in &rows {
        println!("{row}");
    }

    print!("y = ");
    for (i, row) in rows.iter().enumerate() {
        if i != 0 {
            print!(" +  ");
        }
        for (index, state) in row.data.iter().enumerate() {
            match state {
                State::One => print!("x{index} "),
                State::Zero => print!("Nx{index} "),
                State::DontCare => {}
            }
        }
    }
    println!();
    println!("Took: {end:?}");
}

/// Create the start table
fn make_table<const BITS: usize>(list: &[u8]) -> Table<BITS> {
    let mut table = Table::<BITS> {
        data: HashMap::new(),
        not_conbinable: HashSet::new(),
    };
    for i in 0..list.len() {
        if list[i] == 1 {
            let row = Row::<BITS>::from(i);
            table.data.insert(row, false);
        }
    }
    table
}

/// A row with BITS bits in size
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Row<const BITS: usize> {
    data: [State; BITS],
}

/// A table with a variable number of rows with BITS bits each
#[derive(Debug, Clone)]
struct Table<const BITS: usize> {
    data: HashMap<Row<BITS>, bool>,
    not_conbinable: HashSet<Row<BITS>>,
}

impl<const BITS: usize> Table<BITS> {
    /// Make one tick (generate the next step table)
    pub fn tick(mut self) -> TickResult<BITS> {
        let mut table = self.clone();
        table.data.clear();
        let rows = self.data.clone();
        for (row, checked) in &mut self.data {
            for other in &rows {
                if let Some(new) = row.combine(other.0) {
                    *checked = true;
                    table.data.insert(new, false);
                }
            }
        }
        for (row, checked) in &self.data {
            if !*checked {
                table.not_conbinable.insert(row.clone());
            }
        }
        let done = self
            .data
            .iter()
            .all(|(row, _)| table.data.contains_key(row));
        if done {
            TickResult::Done(
                table
                    .data
                    .iter()
                    .map(|(row, _)| row)
                    .chain(&table.not_conbinable)
                    .cloned()
                    .collect(),
            )
        } else {
            TickResult::NotDone(table)
        }
    }
}

impl<const BITS: usize> Default for Row<BITS> {
    fn default() -> Self {
        Self {
            data: [State::default(); BITS],
        }
    }
}

impl<const BITS: usize> Index<usize> for Row<BITS> {
    type Output = State;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<const BITS: usize> IndexMut<usize> for Row<BITS> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<const BITS: usize> Row<BITS> {
    /// Check if two rows can be combined
    fn combinable(&self, other: &Row<BITS>) -> RowCombine {
        if self == other {
            //  Pathetic
            return RowCombine::NotCombinable;
        }
        let mut last_difference = None;
        for (i, (this, that)) in self.data.into_iter().zip(other.data).enumerate() {
            match (this, that) {
                (State::DontCare, a) | (a, State::DontCare) if a != State::DontCare => {
                    return RowCombine::NotCombinable;
                }
                (State::One, State::Zero) | (State::Zero, State::One) => {
                    if last_difference.is_some() {
                        return RowCombine::NotCombinable;
                    } else {
                        last_difference = Some(i);
                    }
                }
                _ => {}
            }
        }
        if let Some(i) = last_difference {
            RowCombine::Combinable(i)
        } else {
            RowCombine::NotCombinable
        }
    }

    /// Try to combine two rows
    pub fn combine(&self, other: &Row<BITS>) -> Option<Row<BITS>> {
        if let RowCombine::Combinable(i) = self.combinable(other) {
            let mut cloned = self.clone();
            cloned[i] = State::DontCare;
            Some(cloned)
        } else {
            None
        }
    }
}

impl<const BITS: usize> From<usize> for Row<BITS> {
    /// Generate a row from the bit representation of a number
    fn from(value: usize) -> Self {
        let mut row = Row::<BITS>::default();
        for i in 0..BITS {
            //  We need to switch the bitorder later!
            row[i] = match value >> i & 1 {
                0 => State::Zero,
                1 => State::One,
                _ => unreachable!(),
            };
        }
        row
    }
}

/// The state a position in a row can have
/// We always start with non "DontCare" values...
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
enum State {
    #[default]
    Zero,
    One,
    DontCare,
}

//  Could be Option<usize>
/// The result of a Row::combine()
enum RowCombine {
    Combinable(usize),
    NotCombinable,
}

//  Pretty printing :)
impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Zero => "0",
            Self::One => "1",
            Self::DontCare => "*",
        })
    }
}

impl<const BITS: usize> Display for Row<BITS> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        //  We reverse rows so we get a LowEndian list (which is used on paper...)
        for (i, value) in self.data.iter().rev().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{value}")?;
        }
        write!(f, "]")
    }
}

impl<const BITS: usize> Display for Table<BITS> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Table [")?;
        writeln!(f, "  // Entries")?;
        for (row, _) in &self.data {
            writeln!(f, "  {row},")?;
        }
        f.write_str("]")
    }
}

/// The result of a tick (generate next table)
enum TickResult<const BITS: usize> {
    NotDone(Table<BITS>),
    Done(Vec<Row<BITS>>),
}
