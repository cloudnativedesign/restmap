use super::schema::{Version};
///Attributes are used within the scope of a Component to define the set of allowed configuration
///attributes they can receive
trait Attribute<T> {

    ///Type guard to be used on functions working only with reference attributes
    fn is_reference(&self) -> bool;
        
    
    ///Checks if the attribute is required
    fn is_required(&self) -> bool {
        self.required
    }

    ///Sets the value after the reference has resolved 
    fn resolve(&mut self, value: T) {
        if self.is_reference() && !self.resolved {
            //Set the value 
            self.resolved_value = value;
            self.resolved = true;
        }
    }
}
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
    use super::*;

    #[test]
    fn instantiate_attribute() {
        let url = ValueAttribute::new("url", ValueType::Str, false);
    }
}
