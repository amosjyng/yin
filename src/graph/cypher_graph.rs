use super::{Graph, KBWrapper, StrongWrapper, WeakWrapper};
use rusted_cypher::cypher_stmt;
use std::collections::HashMap;
use std::rc::Rc;

macro_rules! exec_db {
    ($db:expr, $query:expr) => {
        $db.exec($query).unwrap()
    };
    ($db:expr, $query:expr, $bindings:tt) => {
        $db.exec(cypher_stmt!($query, $bindings).unwrap()).unwrap()
    };
    ( $db:expr, $query:expr, $bindings:tt, { $( $k:expr => $v:ty ),+ } ) => {
        $db.exec(cypher_stmt!($query, $bindings).unwrap()).unwrap().rows()
        $(
            .map(|r| r.get::<$v>($k).unwrap())
        )*
    }
}

/// Graph that is backed by a Neo4j graph database.
pub struct CypherGraph {
    db: rusted_cypher::GraphClient,
}

impl CypherGraph {
    /// Constructs an empty new in-memory graph
    pub fn new(uri: &str) -> Self {
        match rusted_cypher::GraphClient::connect(uri) {
            Ok(client) => CypherGraph { db: client },
            Err(e) => panic!(
                "Couldn't connect to Neo4j database at {}. Error: {}",
                uri, e
            ),
        }
    }
}

impl Graph for CypherGraph {
    fn size(&self) -> usize {
        exec_db!(self.db, "MATCH (n) RETURN COUNT(*)")
            .rows()
            .next()
            .unwrap()
            .get::<usize>("COUNT(*)")
            .unwrap()
    }

    fn add_node(&mut self) -> usize {
        exec_db!(self.db, "CREATE (n) RETURN ID(n)")
            .rows()
            .next()
            .unwrap()
            .get::<usize>("ID(n)")
            .unwrap()
    }

    fn set_node_value(&mut self, id: usize, value: Box<dyn KBWrapper>) {
        // todo: see if lifetime ugliness can be cleaned up without cloning
        let unwrapped_value = match value.as_any().downcast_ref::<WeakWrapper<String>>() {
            Some(ww) => {
                let x = ww.value().unwrap().clone();
                (*x).clone()
            }
            None => value
                .as_any()
                .downcast_ref::<StrongWrapper<String>>()
                .unwrap()
                .value()
                .as_str()
                .clone()
                .to_string(),
        };
        exec_db!(self.db, "MATCH (n) WHERE ID(n) = {id} SET n.value = {value}", {
            "id" => id,
            "value" => unwrapped_value.as_str()
        });
    }

    fn set_node_name(&mut self, id: usize, name: String) {
        exec_db!(self.db, "MATCH (n) WHERE ID(n) = {id} SET n.name = {name}", {
            "id" => id,
            "name" => name.as_str()
        });
    }

    fn node_name(&self, id: usize) -> Option<Rc<String>> {
        exec_db!(self.db, "MATCH (n) WHERE ID(n) = {id} RETURN n.name", {
            "id" => id
        }, {
            "n.name" => Option<String>
        })
        .next()
        .unwrap()
        .map(|s| Rc::new(s))
    }

    fn node_value(&self, id: usize) -> Option<Rc<Box<dyn KBWrapper>>> {
        exec_db!(self.db, "MATCH (n) WHERE ID(n) = {id} RETURN n.value", {
            "id" => id
        }, {
            "n.value" => Option<String>
        })
        .next()
        .unwrap()
        .map(|s| Rc::new(Box::new(StrongWrapper::new(s)) as Box<dyn KBWrapper>))
    }

    fn add_edge(&mut self, from: usize, edge_type: usize, to: usize) {
        exec_db!(
        self.db,
            "MATCH (a), (b) \
            WHERE ID(a) = {from} AND ID(b) = {to} \
            CREATE (a)-[r:R { id: {edge} }]->(b)", {
                "from" => from,
                "to" => to,
                "edge" => edge_type
            });
    }

    fn has_edge(&self, from: usize, edge_type: usize, to: usize) -> bool {
        exec_db!(
        self.db,
            "MATCH (a)-[r:R { id: {edge} }]->(b) \
            WHERE ID(a) = {from} AND ID(b) = {to} \
            RETURN COUNT(r)", {
                "from" => from,
                "edge" => edge_type,
                "to" => to
            }, {
                "COUNT(r)" => usize
            })
        .next()
        .unwrap()
            > 0
    }

