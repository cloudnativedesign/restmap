use super::attribute::{Attribute, ReferenceAttribute, ValueAttribute};
use super::schema::Version;

///An element contained in a schema
pub struct Component<T> {
    version: Version,
    attributes: Option<Vec<Box<dyn Attribute<T>>>>
}
impl<T> Component<T> {
    pub fn new(version: &Version, attributes: Option<Vec<Box<dyn Attribute<T>>>>) -> Self {
        Component {
            version: version.clone(),
            attributes
        }
    }
    ///Appends additional allowed attribute to the Component
    pub fn add_attr(&mut self, attribute: Box<dyn Attribute<T>>) {
        self.attributes.take().map(|mut vec| {
            vec.push(attribute)});
    }

    ///Removes an attribute by name
    pub fn remove_attr(&mut self, attribute_name: &str) {
        //self.attributes.into_iter().filter(|attr|{
            //*attr.has_name(attribute_name)
        //});
        unimplemented!();
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instantiate_component() {
    }

}
