use itertools::Itertools;
use std::cell::{Ref, RefCell, RefMut};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub struct Warehouse {
    stacks: Vec<RefCell<Stack>>,
}

#[derive(Copy, Clone)]
pub enum MoverType {
    MoveSingle,
    MoveMultiple,
}

type Stack = Vec<Crate>;

#[derive(Copy, Clone)]
struct Crate {
    payload: char,
}

impl Warehouse {
    fn iter_stack(&self) -> impl Iterator<Item = Ref<'_, Stack>> {
        self.stacks.iter().map(|stack| stack.borrow())
    }

    fn at(&self, idx: usize) -> Ref<Stack> {
        self.stacks[idx].borrow()
    }

    fn at_mut(&self, idx: usize) -> RefMut<Stack> {
        self.stacks[idx].borrow_mut()
    }

    fn longest_stack_length(&self) -> usize {
        self.iter_stack().map(|stack| stack.len()).max().unwrap()
    }

    pub fn top_of_stacks(&self) -> String {
        self.iter_stack()
            .map(|stack| stack.last().map_or(' ', |c| c.payload))
            .collect()
    }

    pub fn move_crates_between_stacks(
        &self,
        n: usize,
        from: usize,
        to: usize,
        mover_type: MoverType,
    ) {
        let start_drain = self.at(from).len() - n;
        let mut from_stack = self.at_mut(from);
        let crates_to_move = from_stack.drain(start_drain..);
        match mover_type {
            MoverType::MoveSingle => self.at_mut(to).extend(crates_to_move.rev()),
            MoverType::MoveMultiple => self.at_mut(to).extend(crates_to_move),
        };
    }
}

impl Display for Warehouse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let height = self.longest_stack_length();

        let stacks_content = (0..height)
            .rev()
            .map(|idx| {
                self.iter_stack()
                    .map(|stack| {
                        if idx >= stack.len() {
                            format!("{: <3}", "")
                        } else {
                            stack[idx].to_string()
                        }
                    })
                    .join(" ")
            })
            .join("\n");
        let stack_ids = (1..self.stacks.len() + 1)
            .map(|idx| format!("{:^3}", idx))
            .join(" ");

        writeln!(f, "{}", stacks_content)?;
        writeln!(f, "{}", stack_ids)
    }
}

impl Display for Crate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.payload)
    }
}

#[derive(Debug)]
pub struct ParseWarehouseError {
    msg: String,
}

fn parse_crate_str(crate_str: &str) -> Result<Option<Crate>, ParseWarehouseError> {
    if let Some(('[', payload, ']')) = crate_str.chars().collect_tuple() {
        Ok(Some(Crate { payload }))
    } else if crate_str.chars().all(|c| c.is_whitespace()) {
        Ok(None)
    } else {
        Err(ParseWarehouseError {
            msg: format!(
                "Wrong format for crate_str: {}. It must be [<char>]",
                crate_str
            ),
        })
    }
}

impl FromStr for Warehouse {
    type Err = ParseWarehouseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().rev();

        let nb_stacks = lines.next().unwrap().trim().split("  ").count();
        let empty_stack = RefCell::new(vec![]);
        let warehouse = Warehouse {
            stacks: vec![empty_stack; nb_stacks],
        };

        for line in lines {
            let mut boundary = 0;
            for idx in 0..nb_stacks {
                if let Ok(Some(crate_payload)) = parse_crate_str(&line[boundary..boundary + 3]) {
                    warehouse.at_mut(idx).push(crate_payload);
                }
                boundary = boundary + 4;
            }
        }

        Ok(warehouse)
    }
}
