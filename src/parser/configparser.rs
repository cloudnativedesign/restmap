// Configuration Parser conducts validation on the passed, syntactically correct YAML
// configuration and creates the internal representation of the program tree. It is a prestepjjjk

trait Node {
    /// Get a reference to the parent node
    fn get_parent(&self) -> &Self {}
    /// Get a mut reference to the parent node
    fn get_parent_mut(&self) -> &mut Self {}
    /// List leave nodes for the given node
    fn list_leaves(&self) -> [&Self] {}

}



pub struct RawConfig {}
pub struct ParseTree <T>{
    nodes: Vec<T, T: Node>
}
/// Element of the ParseTree
struct GraphNode {}
/// Endpoint represented in the ParseTree
struct EndpointNode {}
/// Loaders represent data source connections to retrieve dependend data for params in the graph
struct LoaderNode {}
struct IteratorNode {}
struct ParamNode {}

