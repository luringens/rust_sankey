mod sankey_render;
mod text_render;

fn main() {
    // Test data - TEMP
    let nodes: Vec<Node> = [
        Node {
            name: "Wages".to_owned(),
            value: 2000,
            col: 0,
            row: 0,
        },
        Node {
            name: "Interest".to_owned(),
            value: 25,
            col: 0,
            row: 1,
        },
        Node {
            name: "Budget".to_owned(),
            value: 2025,
            col: 1,
            row: 0,
        },
        Node {
            name: "Taxes".to_owned(),
            value: 500,
            col: 2,
            row: 0,
        },
        Node {
            name: "Housing".to_owned(),
            value: 450,
            col: 2,
            row: 1,
        },
        Node {
            name: "Food".to_owned(),
            value: 310,
            col: 2,
            row: 2,
        },
        Node {
            name: "Transportation".to_owned(),
            value: 205,
            col: 2,
            row: 3,
        },
        Node {
            name: "Health Care".to_owned(),
            value: 400,
            col: 2,
            row: 4,
        },
        Node {
            name: "Other Necessities".to_owned(),
            value: 160,
            col: 2,
            row: 5,
        },
    ]
    .into_iter()
    .cloned()
    .collect();

    let edges: Vec<Edge> = [
        Edge {
            source: "Wages".to_owned(),
            target: "Budget".to_owned(),
            value: 2000,
        },
        Edge {
            source: "Interest".to_owned(),
            target: "Budget".to_owned(),
            value: 25,
        },
        Edge {
            source: "Budget".to_owned(),
            target: "Taxes".to_owned(),
            value: 500,
        },
        Edge {
            source: "Budget".to_owned(),
            target: "Housing".to_owned(),
            value: 450,
        },
        Edge {
            source: "Budget".to_owned(),
            target: "Food".to_owned(),
            value: 310,
        },
        Edge {
            source: "Budget".to_owned(),
            target: "Transportation".to_owned(),
            value: 205,
        },
        Edge {
            source: "Budget".to_owned(),
            target: "Health Care".to_owned(),
            value: 400,
        },
        Edge {
            source: "Budget".to_owned(),
            target: "Other Necessities".to_owned(),
            value: 160,
        },
    ]
    .into_iter()
    .cloned()
    .collect();

    sankey_render::render_graph(nodes, edges);
}

#[derive(Debug, Clone)]
struct Edge {
    pub source: String,
    pub target: String,
    pub value: u32,
}

#[derive(Debug, Clone)]
struct Node {
    pub name: String,
    pub value: u32,
    pub col: u32,
    pub row: u32,
}
