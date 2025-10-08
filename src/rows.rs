/// The state a position in a row can have
/// We always start with non "DontCare" values...
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum State {
    //  Order is important here to sort the rows later
    DontCare,
    #[default]
    Zero,
    One,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Row(Vec<State>);

impl Row {
    pub fn new(data: Vec<State>) -> Self {
        Self(data)
    }

    pub fn ones(&self) -> usize {
        self.0.iter().filter(|bit| **bit == State::One).count()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Check if two rows can be combined
    fn combinable(&self, other: &Row) -> RowCombine {
        let mut last_difference = None;
        for (i, (this, that)) in (&self.0).into_iter().zip(&other.0).enumerate() {
            match (this, that) {
                (State::DontCare, a) | (a, State::DontCare) if *a != State::DontCare => {
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
    pub fn combine(&self, other: &Row) -> Option<Row> {
        if let RowCombine::Combinable(i) = self.combinable(other) {
            let mut cloned = self.clone();
            cloned.0[i] = State::DontCare;
            Some(cloned)
        } else {
            None
        }
    }

    pub fn get_function_part(&self) -> String {
        self.0
            .iter()
            .enumerate()
            .filter_map(|(i, val)| match val {
                State::One => Some(format!("x{i} ")),
                State::Zero => Some(format!("~x{i} ")),
                State::DontCare => None,
            })
            .collect::<String>()
            .trim_end()
            .to_string()
    }
}

//  Could be Option<usize>
/// The result of a Row::combine()
pub enum RowCombine {
    Combinable(usize),
    NotCombinable,
}
