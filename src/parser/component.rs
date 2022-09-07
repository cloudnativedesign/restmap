use attribute::{Attribute, ReferenceAttribute, ValueAttribute};
use super::schema::{Version};

///An element contained in a schema
pub struct Component<'a, T> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instantiate_component() {
        let component = Component::new();
    }

}
