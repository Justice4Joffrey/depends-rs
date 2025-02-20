use std::{
    cmp::Reverse,
    collections::{BTreeMap, BTreeSet, HashMap},
};

use petgraph::{algo::is_cyclic_directed, graph::NodeIndex, Directed, Direction, Graph};
use proc_macro2::{Ident, Span};
use syn::spanned::Spanned;

use super::parser::GraphvizModel;
use crate::graphviz::{
    graph::{DerivedNodeDef, GraphvizGraph, NodeDef},
    parser::VertexOrEdge,
};

#[cfg_attr(test, derive(Debug))]
pub struct ParsedGraphvizModel {
    pub name: String,
    pub vertices: BTreeMap<String, Node>,
    pub edges: Vec<Edge>,
}

impl ParsedGraphvizModel {
    fn edges_by_vertices(&self) -> HashMap<&str, Vec<&Edge>> {
        let mut result = HashMap::<_, Vec<&Edge>>::new();
        for edge in &self.edges {
            result.entry(edge.to.as_str()).or_default().push(edge);
        }
        result
    }
}

const LABEL: &str = "label";
const CLASS: &str = "class";

#[cfg_attr(test, derive(Debug))]
#[derive(Clone)]
pub struct Node {
    pub ty: String,
    pub span: Span,
    pub attr_span: Span,
}

#[cfg_attr(test, derive(Debug))]
pub struct Edge {
    pub from: String,
    pub to: String,
    pub from_span: Span,
    pub to_span: Span,
    pub operation: String,
    pub dependency_group: Option<String>,
}

impl TryFrom<GraphvizModel> for ParsedGraphvizModel {
    type Error = syn::Error;

    fn try_from(value: GraphvizModel) -> Result<Self, Self::Error> {
        let mut vertices = BTreeMap::new();
        let mut edges = Vec::new();
        for v in value.vertex_or_edges {
            let attribute_map = v.attribute_map();
            match v {
                VertexOrEdge::Vertex(v) => {
                    let ty = attribute_map.get(LABEL).map(|l| l.value()).ok_or_else(|| {
                        syn::Error::new(
                            v.attr_span.span(),
                            format!("Vertex `{}` has no label", v.ident),
                        )
                    })?;
                    vertices.insert(
                        v.ident.to_string(),
                        Node {
                            ty,
                            span: v.ident.span(),
                            attr_span: v.attr_span.span(),
                        },
                    );
                }
                VertexOrEdge::Edge(e) => {
                    let operation = attribute_map.get(LABEL).ok_or_else(|| {
                        syn::Error::new(
                            e.attr_span.span(),
                            format!("Edge `{} -> {}` has no label", e.from, e.to),
                        )
                    })?;
                    let dependency_group = attribute_map.get(CLASS).map(|l| l.value());
                    edges.push(Edge {
                        from: e.from.to_string(),
                        to: e.to.to_string(),
                        from_span: e.from.span(),
                        to_span: e.to.span(),
                        operation: operation.value().to_string(),
                        dependency_group,
                    });
                }
            }
        }
        Ok(Self {
            name: value.name.to_string(),
            vertices,
            edges,
        })
    }
}

fn find_root(graph: &Graph<String, String, Directed>) -> syn::Result<NodeIndex> {
    let nodes = graph
        .node_indices()
        .filter(|&node| graph.edges_directed(node, Direction::Outgoing).count() == 0)
        .collect::<Vec<_>>();
    match nodes.len() {
        // This should never happen, as this graph will necessarily have not
        // enough edges or be cyclic.
        0 => {
            Err(syn::Error::new(
                Span::call_site(),
                "Expected at least one root node, found none.",
            ))
        }
        1 => Ok(nodes[0]),
        _ => {
            Err(syn::Error::new(
                Span::call_site(),
                format!(
                    "More than one root node: {}.",
                    nodes_to_string(graph, &nodes)
                ),
            ))
        }
    }
}

fn longest_path_to_root(
    graph: &Graph<String, String, Directed>,
    node: NodeIndex,
    root: NodeIndex,
    cache: &mut HashMap<NodeIndex, usize>,
) -> usize {
    // If we've calculated the longest path for this node before, return it.
    if let Some(&result) = cache.get(&node) {
        return result;
    }

    // If we're at the root node, the longest path is 0.
    if node == root {
        cache.insert(node, 0);
        return 0;
    }

    // Initialize the longest path to be 0.
    let mut longest_path = 0;
    for neighbor in graph.neighbors(node) {
        longest_path = longest_path.max(1 + longest_path_to_root(graph, neighbor, root, cache));
    }

    // Cache the result for this node.
    cache.insert(node, longest_path);

    longest_path
}

fn longest_paths_to_root(
    graph: &Graph<String, String, Directed>,
    root: NodeIndex,
) -> HashMap<NodeIndex, usize> {
    let mut cache = HashMap::new();

    for node in graph.node_indices() {
        longest_path_to_root(graph, node, root, &mut cache);
    }

    cache
}

fn nodes_to_string(graph: &Graph<String, String, Directed>, nodes: &[NodeIndex]) -> String {
    nodes
        .iter()
        .map(|node| graph.node_weight(*node).unwrap().to_string())
        .collect::<Vec<_>>()
        .join(", ")
}

fn unknown_node_err(span: Span) -> syn::Error {
    syn::Error::new(span, "Unknown node.")
}

impl TryFrom<ParsedGraphvizModel> for GraphvizGraph {
    type Error = syn::Error;

