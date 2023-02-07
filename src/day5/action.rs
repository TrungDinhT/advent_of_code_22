use anyhow::anyhow;
use itertools::Itertools;
use std::str::FromStr;

pub struct Action {
    pub n_moved: usize,
    pub from: usize,
    pub to: usize,
}

impl FromStr for Action {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> anyhow::Result<Self> {
        if let Some(("move", n_crates_str, "from", from_stack_id_str, "to", to_stack_id_str)) =
            s.split_whitespace().collect_tuple()
        {
            let n_crates: usize = n_crates_str.parse()?;
            let from_stack_id: usize = from_stack_id_str.parse()?;
            let to_stack_id: usize = to_stack_id_str.parse()?;
            Ok(Action {
                n_moved: n_crates,
                from: from_stack_id - 1,
                to: to_stack_id - 1,
            })
        } else {
            Err(anyhow!(
                "action string ({}) is not under the right format: \
                \"move <n_crates> from <from_stack_id> to <to_stack_id>\"",
                s
            ))
        }
    }
}
