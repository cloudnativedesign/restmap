
// Configparser parses an incoming yaml read file into a seaquence of nodes that contain the
// parametrization based on a given configuration schema
// It uses the following types
// * Graph (Struct) : This holds the methods to work CRUD with the nodes
// * Traverse (Trait) : Enabling the node to be traversable through links
// * Parse (Trait) : Enabling a compiler to parse the node into a specific executable structure
// * 
use std::rc::Rc;
use std::cell::{RefCell, RefMut};

pub struct RawConfig {}

struct NodeConfig {}


/// Holds the public management interface to the graph
/// It enables structured traversal of the graph through iterator interfaces.
///
pub struct Graph<T>
where T: Parse<T> + Node {
    root: Option<Rc<RefMut<T>>>,
    depth: i32
}
impl<T> Graph<T> {
    pub fn new() {
        Graph {
            root: None,
            depth: 0,
        }
    }
    ///Adds a node to the graph as a child of the selected node
    pub fn add_child(&mut self, at_node: &T, child_node: T) {
        unimplemented!();
    }

    ///Remove a node from the graph indicated by a node reference
    pub fn pop(&mut self, node: &mut T) -> Option<T> {
        unimplemented!();
    }



}
///Node provides core interaction capacity to a specific NodeType
///Mainly intended to allow the graph to CRUD contained Nodes
pub trait Node {
    /// Get a reference to the parent node
    fn get_parent(&self) -> &Self;
    /// Get a mut reference to the parent node
    fn get_parent_mut(&mut self) -> &mut Self;
    /// List leave nodes for the given node
    fn list_leaves(&self) -> [&Self];
}
///Implements basic traversal capacities in the compute graph
trait Traverse<T>
where T: Node {}

trait Parse<T>
where T: Node {
    ///Parses a NodeConfig map into a Node
    fn parse(&mut self, node: NodeConfig) -> T;
}


struct FormatParam {}

struct ReplaceString {
    formstring: String,
    params: Option<Vec<FormatParam>>,
}
impl ReplaceString {
    pub fn new(formstring: String, params: Option<Vec<FormatParam>>) -> Self {
        ReplaceString {
            formstring,
            params
        }
    }
}
/// Endpoint represented in the ParseTree
struct EndpointNode {
    url: ReplaceString
}
impl<T> EndpointNode<T> {
    pub fn new(config: NodeConfig) -> Self {
        //Check the NodeConfig is valid

        EndpointNode {
            
        }
    }
}

/// Loaders represent data source connections to retrieve dependend data for params in the graph
struct LoaderNode {}

struct IteratorNode {}

struct ParamNode {}

