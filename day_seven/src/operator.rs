#[derive(Clone, Debug)]
pub enum Operator {
    Sum,
    Multiply,
    Concat,
}

impl Operator {
    fn next(&self) -> Option<Self> {
        match self {
            Operator::Sum => Some(Operator::Multiply),
            Operator::Multiply => Some(Operator::Concat),
            Operator::Concat => None,
        }
    }

    fn start() -> Self {
        Self::Sum
    }
}

#[derive(Clone)]
pub struct OperatorList {
    operators: Vec<Operator>,
}

impl OperatorList {
    pub fn at(&self, index: usize) -> Operator {
        if index < self.operators.len() {
            self.operators[index].clone()
        } else {
            Operator::start()
        }
    }

    pub fn new() -> OperatorList {
        Self {
            operators: vec![Operator::start()],
        }
    }
}

impl Iterator for OperatorList {
    type Item = OperatorList;

    fn next(&mut self) -> Option<Self::Item> {
        for operator in &mut self.operators {
            if let Some(next_operator) = operator.next() {
                *operator = next_operator;
                return Some(self.clone());
            } else {
                *operator = Operator::start();
            }
        }

        // We start a new digit, making sure it's not in 'start' state anymore.
        self.operators.push(Operator::Multiply);
        Some(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operator_list_iteration() {
        let mut operator_list = OperatorList::new();

        // First iteration
        assert_eq!(operator_list.operators.len(), 1);
        assert!(matches!(operator_list.operators[0], Operator::Sum));

        // Second iteration
        let next = operator_list.next().unwrap();
        assert_eq!(next.operators.len(), 1);
        assert!(matches!(next.operators[0], Operator::Multiply));

        // Third iteration
        let next = operator_list.next().unwrap();
        assert_eq!(next.operators.len(), 1);
        assert!(matches!(next.operators[0], Operator::Concat));

        // Fourth iteration (wraps around and adds new item)
        let next = operator_list.next().unwrap();
        assert_eq!(next.operators.len(), 2);
        assert!(matches!(next.operators[0], Operator::Sum));
        assert!(matches!(next.operators[1], Operator::Sum));

        // Fifth iteration
        let next = operator_list.next().unwrap();
        assert_eq!(next.operators.len(), 2);
        assert!(matches!(next.operators[0], Operator::Multiply));
        assert!(matches!(next.operators[1], Operator::Sum));

        // Sixth iteration
        let next = operator_list.next().unwrap();
        assert_eq!(next.operators.len(), 2);
        assert!(matches!(next.operators[0], Operator::Concat));
        assert!(matches!(next.operators[1], Operator::Sum));

        // Seventh iteration
        let next = operator_list.next().unwrap();
        assert_eq!(next.operators.len(), 2);
        assert!(matches!(next.operators[0], Operator::Sum));
        assert!(matches!(next.operators[1], Operator::Multiply));

        // Eighth iteration
        let next = operator_list.next().unwrap();
        assert_eq!(next.operators.len(), 2);
        assert!(matches!(next.operators[0], Operator::Multiply));
        assert!(matches!(next.operators[1], Operator::Multiply));

        // Ninth iteration
        let next = operator_list.next().unwrap();
        assert_eq!(next.operators.len(), 2);
        assert!(matches!(next.operators[0], Operator::Concat));
        assert!(matches!(next.operators[1], Operator::Multiply));

        // Tenth iteration
        let next = operator_list.next().unwrap();
        assert_eq!(next.operators.len(), 2);
        assert!(matches!(next.operators[0], Operator::Sum));
        assert!(matches!(next.operators[1], Operator::Concat));

        // Tenth iteration
        let next = operator_list.next().unwrap();
        assert_eq!(next.operators.len(), 2);
        assert!(matches!(next.operators[0], Operator::Multiply));
        assert!(matches!(next.operators[1], Operator::Concat));
        // Tenth iteration
        let next = operator_list.next().unwrap();
        assert_eq!(next.operators.len(), 2);
        assert!(matches!(next.operators[0], Operator::Concat));
        assert!(matches!(next.operators[1], Operator::Concat));
        // Tenth iteration
        let next = operator_list.next().unwrap();
        assert_eq!(next.operators.len(), 3);
        assert!(matches!(next.operators[0], Operator::Sum));
        assert!(matches!(next.operators[1], Operator::Sum));
        println!("tester {:?}", next.operators[2]);
        assert!(matches!(next.operators[2], Operator::Multiply));
    }
}
