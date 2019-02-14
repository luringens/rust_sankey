mod text_render;

use image::DynamicImage;
use petgraph::Graph;
use std::collections::HashMap;
use text_render::TextRenderer;

fn main() {
    let mut image = DynamicImage::new_rgba8(500, 250).to_rgba();
    let textrenderer = TextRenderer::default();
    let text = "Hello, world!";
    textrenderer.render_text(text, &mut image);

    // Save the image to a png file
    image.save("image_example.png").unwrap();
    println!("Generated: image_example.png");

    render_graph();
}

struct Entry {
    pub source: String,
    pub target: String,
    pub value: u32,
}

fn render_graph() {
    let sample_data = [
        Entry {
            source: "Wages".to_owned(),
            target: "Budget".to_owned(),
            value: 2000,
        },
        Entry {
            source: "Interest".to_owned(),
            target: "Budget".to_owned(),
            value: 25,
        },
        Entry {
            source: "Budget".to_owned(),
            target: "Taxes".to_owned(),
            value: 500,
        },
        Entry {
            source: "Budget".to_owned(),
            target: "Housing".to_owned(),
            value: 450,
        },
        Entry {
            source: "Budget".to_owned(),
            target: "Food".to_owned(),
            value: 310,
        },
        Entry {
            source: "Budget".to_owned(),
            target: "Transportation".to_owned(),
            value: 205,
        },
        Entry {
            source: "Budget".to_owned(),
            target: "Health Care".to_owned(),
            value: 400,
        },
        Entry {
            source: "Budget".to_owned(),
            target: "Other Necessities".to_owned(),
            value: 160,
        },
    ];
    let mut image = DynamicImage::new_rgba8(500, 250).to_rgba();

    const padding: u32 = 18;

    let mut graph = Graph::<&str, u32>::new();
    let mut nodes = HashMap::new();
    for entry in &sample_data {
        let source = *nodes
            .entry(&entry.source)
            .or_insert(graph.add_node(entry.source.as_ref()));
        let target = *nodes
            .entry(&entry.target)
            .or_insert(graph.add_node(entry.target.as_ref()));
        graph.add_edge(source, target, entry.value);
    }

    let mut starts: Vec<(usize, petgraph::graph::NodeIndex)> = graph
        .externals(petgraph::Direction::Incoming)
        .map(|n| (0, n))
        .collect();

    // Find height and width of graph for positioning
    // Bin nodes into columns
    // Find height of "1 unit" per column, pick highest as the common one
    // "Untangle" graph by ordering in a sensible manner
    // Draw elements on image.
}
