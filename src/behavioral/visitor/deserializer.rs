use super::visitor::Visitor;

#[derive(Default, Debug)]
pub struct TwoValueStruct {
    pub a: i32,
    pub b: i32,
}

#[derive(Default, Debug)]
pub struct TwoValueArray {
    pub ab: [i32; 2],
}

pub trait Deserializer<V: Visitor> {
    fn create(visitor: V) -> Self;

    fn parse_str(&self, _input: &str) -> Result<V::Value, &'static str> {
        Err("parse_str is unimplemented")
    }

    fn parse_vec(&self, _input: Vec<i32>) -> Result<V::Value, &'static str> {
        Err("parse_vec is unimplemented")
    }
}

pub struct StringDeserializer<V: Visitor> {
    pub visitor: V,
}

impl<V: Visitor> Deserializer<V> for StringDeserializer<V> {
    fn create(visitor: V) -> Self {
        Self { visitor }
    }

    fn parse_str(&self, input: &str) -> Result<<V as Visitor>::Value, &'static str> {
        let input_vec: Vec<i32> = input
            .split_ascii_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        Ok(self.visitor.visit_vec(input_vec))
    }
}

pub struct VecDeserializer<V: Visitor> {
    pub visitor: V,
}

impl<V: Visitor> Deserializer<V> for VecDeserializer<V> {
    fn create(visitor: V) -> Self {
        Self { visitor }
    }

    fn parse_vec(&self, input: Vec<i32>) -> Result<<V as Visitor>::Value, &'static str> {
        Ok(self.visitor.visit_vec(input))
    }
}
