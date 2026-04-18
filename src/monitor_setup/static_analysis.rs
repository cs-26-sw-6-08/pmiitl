use crate::{monitor::streams::OutputStream, monitor_setup::operation_types::Operation};


impl OutputStream {
    pub fn static_analysis(self) -> Self {
        self
        .find_time_funcs_bounds()
    }

    fn find_time_funcs_bounds(mut self) -> Self {
        self.operations = update_bounds(self.operations, 0, self.bound);
        self
    }
}

///Updates bounds for sumtime and avgtime 
fn update_bounds(mut operations: Vec<Operation>, idx: usize, bounds: Option<(i128, i128)>) -> Vec<Operation> {
    let Some(expr_at_idx) = operations.get_mut(idx) else { return operations };

    let (idxs, bounds) = match expr_at_idx {
        Operation::LTLAlwaysUnbounded { idx } | 
        Operation::Unary { idx, .. } | 
        Operation::Foreach { idx } |
        Operation::AggregateFunction { idx, .. } => (vec![*idx], bounds),

        Operation::LTLBounded { bound: (r1, r2), idx, .. } => (vec![*idx], bounds.map(|(a, b)| (*r1+a, *r2+b)).or(Some((*r1, *r2)))),

        Operation::Binary { idx_lhs, idx_rhs, .. } => (vec![*idx_lhs, *idx_rhs], bounds),
        
        Operation::String(_) | 
        Operation::Number(_) | 
        Operation::Member(_) | 
        Operation::CurrentTime => (Vec::new(), bounds),
        
        Operation::TimeFunction { idx, bound, .. } => {
            //Update the bounds for sumtime and avgtime
            //As time is defined by 1s = 1000, it should be divided by 1000
            //What's more is that the bound is contains both a and b, e.g. [0,4] => {0,1,2,3,4}
            //which means that the bound should be increased by 1
            *bound = bounds;

            (vec![*idx], bounds)
        }
    };

    idxs.into_iter().fold(operations, |operations_acc, idx| update_bounds(operations_acc, idx, bounds))
}