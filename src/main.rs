/// Warning: this calculates the smaller possible function, not the smallest possible overall...
/// You can take the result of this program and minimize it further.
/// Also: with many bits the calculation takes multiple seconds.
///       16bit: 50s with release build on my macbook m2
use crate::{
    rows::{Row, State},
    table::{StepResult, Table},
};
use rand::random;
use std::time::Instant;

mod rows;
mod table;

fn main() {
    //  See doc of make_table() for explanation
    //let mut table: Table = make_table(&[&[0, 1, 0], &[0, 1, 1], &[1, 1, 0]]);
    let mut table = make_random_table(16);

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
///  0  1  0  1 Needed \
///  0  1  1  1 Needed \
///  1  0  0  0 \
///  1  0  1  0 \
///  1  1  0  1 Needed \
///  1  1  1  0 \
/// is represented as
/// ```rust
/// &[
///   &[0, 1, 0], // ~x2  x1 ~x0
///   &[0, 1, 1], // ~x2  x1  x0
///   &[1, 1, 0], //  x2  x1 ~x0
/// ]
/// ```
#[allow(dead_code)]
fn make_table(data: &[&[u8]]) -> Table {
    let mut table = Table::new();
    for row in data {
        table.insert_row(Row::new(
            row.iter()
                //  We reverse here so that row[0] is x0 and not row[2] is x0...
                //  The human representation inside the table is switched around in byte order
                .rev()
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

#[allow(dead_code)]
fn make_random_table(bits: u32) -> Table {
    let size = 2u32.pow(bits);
    let size = rand::random::<u32>() % size;
    let mut table = Table::new();
    (0..size)
        .map(|_| {
            Row::new(
                (0..bits)
                    .map(|_| match random() {
                        true => State::One,
                        false => State::Zero,
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .for_each(|row| table.insert_row(row));
    table
}
