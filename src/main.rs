use serde::Deserialize;

mod sankey_render;
mod text_render;

use std::error::Error;
use std::fs::File;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {
    /// Input file for nodes
    #[structopt(short = "n", long = "nodes", parse(from_os_str))]
    nodes: PathBuf,
    /// Input file for edges
    #[structopt(short = "e", long = "edges", parse(from_os_str))]
    edges: PathBuf,
    /// Output file, stdout if not present
    #[structopt(short = "o", long = "output", parse(from_os_str))]
    output: Option<PathBuf>,
}

fn main() -> Result<(), Box<Error>> {
    let opt = Opt::from_args();

    let nodes_file = File::open(opt.nodes)?;
    let mut nodes = Vec::new();
    for result in csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(nodes_file)
        .into_deserialize()
    {
        nodes.push(result?);
    }

    let edges_file = File::open(opt.edges)?;
    let mut edges = Vec::new();
    for result in csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(edges_file)
        .into_deserialize()
    {
        edges.push(result?);
    }

    let image = sankey_render::render_graph(nodes, edges);
    let output = opt.output.unwrap_or_else(|| PathBuf::from("output.png"));
    image.save(output)?;
    Ok(())
}

#[derive(Debug, Clone, Deserialize)]
struct Edge {
    pub source: String,
    pub target: String,
    pub value: u32,
}

#[derive(Debug, Clone, Deserialize)]
struct Node {
    pub name: String,
    pub value: u32,
    pub col: u32,
    pub row: u32,
}
