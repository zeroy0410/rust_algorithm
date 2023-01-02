#![allow(non_snake_case)]
#![allow(dead_code)]

use std::cmp;

pub struct SegmentTree<T: Default + Ord + Copy> {
    len: usize,
    buf: Vec<T>,
    op: Ops,
}

pub enum Ops {
    Max,
    Min,
}

impl<T: Default + Ord + Copy> SegmentTree<T> {
    fn up(&mut self, p: usize) {
        self.buf[p] = match self.op {
            Ops::Max => self.buf[p].max(cmp::max(self.buf[p * 2 + 1],self.buf[p * 2])),
            Ops::Min => self.buf[p].min(cmp::min(self.buf[p * 2 + 1],self.buf[p * 2])),
        }
    }
    fn build(&mut self, arr: &[T], l: usize, r: usize, p: usize) {
        if l == r {
            self.buf[p] = arr[l];
            return;
        }
        let mid = (l + r) / 2;
        self.build(arr, l, mid, p << 1);
        self.build(arr, mid+1, r, p << 1 | 1);
        self.up(p);
    }
    pub fn from_vec(arr: &[T], op: Ops) -> Self {
        let len = arr.len();
        let buf: Vec<T> = vec![T::default(); 4 * len];
        let mut v=SegmentTree { len, buf, op };
        v.build(arr, 0, len-1, 1);
        v
    }
    pub fn query(&self, l: usize, r: usize, p: usize, L: usize , R: usize) -> T {
        if l == L && r == R {
            return self.buf[p];
        }
        let mid = (L + R) / 2;
        if r <= mid {
            return self.query(l, r, p<<1, L, mid);
        }
        else if l > mid {
            return self.query(l, r, p<<1|1, mid+1,r);
        }
        return match self.op {
            Ops::Max => cmp::max(self.query(l,mid,p<<1,L,mid),self.query(mid+1,r,p<<1|1,mid+1,R)),
            Ops::Min => cmp::min(self.query(l,mid,p<<1,L,mid),self.query(mid+1,r,p<<1|1,mid+1,R))
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn it_works() {
        let vec = vec![1, 2, -4, 7, 3, -5, 6, 11, -20, 9, 14, 15, 5, 2, -8];
        let min_seg_tree = SegmentTree::from_vec(&vec, Ops::Min);
        assert_eq!(-5, min_seg_tree.query(4, 6, 1,0,vec.len()-1));
        assert_eq!(-20, min_seg_tree.query(0, vec.len() - 1, 1,0,vec.len()-1));
    }
}