    fn outgoing_nodes(&self, from: usize, edge_type: usize) -> Vec<usize> {
        exec_db!(
        self.db,
            "MATCH (a)-[r:R { id: {edge} }]->(b) \
            WHERE ID(a) = {from} \
            RETURN ID(b) ORDER BY ID(b)", {
                "from" => from,
                "edge" => edge_type
            }, {
                "ID(b)" => usize
            })
        .collect()
    }

    fn incoming_nodes(&self, to: usize, edge_type: usize) -> Vec<usize> {
        exec_db!(
        self.db,
            "MATCH (a)<-[r:R { id: {edge} }]-(b) \
            WHERE ID(a) = {to} \
            RETURN ID(b) ORDER BY ID(b)", {
                "to" => to,
                "edge" => edge_type
            }, {
                "ID(b)" => usize
            })
        .collect()
    }

    fn all_outgoing_nodes(&self, from: usize) -> Vec<usize> {
        exec_db!(self.db, "MATCH (a)-->(b) WHERE ID(a) = {from} RETURN ID(b) ORDER BY ID(b)", {
            "from" => from
        }, {
            "ID(b)" => usize
        })
        .collect()
    }

    fn all_incoming_nodes(&self, to: usize) -> Vec<usize> {
        exec_db!(self.db, "MATCH (a)<--(b) WHERE ID(a) = {to} RETURN ID(b) ORDER BY ID(b)", {
            "to" => to
        }, {
            "ID(b)" => usize
        })
        .collect()
    }

    fn into_dot(&self) -> String {
        let mut node_names = HashMap::new();
        let nodes: Vec<String> = exec_db!(self.db, "MATCH (n) RETURN ID(n), n.name ORDER BY ID(n)")
            .rows()
            .map(|r| {
                let id = r.get::<usize>("ID(n)").unwrap();
                let name = r
                    .get::<Option<String>>("n.name")
                    .unwrap()
                    .unwrap_or(id.to_string());
                let row_str = format!("    {} [ label = \"{}\" ]\n", id, name);
                node_names.insert(id, name);
                row_str
            })
            .collect();
        let relations: Vec<String> = exec_db!(
            self.db,
            "MATCH (a)-[r]->(b) RETURN ID(a), r.id, ID(b) ORDER BY ID(a)"
        )
        .rows()
        .map(|r| {
            let from = r.get::<usize>("ID(a)").unwrap();
            let edge_type = r.get::<usize>("r.id").unwrap();
            let to = r.get::<usize>("ID(b)").unwrap();
            format!(
                "    {} -> {} [ label = \"{}\" ]\n",
                from,
                to,
                node_names.get(&edge_type).unwrap()
            )
        })
        .collect();
        let mut dot: String = "digraph {\n".to_owned();
        for node in nodes {
            dot.push_str(node.as_str())
        }
        for relation in relations {
            dot.push_str(relation.as_str())
        }
        dot.push_str("}");
        dot
    }
}

