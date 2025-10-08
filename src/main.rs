use crate::{
    rows::{Row, State},
    table::{StepResult, Table},
};
/// Warning: this calculates the smaller possible function, not the smallest possible overall...
/// You can take the result of this program and minimize it further.
/// Also: with many bits (> 8) the calculation takes multiple seconds. It's not very optimized (yet).
use std::time::Instant;

mod rows;
mod table;

fn main() {
    //  See doc of make_table() for explanation
    let mut table: Table = make_table(&[&[0, 1, 0], &[0, 1, 1], &[1, 1, 0]]);

    let start = Instant::now();
    let rows = loop {
        if let StepResult::Done(rows) = table.next_step() {
            break rows;
        }
    };

    let end = start.elapsed();
    println!("Done:");

    print!("y =  ");
    for (i, row) in rows.into_iter().enumerate() {
        if i != 0 {
            print!("  +  ");
        }
        print!("{}", row.get_function_part());
    }
    println!();
    println!("Took: {end:?}");
}
/// Convenience function \
/// \
/// x2 x1 x0  y \
///  0  0  0  0 \
///  0  0  1  0 \
///  0  1  0  1 \
///  0  1  1  1 \
///  1  0  0  0 \
///  1  0  1  0 \
///  1  1  0  1 \
///  1  1  1  0 \
/// is represented as
/// ```rust
/// //   x2 x1 x0      y
/// &[
///   &[0, 1, 0], // 1
///   &[0, 1, 1], // 1
///   &[1, 1, 0], // 1
/// ]
/// ```
fn make_table(data: &[&[u8]]) -> Table {
    let mut table = Table::new();
    for row in data {
        table.insert_row(Row::new(
            row.iter()
                .map(|val| match val {
                    0 => State::Zero,
                    1 => State::One,
                    _ => panic!("Only 0 and 1 is allowed"),
                })
                .collect(),
        ));
    }
    table
}
