use super::super::action::Action;

const ACTION_STR: &str = "move 1 from 2 to 3";

#[test]
fn test_parse_action() {
    let action: Action = ACTION_STR.parse().unwrap();
    assert_eq!(action.n_moved, 1);
    assert_eq!(action.from, 1);
    assert_eq!(action.to, 2);
}
