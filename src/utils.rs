// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Utility functions

pub fn eigenvalues_2x2(mat: &ndarray::ArrayView2<f64>) -> (f64, f64) {
    let a = mat[(0, 0)];
    let b = mat[(0, 1)];
    let c = mat[(1, 0)];
    let d = mat[(1, 1)];
    let tmp = ((-(a + d) / 2.0).powi(2) - a * d + b * c).sqrt();
    let l1 = (a + d) / 2.0 + tmp;
    let l2 = (a + d) / 2.0 - tmp;
    if l1.abs() > l2.abs() {
        (l1, l2)
    } else {
        (l2, l1)
    }
}

pub fn swap_columns<T>(mat: &mut ndarray::Array2<T>, idx1: usize, idx2: usize)
where
    ndarray::OwnedRepr<T>: ndarray::Data,
{
    let s = mat.raw_dim();
    for i in 0..s[0] {
        mat.swap((i, idx1), (i, idx2));
    }
}

pub fn swap_rows<T>(mat: &mut ndarray::Array2<T>, idx1: usize, idx2: usize)
where
    ndarray::OwnedRepr<T>: ndarray::Data,
{
    let s = mat.raw_dim();
    for i in 0..s[1] {
        mat.swap((idx1, i), (idx2, i));
    }
}

pub fn index_of_largest<'a, T>(c: &ndarray::ArrayView1<T>) -> usize
where
    <ndarray::ViewRepr<&'a T> as ndarray::Data>::Elem:
        std::cmp::PartialOrd + num::traits::Signed + Clone,
{
    // let mut max = num::abs(c[0].clone());
    let mut max = c[0].clone();
    let mut max_idx = 0;
    c.iter()
        .enumerate()
        .map(|(i, ci)| {
            // let ci = num::abs(ci.clone());
            let ci = ci.clone();
            if ci > max {
                max = ci;
                max_idx = i
            }
        })
        .count();
    max_idx
}

// allow dead code for now...
#[allow(dead_code)]
pub fn index_of_largest_abs<'a, T>(c: &ndarray::ArrayView1<T>) -> usize
where
    <ndarray::ViewRepr<&'a T> as ndarray::Data>::Elem:
        std::cmp::PartialOrd + num::traits::Signed + Clone,
{
    let mut max = num::abs(c[0].clone());
    let mut max_idx = 0;
    c.iter()
        .enumerate()
        .map(|(i, ci)| {
            let ci = num::abs(ci.clone());
            if ci > max {
                max = ci;
                max_idx = i
            }
        })
        .count();
    max_idx
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_swap_columns() {
        let mut a: ndarray::Array2<i64> = ndarray::arr2(&[[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
        super::swap_columns(&mut a, 1, 2);
        let c: ndarray::Array2<i64> = ndarray::arr2(&[[1, 3, 2], [4, 6, 5], [7, 9, 8]]);
        a.iter()
            .zip(c.iter())
            .map(|(x, y)| assert_eq!(*x, *y))
            .count();
        // this should work, but it doesn't.
        // assert_eq!(b, c);
    }

    #[test]
    fn test_swap_rows() {
        let mut a: ndarray::Array2<i64> = ndarray::arr2(&[[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
        super::swap_rows(&mut a, 1, 2);
        let c: ndarray::Array2<i64> = ndarray::arr2(&[[1, 2, 3], [7, 8, 9], [4, 5, 6]]);
        a.iter()
            .zip(c.iter())
            .map(|(x, y)| assert_eq!(*x, *y))
            .count();
        // this should work, but it doesn't.
        // assert_eq!(b, c);
    }

    #[test]
    fn test_swap_rows_and_columns() {
        let mut a: ndarray::Array2<i64> = ndarray::arr2(&[[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
        super::swap_rows(&mut a, 1, 2);
        super::swap_columns(&mut a, 1, 2);
        let c: ndarray::Array2<i64> = ndarray::arr2(&[[1, 3, 2], [7, 9, 8], [4, 6, 5]]);
        a.iter()
            .zip(c.iter())
            .map(|(x, y)| assert_eq!(*x, *y))
            .count();
        // this should work, but it doesn't.
        // assert_eq!(b, c);
    }

    #[test]
    fn test_biggest_index() {
        use ndarray::s;
        let j = 1;
        let a: ndarray::Array2<i64> =
            ndarray::arr2(&[[1, 2, 3, 0], [4, 2, 6, 0], [7, 8, 3, 0], [3, 4, 2, 8]]);
        let idx = super::index_of_largest(&a.diag().slice(s![j..]));
        assert_eq!(idx + j, 3);
        // this should work, but it doesn't.
        // assert_eq!(b, c);
    }

    #[test]
    fn test_biggest_index_abs() {
        use ndarray::s;
        let j = 1;
        let a: ndarray::Array2<i64> =
            ndarray::arr2(&[[1, 2, 3, 0], [4, 2, 6, 0], [7, 8, 3, 0], [3, 4, 2, -8]]);
        let idx = super::index_of_largest_abs(&a.diag().slice(s![j..]));
        assert_eq!(idx + j, 3);
        // this should work, but it doesn't.
        // assert_eq!(b, c);
    }
}