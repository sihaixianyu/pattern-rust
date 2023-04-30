pub mod deserializer;
pub mod visitor;

#[cfg(test)]
mod tests {
    use super::deserializer::{StringDeserializer, TwoValueArray, TwoValueStruct, VecDeserializer, Deserializer};

    #[test]
    fn test_visitor() {
        let deserializer = StringDeserializer::create(TwoValueStruct::default());
        let result = deserializer.parse_str("123 456");
        println!("{:?}", result);

        let deserializer = VecDeserializer::create(TwoValueStruct::default());
        let result = deserializer.parse_vec(vec![123, 456]);
        println!("{:?}", result);

        let deserializer = VecDeserializer::create(TwoValueArray::default());
        let result = deserializer.parse_vec(vec![123, 456]);
        println!("{:?}", result);
        println!(
            "Error: {}",
            deserializer.parse_str("123 456").err().unwrap()
        )
    }
}
