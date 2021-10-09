// Naive set theory implementation.

fn intersection(a: Vec<u32>, b: Vec<u32>) -> Vec<u32> {
    let mut result = Vec::new();
    for x in a {
        if b.contains(&x) {
            result.push(x);
        }
    }
    result.sort();
    result
}

fn union_full(a: Vec<u32>, b: Vec<u32>) -> Vec<u32> {
  let mut c = a.into_iter().
    chain(b.into_iter()).collect::<Vec<u32>>();
    c.sort();
    c
}

fn union_unique(a: Vec<u32>, b: Vec<u32>) -> Vec<u32> {
  let mut c = a.into_iter().
    chain(b.into_iter()).collect::<Vec<u32>>();
    c.sort();
    c.dedup();
    c
   
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersection() {
        let a = vec![1, 2, 3, 4];
        let b = vec![2, 4, 6, 8];
        assert_eq!(intersection(a, b), vec![2, 4]);
    }

    #[test]
    fn test_unio_dedup() {
        let a = vec![1, 2, 3, 4];
        let b = vec![2, 4, 6, 8];
        assert_eq!(union_unique(a, b), vec![1, 2, 3, 4, 6, 8]);
    }

    #[test]
    fn test_union_full() {
        let a = vec![1, 2, 3, 4];
        let b = vec![2, 4, 6, 8];
        assert_eq!(union_full(a, b), vec![1, 2,2, 3, 4,4, 6, 8]);
    }
}