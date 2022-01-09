use std::{collections::HashMap, sync::Arc};

use crate::units::{Bits, Fraction, TimeInterval};

pub struct Topology;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(isize);

pub struct Node {
    id: NodeId,
    bandwidth_down: Bits,
    bandwidth_up: Bits,
}

pub struct Edge {
    src: NodeId,
    dst: NodeId,
    latency: TimeInterval,
    jitter: TimeInterval,
    loss: Fraction,
}

pub struct Network {
    nodes: HashMap<NodeId, Arc<Node>>,
    edges: HashMap<(NodeId, NodeId), Arc<Edge>>,
}

impl Network {
    fn node(&self, id: NodeId) -> Option<Arc<Node>> {
        self.nodes.get(&id).map(|node| node.clone())
    }

    fn edge(&self, src: NodeId, dst: NodeId) -> Option<Arc<Edge>> {
        self.edges.get(&(src, dst)).map(|edge| edge.clone())
    }
}

pub struct Path {
    path: Vec<Arc<NodeId>>,
    latency: TimeInterval,
    loss: Fraction,
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(core::cmp::Ordering::Equal)
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let ord = self.latency.partial_cmp(&other.latency);

        if ord != Some(core::cmp::Ordering::Equal) {
            return ord;
        }

        self.loss.partial_cmp(&other.loss)
    }
}
