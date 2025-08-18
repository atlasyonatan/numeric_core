pub trait ContiguousPartitionExt {
    type Item;

    fn contiguous_partitions(&self, n: usize) -> Option<ContiguousPartitions<'_, Self::Item>>;
}

impl<T> ContiguousPartitionExt for [T] {
    type Item = T;

    fn contiguous_partitions(&self, n: usize) -> Option<ContiguousPartitions<'_, T>> {
        ContiguousPartitions::new(self, n)
    }
}

pub struct ContiguousPartitions<'a, T> {
    items: &'a [T],
    n: usize,
    split_indices: Vec<usize>,
    finished: bool,
}

impl<'a, T> ContiguousPartitions<'a, T> {
    pub fn new(items: &'a [T], n: usize) -> Option<Self> {
        let len = items.len();
        if n == 0 || n > len {
            return None;
        }

        // initial split indices for n > 1
        let split_indices = if n == 1 { vec![] } else { (1..n).collect() };

        Some(Self {
            items,
            n,
            split_indices,
            finished: false,
        })
    }

    fn build_partition(&self) -> Vec<&'a [T]> {
        if self.n == 1 {
            return vec![self.items];
        }

        let mut result = Vec::with_capacity(self.n);
        let mut start = 0;
        for &split in &self.split_indices {
            result.push(&self.items[start..split]);
            start = split;
        }
        result.push(&self.items[start..]);
        result
    }

    fn advance(&mut self) -> bool {
        if self.n <= 1 {
            return false;
        }

        let len = self.items.len();

        for i in (0..self.split_indices.len()).rev() {
            let max = if i + 1 < self.split_indices.len() {
                self.split_indices[i + 1] - 1
            } else {
                len - 1
            };

            if self.split_indices[i] < max {
                self.split_indices[i] += 1;
                for j in i + 1..self.split_indices.len() {
                    self.split_indices[j] = self.split_indices[j - 1] + 1;
                }
                return true;
            }
        }

        false
    }
}

impl<'a, T> Iterator for ContiguousPartitions<'a, T> {
    type Item = Vec<&'a [T]>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        let partition = self.build_partition();

        if !self.advance() {
            self.finished = true;
        }

        Some(partition)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_n_zero() {
        let digits = [1, 2, 3, 4, 5];
        assert!(digits.contiguous_partitions(0).is_none());
    }

    #[test]
    fn test_n_greater_than_len() {
        let digits = [1, 2, 3, 4, 5];
        assert!(digits.contiguous_partitions(6).is_none());
    }

    #[test]
    fn test_n_one() {
        let digits = [1, 2, 3, 4, 5];
        let partitions: Vec<Vec<Vec<i32>>> = digits
            .contiguous_partitions(1)
            .unwrap()
            .map(|p| p.into_iter().map(|s| s.to_vec()).collect())
            .collect();
        assert_eq!(partitions, vec![vec![digits.to_vec()]]);
    }

    #[test]
    fn test_n_len() {
        let digits = [1, 2, 3, 4, 5];
        let partitions: Vec<Vec<Vec<i32>>> = digits
            .contiguous_partitions(5)
            .unwrap()
            .map(|p| p.into_iter().map(|s| s.to_vec()).collect())
            .collect();
        let expected: Vec<Vec<Vec<i32>>> = vec![digits.iter().map(|x| vec![*x]).collect()];
        assert_eq!(partitions, expected);
    }

    #[test]
    fn test_n_three() {
        let digits = [1, 2, 3, 4, 5];
        let partitions: Vec<Vec<Vec<i32>>> = digits
            .contiguous_partitions(3)
            .unwrap()
            .map(|p| p.into_iter().map(|s| s.to_vec()).collect())
            .collect();

        let expected = vec![
            vec![vec![1], vec![2], vec![3, 4, 5]],
            vec![vec![1], vec![2, 3], vec![4, 5]],
            vec![vec![1], vec![2, 3, 4], vec![5]],
            vec![vec![1, 2], vec![3], vec![4, 5]],
            vec![vec![1, 2], vec![3, 4], vec![5]],
            vec![vec![1, 2, 3], vec![4], vec![5]],
        ];

        assert_eq!(partitions, expected);
    }
}
