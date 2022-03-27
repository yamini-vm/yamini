use yamini::memory::Stack;

#[test]
fn test_stack_push() {
    let mut stack = Stack::new();
    stack.push(3);
    stack.push(4);

    assert_eq!(stack.data(), &[3, 4]);
    assert_eq!(stack.head(), 2);
}

#[test]
fn test_stack_pop() {
    let mut stack = Stack::new();
    stack.push(3);
    stack.push(4);

    assert_eq!(stack.pop(), Some(4));
    assert_eq!(stack.pop(), Some(3));
    assert_eq!(stack.pop(), None);
}