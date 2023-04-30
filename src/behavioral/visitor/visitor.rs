use super::deserializer::{TwoValueArray, TwoValueStruct};

pub trait Visitor {
    type Value;

    fn visit_vec(&self, v: Vec<i32>) -> Self::Value;
}

impl Visitor for TwoValueStruct {
    type Value = TwoValueStruct;

    fn visit_vec(&self, v: Vec<i32>) -> Self::Value {
        TwoValueStruct { a: v[0], b: v[1] }
    }
}

impl Visitor for TwoValueArray {
    type Value = TwoValueArray;

    fn visit_vec(&self, v: Vec<i32>) -> Self::Value {
        let mut ab = [0_i32; 2];

        ab[0] = v[0];
        ab[1] = v[1];

        TwoValueArray { ab }
    }
}
