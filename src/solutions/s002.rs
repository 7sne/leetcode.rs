pub struct S002;

impl S002 {
    #[allow(dead_code)]
    pub(crate) fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works() {
        let solution = S002::add_two_numbers(
            Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: None,
                    })),
                })),
            })),
            Some(Box::new(ListNode {
                val: 5,
                next: Some(Box::new(ListNode {
                    val: 6,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: None,
                    })),
                })),
            })),
        );
    }
}
