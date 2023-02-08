use super::super::warehouse::Warehouse;
use crate::day5::warehouse::MoverType::{MoveMultiple, MoveSingle};

const WAREHOUSE_STR: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 
";

#[test]
fn test_warehouse_from_str() {
    let warehouse = WAREHOUSE_STR.parse::<Warehouse>().unwrap();
    assert_eq!(WAREHOUSE_STR, warehouse.to_string());
}

#[test]
fn test_top_of_stacks() {
    let warehouse = WAREHOUSE_STR.parse::<Warehouse>().unwrap();
    assert_eq!("NDP", warehouse.top_of_stacks());
}

#[test]
fn test_move_crates_to_other_one_at_a_time() {
    let ref_warehouse_after_move_str: &str = "    [D] [Z]
    [C] [N]
    [M] [P]
 1   2   3 
";

    let warehouse = WAREHOUSE_STR.parse::<Warehouse>().unwrap();
    warehouse.move_crates_between_stacks(2, 0, 2, MoveSingle);

    assert_eq!(ref_warehouse_after_move_str, warehouse.to_string());
}

#[test]
fn test_move_crates_to_other_multiple_at_a_time() {
    let ref_warehouse_after_move_str: &str = "    [D] [N]
    [C] [Z]
    [M] [P]
 1   2   3 
";

    let warehouse = WAREHOUSE_STR.parse::<Warehouse>().unwrap();
    warehouse.move_crates_between_stacks(2, 0, 2, MoveMultiple);

    assert_eq!(ref_warehouse_after_move_str, warehouse.to_string());
}
