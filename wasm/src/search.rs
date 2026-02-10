use alloc::vec;
use alloc::vec::Vec;
use wasm_bindgen::prelude::{wasm_bindgen};

#[wasm_bindgen]
pub struct PrefixHash {
    poly: Vec<u64>,
    degree: Vec<u64>,
    size: usize,
}

#[wasm_bindgen]
impl PrefixHash {
    pub fn new(n: usize) -> Self {
        let key_size = 30; 
        Self {
            poly: vec![0; n + key_size + 1],
            degree: vec![1; n + key_size + 1],
            size: n
        }
    }
}

const P: u64 = 1_000_000_007;
const X: u64 = 257;

#[wasm_bindgen]
impl PrefixHash {
    pub fn build(&mut self, s: Vec<u16>) {
        for (c,i) in s.into_iter().zip(0..) {
            self.poly[i+1] = ((self.poly[i] + c as u64) * X) % P;
            self.degree[i+1] = (self.degree[i] * X) % P;
        }
    }

    pub fn find(&mut self, x: Vec<u16>) -> Vec<usize> {
        let k = x.len();
        for (c,i) in x.into_iter().zip((self.size+1)..) {
            self.poly[i+1] = ((self.poly[i] + c as u64) * X) % P;
            self.degree[i+1] = (self.degree[i] * X) % P;
        }
        let j = self.size+1;
        let mut result = vec![];
        for i in 1..=self.size {
            let a = (self.poly[i+k] + self.poly[j]*self.degree[k]) % P;
            let b = (self.poly[j+k] + self.poly[i]*self.degree[k]) % P;
            if a == b { result.push(i); }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    async fn test() {
        let xs = vec![104,101,108,108,111,32,119,111,114,108,100]; // hello world
        let ys = vec![119,111,114,108,100]; // world
        let mut prefix = PrefixHash::new(xs.len());
        prefix.build(xs);
        let result = prefix.find(ys);
        assert_eq!(result, vec![6]);
    }

    #[wasm_bindgen_test]
    async fn test2() {
        let xs = vec![104,101,108,108,111,32,119,111,114,108,100]; // hello world
        let ys = vec![101,101,101]; // eee
        let mut prefix = PrefixHash::new(xs.len());
        prefix.build(xs);
        let result = prefix.find(ys);
        assert_eq!(result, vec![] as Vec<usize>);
    }    

    #[wasm_bindgen_test]
    async fn test3() {
        let xs = vec![104,101,108,108,111,32,119,111,114,108,100]; // hello world
        let ys = vec![111]; // o
        let mut prefix = PrefixHash::new(xs.len());
        prefix.build(xs);
        let result = prefix.find(ys);
        assert_eq!(result, vec![4,7]);
    }    

}

