use std::usize;

trait UnionFind {
    fn union(&mut self, p: usize, q: usize);
    fn find(&self, p: usize) -> usize;
    fn connected(&mut self, p: usize, q: usize) -> bool {
        self.find(p) == self.find(q)
    }
}

struct FirstFind {
    count: usize,
    id: Vec<usize>,
}

impl FirstFind {
    fn new(n: usize) -> Self {
        let id = (0..n).map(|i| i).collect();

        Self { count: n, id }
    }
}

impl UnionFind for FirstFind {
    fn union(&mut self, p: usize, q: usize) {
        // Reflexive : p is connected to p.
        // Symmetric : If p is connected to q, then q is connected to p.
        // Transitive : If p is connected to q and q is connected to r, then p is connected to r
        for i in 0..self.id.len() - 1 {
            if self.id[i] == self.id[p] {
                self.id[i] = self.id[p]
            }
        }

        self.count = self.count - 1;
        self.id[q] = self.id[p];
    }

    fn find(&self, p: usize) -> usize {
        self.id[p]
    }
}

struct QuickFind {
    count: usize,
    id: Vec<usize>,
    sz: Vec<usize>,
}

impl QuickFind {
    fn new(n: usize) -> QuickFind {
        let id: Vec<usize> = (0..n).map(|i| i).collect();
        Self {
            count: n,
            id: id.clone(),
            sz: id,
        }
    }
}

impl UnionFind for QuickFind {
    fn union(&mut self, p: usize, q: usize) {
        let i = self.find(p);
        let j = self.find(q);
        if self.sz[i] < self.sz[j] {
            self.id[i] = j;
            self.sz[j] += self.sz[i];
        } else {
            self.id[j] = i;
            self.sz[i] += self.sz[j];
        }
        self.count = self.count - 1;
    }

    fn find(&self, p: usize) -> usize {
        while p != self.id[p] {
            return p;
        }
        panic!("Out of bounds");
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stuff() {
        let mut f = FirstFind::new(21);

        assert_eq!(f.find(11), 11);
    }
}
