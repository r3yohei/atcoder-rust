/// セグ木
/// 
/// 各ノードには最初、idに相当する値が入っている。
/// get i: a[i]を返す
/// update i x: a[i]=x
/// query l r: [l,r)をカバーするノードに対してopを適用したもの
/// 使用例
/// let mut seg: SEG<SUM> = SEG::new(4);
/// seg.update(0,1);
/// assert_eq!(seg.query(0, 1), 1);

pub trait Monoid {
    type T: Clone + std::fmt::Debug;
    fn id() -> Self::T;
    fn op(a: &Self::T, b: &Self::T) -> Self::T;
}

struct SUM;
impl Monoid for SUM {
    type T = i64;
    fn id() -> Self::T {
        0
    }
    fn op(a: &Self::T, b: &Self::T) -> Self::T {
        *a + *b
    }
}

struct PRODUCT;
impl Monoid for PRODUCT {
    type T = i64;
    fn id() -> Self::T {
        1
    }
    fn op(a: &Self::T, b: &Self::T) -> Self::T {
        *a * *b
    }
}

struct MIN;
impl Monoid for MIN {
    type T = i64;
    fn id() -> Self::T {
        std::i64::MAX
    }
    fn op(a: &Self::T, b: &Self::T) -> Self::T {
        std::cmp::min(*a, *b)
    }
}

struct MAX;
impl Monoid for MAX {
    type T = i64;
    fn id() -> Self::T {
        std::i64::MIN
    }
    fn op(a: &Self::T, b: &Self::T) -> Self::T {
        std::cmp::max(*a, *b)
    }
}

struct GCD;
impl Monoid for GCD {
    type T = i64;
    fn id() -> Self::T {
        0
    }
    fn op(a: &Self::T, b: &Self::T) -> Self::T {
        num::integer::gcd(*a, *b)
    }
}

pub struct SEG<M: Monoid> {
    pub n: usize,
    pub buf: Vec<M::T>,
}

impl<M: Monoid> SEG<M> {
    pub fn new(n: usize) -> SEG<M> {
        let mut m = 1;
        while m < n { m *= 2; }
        SEG {
            n: m,
            buf: vec![M::id().clone(); 2 * m],
        }
    }

    pub fn update(&mut self, k: usize, a: M::T) {
        let mut k = k + self.n;
        self.buf[k] = a;

        while k > 1 {
            k = k >> 1;
            self.buf[k] = M::op(&self.buf[k*2], &self.buf[k*2+1]);
        }
    }
    
    pub fn get(&self, k: usize) -> M::T {
        self.buf[k + self.n].clone()
    }

    pub fn do_query(&self, a: usize, b: usize, k: usize, l: usize, r: usize) -> M::T {
        if r <= a || b <= l {
            return M::id();
        }

        if a <= l && r <= b {
            return self.buf[k].clone();
        } else {
            let vl = self.do_query(a,b,k*2,l,(l+r)/2);
            let vr = self.do_query(a,b,k*2+1,(l+r)/2,r);
            return M::op(&vl, &vr);
        }
    }

    // [a,b)
    pub fn query(&self, a: usize, b: usize) -> M::T {
        self.do_query(a,b,1,0,self.n)
    }
}