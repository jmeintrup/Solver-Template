use basic_graph_lib::graph::Graph;
use basic_graph_lib::io::UniversalReader;

fn main() -> std::io::Result<()> {
    let stdin = std::io::stdin();
    let reader = UniversalReader(stdin.lock());
    let graph = Graph::try_from(reader)?;
    println!("{:?}", graph);
    Ok(())
}