    fn try_from(value: ParsedGraphvizModel) -> Result<Self, Self::Error> {
        if value.vertices.len() < 2 {
            return Err(syn::Error::new(
                Span::call_site(),
                "A Graphviz model must have at least 2 vertices.",
            ));
        }
        if value.edges.is_empty() {
            return Err(syn::Error::new(
                Span::call_site(),
                "A Graphviz model must have at least 1 edge.",
            ));
        }
        let mut graph = Graph::<String, String>::new();
        let nodes = value
            .vertices
            .keys()
            .map(|k| (k.clone(), graph.add_node(k.clone())))
            .collect::<HashMap<_, _>>();

        let mut edges = Vec::with_capacity(value.edges.len());
        for Edge {
            from,
            to,
            from_span,
            to_span,
            ..
        } in value.edges.iter()
        {
            edges.push((
                *nodes
                    .get(from)
                    .ok_or_else(|| unknown_node_err(*from_span))?,
                *nodes.get(to).ok_or_else(|| unknown_node_err(*to_span))?,
            ));
        }

        graph.extend_with_edges(edges);

        if is_cyclic_directed(&graph) {
            return Err(syn::Error::new(
                Span::call_site(),
                "Cycle detected in graph definition.",
            ));
        }
        let root_node = find_root(&graph)?;
        let mut distances = longest_paths_to_root(&graph, root_node)
            .into_iter()
            .collect::<Vec<_>>();

        distances.sort_by_key(|(node_idx, distance)| (Reverse(*distance), *node_idx));
        let edges = value.edges_by_vertices();
        let length = distances.len();
        let mut derived = Vec::with_capacity(length);
        let mut inputs = Vec::with_capacity(length);
        for (d, _) in distances.into_iter() {
            let name = graph.node_weight(d).unwrap();
            let type_name = value.vertices[name].clone();
            let node_def = NodeDef {
                ident: Ident::new(name, Span::call_site()),
                ty: Ident::new(&type_name.ty, Span::call_site()),
            };
            match edges.get(name.as_str()) {
                Some(edges) => {
                    let set = edges
                        .iter()
                        .map(|e| (e.operation.clone(), e.dependency_group.clone()))
                        .collect::<BTreeSet<(String, Option<String>)>>();
                    if set.len() == 1 {
                        let (operation, dependency_type) = set.into_iter().next().unwrap();
                        let incoming = edges
                            .iter()
                            .map(|e| {
                                NodeDef {
                                    ident: Ident::new(&e.from, Span::call_site()),
                                    ty: Ident::new(&value.vertices[&e.from].ty, Span::call_site()),
                                }
                            })
                            .collect::<Vec<_>>();
                        let dependency_type = match (dependency_type, incoming.len()) {
                            (Some(dependency_type), _) => {
                                Some(Ident::new(&dependency_type, Span::call_site()))
                            }
                            (None, 1) => None,
                            (None, _) => {
                                return Err(syn::Error::new(
                                    value.vertices[name].attr_span,
                                    "Multiple incoming edges but no `class` attribute.",
                                ));
                            }
                        };
                        derived.push(DerivedNodeDef {
                            node: node_def,
                            operation: Ident::new(&operation, Span::call_site()),
                            incoming,
                            dependency_type,
                        });
                    } else {
                        let values = format!("{:?}", set);
                        return Err(syn::Error::new(
                            value.vertices[name].span,
                            format!(
                                "Multiple values for `label` and `class` on edges to node: {}.",
                                values
                            ),
                        ));
                    }
                }
                None => inputs.push(node_def),
            }
        }
        if derived.last().cloned().is_none() {
            // This should never happen because we checked that the graph is
            // connected.
            return Err(syn::Error::new(
                Span::call_site(),
                "Can't find a root node.",
            ));
        };

        Ok(GraphvizGraph {
            name: Ident::new(&value.name, Span::call_site()),
            inputs,
            derived,
        })
    }
}

#[cfg(test)]
mod tests {
    use syn::parse2;

    use super::*;

    #[test]
    fn test_parse_graphviz() {
        let input = quote::quote! {
            digraph MyGraph {
                a [label = "SomeType"];
                b [label = "AnotherType"];
                c [label = "SomeType"];
                a -> b [label = "some_path", class = "some_class"];
                c -> b [label = "another"];
            }
        };
        let model = parse2::<GraphvizModel>(input).unwrap();
        let parsed = ParsedGraphvizModel::try_from(model).unwrap();
        assert_eq!(parsed.name, "MyGraph".to_string());
        let vertices = parsed
            .vertices
            .iter()
            .map(|(k, v)| (k.as_str(), v.ty.as_str()))
            .collect::<Vec<_>>();
        assert_eq!(
            vertices,
            vec![("a", "SomeType"), ("b", "AnotherType"), ("c", "SomeType"),]
        );
        let edges = parsed
            .edges
            .iter()
            .map(|e| (e.from.as_str(), e.to.as_str(), e.operation.as_str()))
            .collect::<Vec<_>>();
        assert_eq!(edges, vec![("a", "b", "some_path"), ("c", "b", "another"),]);
    }

    #[test]
    fn test_parse_empty() {
        let input = quote::quote! {
            digraph G {

            }
        };
        let model = parse2::<GraphvizModel>(input).unwrap();
        let parsed = ParsedGraphvizModel::try_from(model).unwrap();
        assert_eq!(parsed.name, "G");
        assert!(parsed.vertices.is_empty());
        assert!(parsed.edges.is_empty());
    }
}
