use super::schema::{Version};
///Attributes are used within the scope of a Component to define the set of allowed configuration
///attributes they can receive
pub trait Attribute<T> {

    ///Type guard to be used on functions working only with reference attributes
    fn is_reference(&self) -> bool;

    fn is_resolved(&self) -> bool;
        
    ///Checks if the attribute is required
    fn is_required(&self) -> bool;
    
    /// Enables filter operations 
    fn has_name(&self, name: &str) -> bool;

    ///Sets the value after the reference has resolved 
    fn resolve(&mut self, value: T);
}
//REFERENCE ATTRIBUTE -------------------------
pub struct ReferenceAttribute<'a, T> {
    name: String,
    reference_type: ReferenceAttributeType,
    reference: &'a str,
    required: bool, 
    resolved: bool,
    resolved_value: Option<T>
} 
impl<'a, T> ReferenceAttribute<'a, T> {
    pub fn new(name: String, 
               reference: &'a str,
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
    fn is_resolved(&self) -> bool {self.resolved}
    fn is_required(&self) -> bool {self.required}
    fn has_name(&self, name: &str) -> bool {self.name == *name}

    fn resolve(&mut self, value: T) {
        self.resolved_value = Some(value);
        self.resolved = true;
    }
}

//VALUE ATTRIBUTE-------------------------
pub struct ValueAttribute {
    name: String,  
    datatype: ValueType,
    required: bool,

}
impl ValueAttribute {
    pub fn new(name: String, datatype: ValueType, required:bool) -> Self {
        ValueAttribute {
            name, 
            datatype, 
            required
        }
    }
}
impl<T> Attribute<T> for ValueAttribute {
    fn is_reference(&self) -> bool {false}
    fn is_resolved(&self) -> bool {false}
    fn is_required(&self) -> bool {self.required}
    fn has_name(&self, name: &str) -> bool {self.name == *name}

    fn resolve(&mut self, value: T) {}
}

///Supported data types within the Schema
pub enum ValueType {
    Str,
    Bool,
    Int,
    Float,
    Object,
}

///Allowed reference attributes within the Schema version
pub enum ReferenceAttributeType {
    Resolver,
    Endpoint,
    Param
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instantiate_attribute() {
        let url = ValueAttribute::new("url".to_string(), ValueType::Str, false);
    }
}
