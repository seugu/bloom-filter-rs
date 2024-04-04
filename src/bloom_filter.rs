use hex;
use sha3::{Digest, Sha3_256};
use crate::helper;

#[derive(PartialEq,Debug)]
pub struct  bloomFilter{
    state: Vec<bool>,
}

impl bloomFilter{
    // we init a bloom filter with a all zero vec  
    pub fn init(length: usize) -> Self{
        let state = vec![false; length];
        Self {state}
    }

    pub fn add_item(&self, item: &str) -> Self {
        let mut hasher = Sha3_256::new();
        hasher.update(item);
        let result = hasher.finalize();       
        todo!()
    } 
}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    pub fn init_test(){
        let bloomFilter5 = bloomFilter::init(5);
        let boolVec5 = vec![false; 5];
        assert_eq!(bloomFilter5,bloomFilter { state: boolVec5 });
    }

    #[test]
    pub fn add_item_test(){
        let bloomFilter5 = bloomFilter::init(256);
        bloomFilter5.add_item("first");
    }
}
