pub struct S002;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val:  i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self { ListNode { next: None, val } }
}

impl S002 {
    #[allow(dead_code)]
    pub(crate) fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        Self::add_with_carry(l1, l2, 0)
    }

    #[allow(dead_code)]
    pub(crate) fn add_with_carry(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
        carry: i32,
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (Some(l1), Some(l2)) => {
                let sum = l1.val + l2.val + carry;
                let carry = sum % 10;
                if sum < 10 {
                    Some(Box::new(ListNode {
                        val:  sum,
                        next: Self::add_with_carry(l1.next, l2.next, carry),
                    }))
                } else {
                    Some(Box::new(ListNode {
                        val:  0,
                        next: Self::add_with_carry(l1.next, l2.next, 1),
                    }))
                }
            }
            (Some(node), None) | (None, Some(node)) => {
                Some(Box::new(ListNode {
                    val:  node.val + carry,
                    next: Self::add_with_carry(node.next, None, 0),
                }))
            }
            (None, None) => return None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works() {
        let solution = S002::add_two_numbers(
            Some(Box::new(ListNode {
                val:  2,
                next: Some(Box::new(ListNode {
                    val:  4,
                    next: Some(Box::new(ListNode {
                        val:  3,
                        next: None,
                    })),
                })),
            })),
            Some(Box::new(ListNode {
                val:  5,
                next: Some(Box::new(ListNode {
                    val:  6,
                    next: Some(Box::new(ListNode {
                        val:  4,
                        next: None,
                    })),
                })),
            })),
        );
        assert_eq!(
            solution,
            Some(Box::new(ListNode {
                val:  7,
                next: Some(Box::new(ListNode {
                    val:  0,
                    next: Some(Box::new(ListNode {
                        val:  8,
                        next: None,
                    })),
                })),
            }))
        );
    }
}
