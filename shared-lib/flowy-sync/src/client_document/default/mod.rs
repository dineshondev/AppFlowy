use lib_ot::{core::OperationBuilder, text_delta::TextOperations};

#[inline]
pub fn initial_document_operations() -> TextOperations {
    OperationBuilder::new().insert("\n").build()
}

#[inline]
pub fn initial_document_str() -> String {
    initial_document_operations().json_str()
}

#[inline]
pub fn initial_read_me() -> TextOperations {
    let json = include_str!("READ_ME.json");
    TextOperations::from_json(json).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::client_document::default::initial_read_me;

    #[test]
    fn load_read_me() {
        println!("{}", initial_read_me().json_str());
    }
}