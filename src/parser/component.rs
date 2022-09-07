///An element contained in a schema
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
        if self.is_reference() && !self.resolved {
            //Set the value 
            self.resolved_value = value;
            self.resolved = true;
        }
    }
}
