//Details the schema definition to manage versions of the parsing system
//

///Defines the allowed structure and configuration of a specific schema version in the system. Used
///by the configuration parser to check syntax is correct
pub struct Schema<'a, T> {
    version: Version,
    components: &'a [Component<'a, T>]
}

type Version = String;
//impl version {
//    fn new(value: &str) -> Self {
//       value.to_string() 
//    }
//}

///A schema element with the associated attributes supported in a given version
struct Component<'a, T> {
    version: Version,
    attributes: Option<&'a Vec<Box<dyn Attribute<T>>>>
}
impl<'a, T> Component<'a, T> {
    pub fn new(version: &Version, attributes: Option<&[Box<dyn Attribute<T>>]>) -> Self {
        Component {
            version,
            attributes
        }
    }
    ///Appends additional allowed attribute to the Component
    pub fn add_attr(&mut self, attribute: &Box<dyn Attribute<T>>) {
        *self.attributes.push(attribute)
    }

    ///Removes an attribute by name
    pub fn remove_attr(&mut self, attribute_name: &str) {
        self.attributes = self.attributes.iter_mut().filter(|attr| *attr.name == attribute_name);
    }
}
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
        if self.is_reference() & !self.resolved {
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
    fn instantiate_version() {
        let version_010: Version = "0.1.0".to_string();
    }
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