/// While these tests connect to an actual external DB, it is still possible for them to run in
/// multi-threaded mode because each test generally creates its own nodes.
#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;
    use std::collections::HashSet;

    /// Default Neo4j 3.x instance to connect to. Note that the local password should be changed to
    /// dummy_password first. All tests in this section are ignored by default to allow tests to
    /// pass even when there is no local instance of Neo4j running.
    const TEST_DB_URI: &'static str = "http://neo4j:dummy_password@127.0.0.1:7474/db/data";

    /// Convert a vec of IDs to a set because Neo4j isn't guaranteed to create nodes with
    /// sequential IDs.
    fn ids_as_set(ids: Vec<usize>) -> HashSet<usize> {
        let mut set = HashSet::new();
        for id in ids {
            set.insert(id);
        }
        set
    }

    macro_rules! assert_unordered_eq {
        ($a:expr, $b:expr) => {
            assert_eq!(ids_as_set($a), ids_as_set($b));
        };
    }

    #[test]
    #[ignore]
    fn test_create() {
        bind_cypher_graph(TEST_DB_URI);
    }

    #[test]
    #[ignore]
    fn test_add_node() {
        bind_cypher_graph(TEST_DB_URI);
        let mut g = InjectionGraph {};
        let id = g.add_node();
        assert!(g.node_value(id).is_none());
        assert_eq!(g.node_name(id), None);
    }

    #[test]
    #[ignore]
    fn test_size() {
        bind_cypher_graph(TEST_DB_URI);
        let mut g = InjectionGraph {};
        let initial_size = g.size();
        g.add_node();
        // Because we're accessing the same instance of the cypher DB every time, we cannot
        // guarantee that another node won't be added in the meantime when tests run in parallel.
        // However, we can still test that the queries are returning successfully, at least.
        dbg!("Initial {} turned into {}", initial_size, g.size());
        assert!(g.size() >= initial_size + 1);
    }

    #[test]
    #[ignore]
    fn test_set_node_value() {
        bind_cypher_graph(TEST_DB_URI);
        let mut g = InjectionGraph {};
        let a_id = g.add_node();
        let v = Rc::new("5".to_string());
        g.set_node_value(a_id, Box::new(WeakWrapper::new(&v)));
        assert_eq!(unwrap_strong(g.node_value(a_id)), Some(v));
        assert_eq!(g.node_name(a_id), None);
    }

    #[test]
    #[ignore]
    fn test_retrieve_node_name() {
        bind_cypher_graph(TEST_DB_URI);
        let mut g = InjectionGraph {};
        let a_id = g.add_node();
        g.set_node_name(a_id, "A".to_string());
        assert_eq!(g.node_name(a_id), Some(Rc::new("A".to_string())));
    }

    #[test]
    #[ignore]
    fn test_retrieve_node_name_value() {
        bind_cypher_graph(TEST_DB_URI);
        let mut g = InjectionGraph {};
        let a_id = g.add_node();
        let v = Rc::new("5".to_string());
        g.set_node_name(a_id, "A".to_string());
        g.set_node_value(a_id, Box::new(WeakWrapper::new(&v)));
        assert_eq!(g.node_name(a_id), Some(Rc::new("A".to_string())));
        assert_eq!(unwrap_strong(g.node_value(a_id)), Some(v));
    }

    #[test]
    #[ignore]
    fn test_no_outgoing_node() {
        bind_cypher_graph(TEST_DB_URI);
        let mut g = InjectionGraph {};
        let a_id = g.add_node();
        assert_eq!(g.all_outgoing_nodes(a_id), Vec::<usize>::new());
        assert_eq!(g.outgoing_nodes(a_id, a_id), Vec::<usize>::new());
    }

    #[test]
    #[ignore]
    fn test_one_outgoing_node() {
        bind_cypher_graph(TEST_DB_URI);
        let mut g = InjectionGraph {};
        let a_id = g.add_node();
        let b_id = g.add_node();
        let edge_type = g.add_node();
        g.add_edge(a_id, edge_type, b_id);
        assert_eq!(g.all_outgoing_nodes(a_id), vec![b_id]);
        assert_eq!(g.outgoing_nodes(a_id, edge_type), vec![b_id]);
    }

    #[test]
    #[ignore]
    fn test_multiple_outgoing_nodes() {
        bind_cypher_graph(TEST_DB_URI);
        let mut g = InjectionGraph {};
        let a_id = g.add_node();
        let b_id = g.add_node();
        let c_id = g.add_node();
        let edge_type = g.add_node();
        g.add_edge(a_id, edge_type, b_id);
        g.add_edge(a_id, edge_type, c_id);
        assert_unordered_eq!(g.all_outgoing_nodes(a_id), vec![b_id, c_id]);
        assert_unordered_eq!(g.outgoing_nodes(a_id, edge_type), vec![b_id, c_id]);
    }

    #[test]
    #[ignore]
    fn test_outgoing_ignores_incoming_nodes() {
        bind_cypher_graph(TEST_DB_URI);
        let mut g = InjectionGraph {};
        let a_id = g.add_node();
        let b_id = g.add_node();
        let c_id = g.add_node();
        let d_id = g.add_node();
        let edge_type = g.add_node();
        g.add_edge(a_id, edge_type, b_id);
        g.add_edge(a_id, edge_type, d_id);
        g.add_edge(c_id, edge_type, a_id);
        assert_unordered_eq!(g.all_outgoing_nodes(a_id), vec![b_id, d_id]);
        assert_unordered_eq!(g.outgoing_nodes(a_id, edge_type), vec![b_id, d_id]);
    }

    #[test]
    #[ignore]
    fn test_outgoing_ignores_wrong_edge_type() {
        bind_cypher_graph(TEST_DB_URI);
        let mut g = InjectionGraph {};
        let a_id = g.add_node();
        let b_id = g.add_node();
        let c_id = g.add_node();
        let d_id = g.add_node();
        let edge_type1 = g.add_node();
        let edge_type2 = g.add_node();
        g.add_edge(a_id, edge_type1, b_id);
        g.add_edge(a_id, edge_type2, c_id);
        g.add_edge(a_id, edge_type1, d_id);
        assert_unordered_eq!(g.all_outgoing_nodes(a_id), vec![b_id, c_id, d_id]);
        assert_unordered_eq!(g.outgoing_nodes(a_id, edge_type1), vec![b_id, d_id]);
    }

    #[test]
    #[ignore]
    fn test_has_edge() {
        bind_cypher_graph(TEST_DB_URI);
        let mut g = InjectionGraph {};
        let a_id = g.add_node();
        let b_id = g.add_node();
        let edge_type1 = g.add_node();
        let edge_type2 = g.add_node();
        g.add_edge(a_id, edge_type1, b_id);
        assert!(g.has_edge(a_id, edge_type1, b_id));
        assert!(!g.has_edge(a_id, edge_type2, b_id));
        assert!(!g.has_edge(b_id, edge_type2, a_id));
    }

    #[test]
    #[ignore]
    fn test_no_incoming_node() {
        bind_cypher_graph(TEST_DB_URI);
        let mut g = InjectionGraph {};
        let a_id = g.add_node();
        assert_eq!(g.all_incoming_nodes(a_id), Vec::<usize>::new());
        assert_eq!(g.incoming_nodes(a_id, a_id), Vec::<usize>::new());
    }

    #[test]
    #[ignore]
    fn test_incoming_node() {
        bind_cypher_graph(TEST_DB_URI);
        let mut g = InjectionGraph {};
        let a_id = g.add_node();
        let b_id = g.add_node();
        let edge_type = g.add_node();
        g.add_edge(b_id, edge_type, a_id);
        assert_eq!(g.all_incoming_nodes(a_id), vec![b_id]);
        assert_eq!(g.incoming_nodes(a_id, edge_type), vec![b_id]);
    }

    #[test]
    #[ignore]
    fn test_multiple_incoming_nodes() {
        bind_cypher_graph(TEST_DB_URI);
        let mut g = InjectionGraph {};
        let a_id = g.add_node();
        let b_id = g.add_node();
        let c_id = g.add_node();
        let edge_type = g.add_node();
        g.add_edge(b_id, edge_type, a_id);
        g.add_edge(c_id, edge_type, a_id);
        assert_unordered_eq!(g.all_incoming_nodes(a_id), vec![b_id, c_id]);
        assert_unordered_eq!(g.incoming_nodes(a_id, edge_type), vec![b_id, c_id]);
    }

    #[test]
    #[ignore]
    fn test_incoming_ignores_outgoing_nodes() {
        bind_cypher_graph(TEST_DB_URI);
        let mut g = InjectionGraph {};
        let a_id = g.add_node();
        let b_id = g.add_node();
        let c_id = g.add_node();
        let d_id = g.add_node();
        let edge_type = g.add_node();
        g.add_edge(b_id, edge_type, a_id);
        g.add_edge(d_id, edge_type, a_id);
        g.add_edge(a_id, edge_type, c_id);
        assert_unordered_eq!(g.all_incoming_nodes(a_id), vec![b_id, d_id]);
        assert_unordered_eq!(g.incoming_nodes(a_id, edge_type), vec![b_id, d_id]);
    }

    #[test]
    #[ignore]
    fn test_incoming_ignores_wrong_edge_type() {
        bind_cypher_graph(TEST_DB_URI);
        let mut g = InjectionGraph {};
        let a_id = g.add_node();
        let b_id = g.add_node();
        let c_id = g.add_node();
        let d_id = g.add_node();
        let edge_type1 = g.add_node();
        let edge_type2 = g.add_node();
        g.add_edge(b_id, edge_type1, a_id);
        g.add_edge(c_id, edge_type2, a_id);
        g.add_edge(d_id, edge_type1, a_id);
        assert_unordered_eq!(g.all_incoming_nodes(a_id), vec![b_id, c_id, d_id]);
        assert_unordered_eq!(g.incoming_nodes(a_id, edge_type1), vec![b_id, d_id]);
    }

    #[test]
    #[ignore]
    fn test_into_dot() {
        bind_cypher_graph(TEST_DB_URI);
        let mut g = InjectionGraph {};
        let a_id = g.add_node();
        let b_id = g.add_node();
        let edge_type_id = g.add_node();
        g.set_node_name(b_id, "B node".to_owned());
        g.set_node_name(edge_type_id, "test attr".to_owned());
        g.add_edge(a_id, edge_type_id, b_id);

        let dot_representation = g.into_dot();
        print_graph_debug();
        assert!(dot_representation.starts_with("digraph"));
        assert!(dot_representation.contains(" [ label = \"B node\" ]"));
        // Like with the size test, we cannot guarantee that this is the first run, so we test only
        // that the query returns successfully
        assert!(
            dot_representation
                .matches(" [ label = \"test attr\" ]")
                .count()
                >= 2 // one label for the node, another for the edge
        );
    }
}
