use std::collections::HashSet;

use swc_core::atoms::Atom;
use webpack_analyzer_v2::dependency_graph::DependencyGraph;
use webpack_analyzer_v2::module::WebpackModule;

fn module(id: &str) -> WebpackModule {
    WebpackModule::new(Atom::from(id), String::from("/* stub */"))
}

#[test]
fn ignore_missing_dependencies_in_reachability() {
    let mut graph = DependencyGraph::new();
    let mut a = module("A");
    // Reference to a missing module "MISSING" should not be traversed
    a.add_dependency(Atom::from("MISSING"));
    graph.add_module(a);

    let reachable = graph.get_reachable_modules(&Atom::from("A"));
    let mut expected = HashSet::new();
    expected.insert(Atom::from("A"));
    assert_eq!(reachable, expected, "Reachability should exclude missing deps");
}

#[test]
fn cycles_do_not_inflate_reachability() {
    let mut graph = DependencyGraph::new();
    let mut a = module("A");
    let mut b = module("B");
    a.add_dependency(Atom::from("B"));
    b.add_dependency(Atom::from("A"));
    graph.add_module(a);
    graph.add_module(b);

    let reachable = graph.get_reachable_modules(&Atom::from("A"));
    assert_eq!(reachable.len(), 2);
    assert!(reachable.contains(&Atom::from("A")));
    assert!(reachable.contains(&Atom::from("B")));
}

#[test]
fn multiple_entries_union_of_reachability() {
    let mut graph = DependencyGraph::new();
    let mut a = module("A");
    let b = module("B");
    a.add_dependency(Atom::from("B"));
    graph.add_module(a);
    graph.add_module(b);

    let mut c = module("C");
    let d = module("D");
    c.add_dependency(Atom::from("D"));
    graph.add_module(c);
    graph.add_module(d);

    let reachable = graph.get_reachable_from_multiple(&[Atom::from("A"), Atom::from("C")]);
    assert_eq!(reachable.len(), 4);
    for id in ["A", "B", "C", "D"].iter() {
        assert!(reachable.contains(&Atom::from(*id)));
    }
}


