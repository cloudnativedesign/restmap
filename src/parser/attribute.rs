
//REFERENCE ATTRIBUTE -------------------------
struct ReferenceAttribute<'a, T> {
    name: &'a str,
    reference_type: ReferenceAttributeType,
    reference: &'a str,
    required: bool, 
    resolved: bool,
    resolved_value: Option<T>
} 
impl<'a, T> ReferenceAttribute<'a, T> {
    pub fn new(name: &str, 
               reference: &str,
               reference_type: ReferenceAttributeType,
               required: bool,
               ) -> Self {
        ReferenceAttribute {
            name, 
            reference_type, 
            reference,
            required,
            resolved: false,
            resolved_value: None,
        }
    }
}
impl<'a, T> Attribute<T> for ReferenceAttribute<'a, T> {
    fn is_reference(&self) -> bool {true}
}

//VALUE ATTRIBUTE-------------------------
struct ValueAttribute<'a> {
    name: &'a str,
    datatype: ValueType,
    required: bool,

}
impl<'a> ValueAttribute<'a> {
    pub fn new(name: &str, datatype: ValueType, required:bool) -> Self {
        ValueAttribute {
            name, datatype, required
        }
    }
}
impl<'a, T> Attribute<T> for ValueAttribute<'a> {
   fn is_reference(&self) -> bool {false} 
}

///Supported data types within the Schema
enum ValueType {
    Str,
    Bool,
    Int,
    Float,
    Object,
}

///Allowed reference attributes within the Schema version
enum ReferenceAttributeType {
    Resolver,
    Endpoint,
    Param
}

#[cfg(test)]
mod tests {
    #[test]
    fn instantiate_attribute() {
        let url = ValueAttribute::new("url", ValueType::Str, false);
    }
    #[test]
    fn instantiate_component() {
        let v: Version = "0.1.0".to_string();
        let comp = Component::new(
                &v,
                None
            );
    }
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
