use super::{
    debug_wrapper, BaseNode, BaseNodeTrait, CommonNodeTrait, InheritanceNode, InheritanceNodeTrait,
};
use crate::graph::value_wrappers::KBValue;
use std::cmp::{Eq, PartialEq};
use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::hash::Hash;
use std::rc::Rc;

/// Final node wrapper that offers a stable API for all concept abstractions dependent on it.
#[derive(Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct FinalNode {
    inode: InheritanceNode,
}

#[allow(clippy::new_without_default)]
impl FinalNode {
    /// Create a new node.
    pub fn new() -> Self {
        FinalNode {
            inode: InheritanceNode::new(),
        }
    }

    /// Create a new node with an inheritance relation.
    pub fn new_with_inheritance(type_id: usize) -> Self {
        Self::from(InheritanceNode::new_with_inheritance(type_id))
    }

    /// Leak inheritance-level functionality.
    pub fn inheritance_wrapper(&self) -> &InheritanceNode {
        &self.inode
    }

    /// Leak base-level functionality.
    pub fn base_wrapper(&self) -> &BaseNode {
        &self.inode.base_wrapper()
    }
}

impl From<usize> for FinalNode {
    fn from(id: usize) -> Self {
        FinalNode {
            inode: InheritanceNode::from(id),
        }
    }
}

impl From<BaseNode> for FinalNode {
    fn from(b: BaseNode) -> Self {
        FinalNode {
            inode: InheritanceNode::from(b),
        }
    }
}

impl From<InheritanceNode> for FinalNode {
    fn from(b: InheritanceNode) -> Self {
        FinalNode { inode: b }
    }
}

impl<'a> TryFrom<&'a str> for FinalNode {
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        InheritanceNode::try_from(name).map(|n| FinalNode { inode: n })
    }
}

impl Debug for FinalNode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        debug_wrapper("FWrapper", self, f)
    }
}

impl CommonNodeTrait for FinalNode {
    fn id(&self) -> usize {
        self.inode.id()
    }

    fn set_internal_name(&mut self, name: String) {
        self.inode.set_internal_name(name);
    }

    fn internal_name(&self) -> Option<Rc<String>> {
        self.inode.internal_name()
    }
}

impl BaseNodeTrait<FinalNode> for FinalNode {
    fn set_value(&mut self, value: Rc<dyn KBValue>) {
        self.inode.set_value(value)
    }

    fn value(&self) -> Option<Rc<dyn KBValue>> {
        self.inode.value()
    }

    fn add_flag(&mut self, flag_type: usize) {
        self.inode.add_flag(flag_type);
    }

    fn has_flag(&self, flag_type: usize) -> bool {
        self.inode.has_flag(flag_type)
    }

    fn add_outgoing(&mut self, edge_type: usize, to: &FinalNode) {
        self.inode.add_outgoing(edge_type, &to.inode)
    }

    fn add_incoming(&mut self, edge_type: usize, from: &FinalNode) {
        self.inode.add_incoming(edge_type, &from.inode)
    }

    fn has_outgoing(&self, edge_type: usize, to: &FinalNode) -> bool {
        self.inode.has_outgoing(edge_type, &to.inode)
    }

    fn has_incoming(&self, edge_type: usize, from: &FinalNode) -> bool {
        self.inode.has_incoming(edge_type, &from.inode)
    }

    fn outgoing_nodes(&self, edge_type: usize) -> Vec<FinalNode> {
        self.inode
            .outgoing_nodes(edge_type)
            .into_iter()
            .map(FinalNode::from)
            .collect()
    }

    fn incoming_nodes(&self, edge_type: usize) -> Vec<FinalNode> {
        self.inode
            .incoming_nodes(edge_type)
            .into_iter()
            .map(FinalNode::from)
            .collect()
    }
}

