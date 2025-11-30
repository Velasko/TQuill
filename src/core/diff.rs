use std::ops::Range;
use std::cmp::{Ordering, min, max};

#[cfg_attr(test, derive(Debug))]
#[derive(PartialEq, Clone)]
pub struct Diff {
    slice: Range<usize>,
    repl: Vec<u8>,
}

impl Diff {
    fn new(slice: Range<usize>, data: &[u8]) -> Self {
        Self {
            slice,
            repl: Vec::from(data),
        }
    }

    fn intersects(&self, other: &Self) -> bool {
        self.slice.contains(&other.slice.start) || self.slice.contains(&other.slice.end)
    }

    fn binary_search(&self, other: &Self) -> Ordering {
        if self.slice.end < other.slice.start {
            Ordering::Greater
        } else if other.slice.end < self.slice.start {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }

    fn union(self, mut other: Self) -> Self {
        let start = min(other.slice.start, self.slice.start);
        let end = max(self.slice.end, other.slice.end);

        let slice_start = self.slice.start.saturating_sub(other.slice.start);
        let slice_end = min(self.slice.end - other.slice.start + 1, other.repl.len());

        let old = other.repl;
        other.repl = Vec::from(&old[..slice_start]);
        other.repl.extend(self.repl);
        other.repl.extend_from_slice(&old[slice_end..]);

        other.slice = start..end;

        other
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diff_mix_n_slices() {
        let a = Diff::new(2..6, b"23456");
        let b = Diff::new(8..13, b"89ABCD");
        let c = Diff::new(15..20, b"-----");
        let d = Diff::new(5..9, b"56789");


        assert_eq!(b.binary_search(&a), Ordering::Less);
        assert_eq!(b.binary_search(&c), Ordering::Greater);
        assert_eq!(b.binary_search(&d), Ordering::Equal);

        let mut diffs = vec![a, b, c];
        let matches = diffs.extract_if(.., |v| d.intersects(v)).collect::<Vec<_>>();
        let union = matches.into_iter().fold(d, |a, b| a.union(b));

        let res = Diff::new(2..13, b"23456789ABCD");
        assert_eq!(union, res);

        let del = Diff::new(1..70, b"");
        assert_eq!(del.clone(), del.union(union));
    }
}
