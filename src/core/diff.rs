use std::ops::Range;
use std::cmp::{Ordering, min, max};

#[cfg_attr(test, derive(Debug))]
#[derive(PartialEq, Clone)]
pub struct Diff {
    slice: Range<usize>,
    repl: String,
}

impl Diff {
    fn new(slice: Range<usize>, data: &str) -> Self {
        Self {
            slice,
            repl: String::from(data),
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

        other.repl.replace_range(slice_start..slice_end, &self.repl);

        other.slice = start..end;

        other
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diff_mix_n_slices() {
        let a = Diff::new(2..6, "23456");
        let b = Diff::new(8..13, "89ABCD");
        let c = Diff::new(15..20, "-----");
        let d = Diff::new(5..9, "56789");


        assert_eq!(b.binary_search(&a), Ordering::Less);
        assert_eq!(b.binary_search(&c), Ordering::Greater);
        assert_eq!(b.binary_search(&d), Ordering::Equal);

        let mut diffs = vec![a, b, c];
        let matches = diffs.extract_if(.., |v| d.intersects(v)).collect::<Vec<_>>();
        let union = matches.into_iter().fold(d, |a, b| a.union(b));

        let res = Diff::new(2..13, "23456789ABCD");
        assert_eq!(union, res);

        let del = Diff::new(1..70, "");
        assert_eq!(del.clone(), del.union(union));
    }
}