impl InheritanceNodeTrait<FinalNode> for FinalNode {
    fn inheritance_nodes(&self) -> Vec<FinalNode> {
        self.inode
            .inheritance_nodes()
            .into_iter()
            .map(FinalNode::from)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::value_wrappers::{unwrap_weak, WeakValue};
    use crate::tao::archetype::ArchetypeTrait;
    use crate::tao::initialize_kb;
    use crate::tao::relation::attribute::{Inherits, Owner};

    #[test]
    fn create_and_retrieve_node_id() {
        initialize_kb();
        let node1 = FinalNode::new();
        let node2 = FinalNode::new();
        assert_eq!(node1.id() + 1, node2.id());
    }

    #[test]
    fn from_node_id() {
        initialize_kb();
        let node = FinalNode::new();
        let node_copy = FinalNode::from(node.id());
        assert_eq!(node.id(), node_copy.id());
    }

    #[test]
    fn from_name() {
        initialize_kb();
        let mut node = FinalNode::new();
        node.set_internal_name("A".to_string());
        assert_eq!(FinalNode::try_from("A"), Ok(node));
        assert!(FinalNode::try_from("B").is_err());
    }

    #[test]
    fn create_and_retrieve_node_name() {
        initialize_kb();
        let mut node = FinalNode::new();
        node.set_internal_name("A".to_string());
        assert_eq!(node.internal_name(), Some(Rc::new("A".to_string())));
    }

    #[test]
    fn retrieve_node_value() {
        initialize_kb();
        let mut node = FinalNode::new();
        let v = Rc::new(5);
        node.set_value(Rc::new(WeakValue::new(&v)));
        assert_eq!(unwrap_weak::<i32>(node.value()), Some(v));
    }

    #[test]
    fn create_with_inheritance() {
        initialize_kb();
        let owner = FinalNode::new();
        let mut type1 = FinalNode::new();
        type1.add_outgoing(Owner::TYPE_ID, &owner);
        let node = FinalNode::new_with_inheritance(type1.id());
        assert!(node.has_outgoing(Owner::TYPE_ID, &owner));
    }

    #[test]
    fn check_inheritance_nodes() {
        initialize_kb();
        let type1 = FinalNode::new();
        let mut type2 = FinalNode::new();
        let mut a = FinalNode::new();
        type2.add_outgoing(Inherits::TYPE_ID, &type1);
        a.add_outgoing(Inherits::TYPE_ID, &type2);
        assert_eq!(a.inheritance_nodes(), vec![type1, type2, a]);
        assert_eq!(type2.inheritance_nodes(), vec![type1, type2]);
        assert_eq!(type1.inheritance_nodes(), vec![type1]);
    }

    #[test]
    fn test_flags() {
        initialize_kb();
        let mut a = FinalNode::new();
        let b = FinalNode::new();
        assert!(!a.has_flag(b.id()));

        a.add_flag(b.id());
        assert!(a.has_flag(b.id()));
    }

    #[test]
    fn test_inherited_flags() {
        initialize_kb();
        let mut a = FinalNode::new();
        let b = FinalNode::new();
        let mut c = FinalNode::new();
        c.add_outgoing(Inherits::TYPE_ID, &a);
        assert!(!c.has_flag(b.id()));

        a.add_flag(b.id());
        assert!(c.has_flag(b.id()));
    }

    #[test]
    fn no_outgoing_nodes() {
        initialize_kb();
        let a = FinalNode::new();
        assert_eq!(a.outgoing_nodes(a.id()), Vec::new());
    }

    #[allow(clippy::many_single_char_names)]
    #[test]
    fn outgoing_nodes() {
        initialize_kb();
        let mut a = FinalNode::new();
        let b = FinalNode::new();
        let c = FinalNode::new();
        let d = FinalNode::new();
        let mut e = FinalNode::new();
        let edge_type1 = FinalNode::new();
        let edge_type2 = FinalNode::new();
        a.add_outgoing(edge_type1.id(), &b);
        a.add_outgoing(edge_type2.id(), &c);
        a.add_outgoing(edge_type1.id(), &d);
        e.add_outgoing(edge_type1.id(), &a);
        assert_eq!(a.outgoing_nodes(edge_type1.id()), vec![b, d]);
    }

    #[test]
    fn inherited_outgoing_nodes() {
        initialize_kb();
        let mut type1 = FinalNode::new();
        let mut type2 = FinalNode::new();
        let mut a = FinalNode::new();
        let b = FinalNode::new();
        let c = FinalNode::new();
        let mut d = FinalNode::new();
        let edge_type = FinalNode::new();
        a.add_outgoing(edge_type.id(), &b);
        type1.add_outgoing(edge_type.id(), &c);
        type2.add_outgoing(edge_type.id(), &c);
        d.add_outgoing(edge_type.id(), &a);

        type2.add_outgoing(Inherits::TYPE_ID, &type1);
        a.add_outgoing(Inherits::TYPE_ID, &type2);
        assert_eq!(a.outgoing_nodes(edge_type.id()), vec![b, c]);
        assert_eq!(type1.outgoing_nodes(edge_type.id()), vec![c]);
    }

    #[test]
    fn no_incoming_nodes() {
        initialize_kb();
        let a = FinalNode::new();
        assert_eq!(a.incoming_nodes(a.id()), Vec::new());
    }

    #[allow(clippy::many_single_char_names)]
    #[test]
    fn incoming_nodes() {
        initialize_kb();
        let mut a = FinalNode::new();
        let b = FinalNode::new();
        let c = FinalNode::new();
        let d = FinalNode::new();
        let mut e = FinalNode::new();
        let edge_type1 = FinalNode::new();
        let edge_type2 = FinalNode::new();
        a.add_incoming(edge_type1.id(), &b);
        a.add_incoming(edge_type2.id(), &c);
        a.add_incoming(edge_type1.id(), &d);
        e.add_incoming(edge_type1.id(), &a);
        assert_eq!(a.incoming_nodes(edge_type1.id()), vec![b, d]);
    }

    #[test]
    fn inherited_incoming_nodes() {
        initialize_kb();
        let mut type1 = FinalNode::new();
        let mut type2 = FinalNode::new();
        let mut a = FinalNode::new();
        let b = FinalNode::new();
        let c = FinalNode::new();
        let mut d = FinalNode::new();
        let edge_type = FinalNode::new();
        a.add_incoming(edge_type.id(), &b);
        type1.add_incoming(edge_type.id(), &c);
        type2.add_incoming(edge_type.id(), &c);
        d.add_incoming(edge_type.id(), &a);

        // inherit links are always outgoing
        type2.add_outgoing(Inherits::TYPE_ID, &type1);
        a.add_outgoing(Inherits::TYPE_ID, &type2);
        assert_eq!(a.incoming_nodes(edge_type.id()), vec![b, c]);
        assert_eq!(type1.incoming_nodes(edge_type.id()), vec![c]);
    }

    #[test]
    fn test_has_outgoing() {
        initialize_kb();
        let mut a = FinalNode::new();
        let b = FinalNode::new();
        let edge_type1 = FinalNode::new();
        let edge_type2 = FinalNode::new();
        a.add_outgoing(edge_type1.id(), &b);
        assert!(a.has_outgoing(edge_type1.id(), &b));
        assert!(!a.has_outgoing(edge_type2.id(), &b));
        assert!(!b.has_outgoing(edge_type1.id(), &a));
    }

    #[test]
    fn test_has_incoming() {
        initialize_kb();
        let mut a = FinalNode::new();
        let b = FinalNode::new();
        let edge_type1 = FinalNode::new();
        let edge_type2 = FinalNode::new();
        a.add_incoming(edge_type1.id(), &b);
        assert!(a.has_incoming(edge_type1.id(), &b));
        assert!(!a.has_incoming(edge_type2.id(), &b));
        assert!(!b.has_incoming(edge_type1.id(), &a));
    }

    #[test]
    fn inherited_has_outgoing() {
        initialize_kb();
        let mut type1 = FinalNode::new();
        let mut type2 = FinalNode::new();
        let mut a = FinalNode::new();
        let b = FinalNode::new();
        let c = FinalNode::new();
        let edge_type = FinalNode::new();
        type1.add_outgoing(edge_type.id(), &b);
        type1.add_incoming(edge_type.id(), &c);

        // inherit links are always outgoing
        type2.add_outgoing(Inherits::TYPE_ID, &type1);
        a.add_outgoing(Inherits::TYPE_ID, &type2);
        assert!(a.has_outgoing(edge_type.id(), &b));
        assert!(!a.has_outgoing(edge_type.id(), &c));
    }

    #[test]
    fn inherited_has_incoming() {
        initialize_kb();
        let mut type1 = FinalNode::new();
        let mut type2 = FinalNode::new();
        let mut a = FinalNode::new();
        let b = FinalNode::new();
        let c = FinalNode::new();
        let edge_type = FinalNode::new();
        type1.add_outgoing(edge_type.id(), &b);
        type1.add_incoming(edge_type.id(), &c);

        // inherit links are always outgoing
        type2.add_outgoing(Inherits::TYPE_ID, &type1);
        a.add_outgoing(Inherits::TYPE_ID, &type2);
        assert!(!a.has_incoming(edge_type.id(), &b));
        assert!(a.has_incoming(edge_type.id(), &c));
    }
}
