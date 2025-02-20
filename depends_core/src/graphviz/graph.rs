use proc_macro2::Ident;

#[derive(Clone)]
#[cfg_attr(test, derive(Debug))]
pub struct DerivedNodeDef {
    pub node: NodeDef,
    pub operation: Ident,
    pub incoming: Vec<NodeDef>,
    /// None for single `Dependency`.
    pub dependency_type: Option<Ident>,
}

#[derive(Debug, Clone)]
pub struct NodeDef {
    pub ident: Ident,
    pub ty: Ident,
}

/// A graph of nodes which is guaranteed to to be connected, have at least 2
/// nodes, no cycles and one root node.
pub struct GraphvizGraph {
    pub name: Ident,
    /// All input nodes
    pub inputs: Vec<NodeDef>,
    /// All non-root derived nodes
    pub derived: Vec<DerivedNodeDef>,
}
