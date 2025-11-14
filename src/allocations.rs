use num::Num;
use std::ops::AddAssign;
use std::ops::SubAssign;
/*
Say you have 100 cookies that need to get allocated to 5 people. That can be done as
    [100, 0, 0, 0, 0]
    [5, 21, 15, 38, 21]
    [0, 50, 50, 0, 0]
    ...
This iterator goes through all possible allocations starting with [total, 0, ..., 0]
and ending with [0, 0, ..., total]
*/
#[allow(dead_code)]
pub struct AllocationsIterator<T>
where
    T: Num,
{
    n: usize,
    total: T,
    current: Vec<T>,
    done: bool,
}

impl<T> AllocationsIterator<T>
where
    T: Num + Copy + Clone + PartialOrd + AddAssign + SubAssign,
{
    fn new(n: usize, total: T) -> Self {
        let mut current = vec![T::zero(); n];
        current[0] = total;
        Self {
            n,
            total,
            current,
            done: n == 0,
        }
    }

    fn increment(&mut self) {
        if self.done {
            return;
        }

        for index in 1..self.n {
            if self.current[0] > T::zero() {
                self.current[0] -= T::one();
                self.current[index] += T::one();
                return;
            } else {
                let v = self.current[index];
                self.current[0] += v;
                self.current[index] = T::zero();
            }
        }
        self.done = true;
    }
}

impl<T> Iterator for AllocationsIterator<T>
where
    T: Num + Copy + Clone + PartialOrd + AddAssign + SubAssign,
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        let current_tuple = self.current.clone();
        self.increment();
        Some(current_tuple)
    }
}

pub fn allocations<T>(n: usize, total: T) -> AllocationsIterator<T>
where
    T: Num + Copy + Clone + PartialOrd + AddAssign + SubAssign,
{
    AllocationsIterator::new(n, total)
}
