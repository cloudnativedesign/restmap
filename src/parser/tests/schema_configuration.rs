mod tests {
    use crate::parser::schema::{ValueType};
    use crate::parser::component::{Component};
    use crate::parser::attribute::{ValueAttribute, ReferenceAttribute};

    #[test]
    fn add_attribute_to_component() {
        let comp = Component::new(&"0.1.0".to_string(), None);
        let url = ValueAttribute::new("url", ValueType::Str, false);
        comp.add_attr(&url);
    }

    #[test]
    fn remove_attribute_from_component() {
        let comp = Component::new(&"0.1.0".to_string(), None);
        let url = ValueAttribute::new("url", ValueType::Str, false);
        comp.add_attr(&url);
        comp.remove_attr("url");
    }
}
