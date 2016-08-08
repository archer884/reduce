#[derive(Debug)]
pub struct DivisorIterator {
    value: i32,
    last: Option<i32>
}

impl DivisorIterator {
    pub fn new(n: i32) -> DivisorIterator {
        DivisorIterator {
            value: n,
            last: Some(1),
        }
    }
}

impl Iterator for DivisorIterator {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        match self.last {
            None => None,
            Some(last) => {
                let ret = last;
                self.last = InclusiveRange::new(last + 1, self.value).filter(|n| self.value % n == 0).nth(0);
                Some(ret)
            }
        }
    }
}

// Because fuck my life
struct InclusiveRange {
    current: i32,
    max: i32,
}

impl InclusiveRange {
    fn new(current: i32, max: i32) -> InclusiveRange {
        InclusiveRange {
            current: current,
            max: max,
        }
    }
}

impl Iterator for InclusiveRange {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current > self.max {
            return None;
        }

        let ret = self.current;
        self.current += 1;

        Some(ret)
    }
}

#[cfg(test)]
mod tests {
    use divisors::DivisorIterator;

    #[test]
    fn divisors_of_4096() {
        let divisors: Vec<_> = DivisorIterator::new(4096).collect();

        assert_eq!(&[1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096], &divisors[..])
    }
}
