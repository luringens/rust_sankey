mod text_render;

use image::DynamicImage;
use std::collections::HashMap;
use text_render::TextRenderer;

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

    render_graph(nodes, edges);
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

#[derive(Debug, Clone)]
struct PositionedNode {
    pub x1: u32,
    pub x2: u32,
    pub y1: u32,
    pub y2: u32,
    pub l: u32,
    pub r: u32,
    pub value: u32,
}

fn render_graph(mut nodes: Vec<Node>, mut edges: Vec<Edge>) {
    nodes.sort_unstable_by_key(|e| e.row);
    nodes.sort_by_key(|e| e.col);

    let (max_height_col, max_height) = {
        let mut heights = HashMap::new();
        for node in nodes.iter() {
            let value = heights.entry(node.col).or_insert(0);
            *value += node.value + 1;
        }

        let (a, b) = heights.iter().max_by_key(|(_, v)| *v).expect("No nodes?");
        (*a, *b)
    };

    let max_height_cols = nodes.iter().filter(|e| e.col == max_height_col).count();

    let width = nodes.iter().map(|node| node.col).max().expect("No nodes?");

    let image_width: u32 = 600;
    let image_height: u32 = 600;
    let padding: u32 = 14;
    let node_width: u32 = 10;
    let height_per_value: f32 = {
        let padding_space = (max_height_cols as u32 - 1 + 2) * padding;
        (image_height - padding_space) as f32 / max_height as f32
    };
    let col_separation = (image_width - 2 * padding - node_width) / width;

    let mut image = DynamicImage::new_rgba8(image_width, image_height).to_rgba();

    let mut rects: HashMap<String, PositionedNode> = HashMap::new();
    for col in 0..=width {
        let mut prev_y = padding;

        for node in nodes.iter().filter(|node| node.col == col) {
            let x1 = padding + node.col * col_separation;
            let x2 = x1 + node_width;
            let y1 = prev_y;
            let y2 = y1 + (node.value as f32 * height_per_value) as u32;
            prev_y = y2 + padding;
            let (l, r) = (0, 0);
            let n = node.name.clone();
            let value = node.value;
            rects.insert(
                n,
                PositionedNode {
                    x1,
                    x2,
                    y1,
                    y2,
                    l,
                    r,
                    value,
                },
            );
        }
    }

    edges.sort_unstable_by_key(|e| rects[&e.target].y1);
    edges.sort_by_key(|e| rects[&e.source].y1);

    for edge in edges.iter() {
        rects.get_mut(&edge.source).unwrap().r += edge.value;
        rects.get_mut(&edge.target).unwrap().l += edge.value;
        let source = &rects[&edge.source];
        let target = &rects[&edge.target];
        let x1 = source.x2 + 1;
        let x2 = target.x1 - 1;
        let y1_top = source.y1 + ((source.r - edge.value) as f32 * height_per_value) as u32;
        let y1_bot = y1_top + (edge.value as f32 * height_per_value) as u32;
        let y2_top = target.y1 + ((target.l - edge.value) as f32 * height_per_value) as u32;
        let y2_bot = y2_top + (edge.value as f32 * height_per_value) as u32;

        let mut top_line = get_line((x1, y1_top), (x2, y2_top)).into_iter();
        let mut bot_line = get_line((x1, y1_bot), (x2, y2_bot)).into_iter();
        let mut top = top_line.next();
        let mut bot = bot_line.next();

        while top.is_some() || bot.is_some() {
            if top.is_none() || top.unwrap().0 > bot.unwrap().0 {
                let (x, y) = bot.unwrap();
                image.put_pixel(x, y, image::Rgba([125u8, 190u8, 255u8, 255u8]));
                bot = bot_line.next();
                continue;
            }
            if bot.is_none() || top.unwrap().0 < bot.unwrap().0 {
                let (x, y) = top.unwrap();
                image.put_pixel(x, y, image::Rgba([125u8, 190u8, 255u8, 255u8]));
                top = top_line.next();
                continue;
            }

            for (x, y) in get_line(top.unwrap(), bot.unwrap()) {
                image.put_pixel(x, y, image::Rgba([125u8, 190u8, 255u8, 255u8]));
            }
            top = top_line.next();
            bot = bot_line.next();
        }
    }

    let textrenderer = TextRenderer::default();
    for (name, rect) in rects.iter() {
        for (x, y) in (rect.y1..=rect.y2).flat_map(|y| (rect.x1..=rect.x2).map(move |x| (x, y))) {
            image.put_pixel(x, y, image::Rgba([0u8, 127u8, 255u8, 255u8]));
        }

        let text = format!("{}: {}", name, rect.value);
        let y_mid = rect.y1 + (rect.y2 - rect.y1) / 2;
        if rect.x2 + 2 * padding > image_width {
            textrenderer.render_text(&text, rect.x1 - node_width * 3, y_mid, true, &mut image);
        } else {
            textrenderer.render_text(&text, rect.x1, y_mid, false, &mut image);
        }
    }

    image.save("./output.png").expect("Failed to save image");
}

/// http://www.roguebasin.com/index.php?title=Bresenham%27s_Line_Algorithm#Rust
fn get_line(a: (u32, u32), b: (u32, u32)) -> Vec<(u32, u32)> {
    let mut points = Vec::<(u32, u32)>::new();
    let mut x1 = a.0 as i32;
    let mut y1 = a.1 as i32;
    let mut x2 = b.0 as i32;
    let mut y2 = b.1 as i32;
    let is_steep = (y2 - y1).abs() > (x2 - x1).abs();
    if is_steep {
        std::mem::swap(&mut x1, &mut y1);
        std::mem::swap(&mut x2, &mut y2);
    }
    let mut reversed = false;
    if x1 > x2 {
        std::mem::swap(&mut x1, &mut x2);
        std::mem::swap(&mut y1, &mut y2);
        reversed = true;
    }
    let dx = x2 - x1;
    let dy = (y2 - y1).abs();
    let mut err = dx / 2;
    let mut y = y1;
    let ystep: i32;
    if y1 < y2 {
        ystep = 1;
    } else {
        ystep = -1;
    }
    for x in x1..(x2 + 1) {
        if is_steep {
            points.push((y as u32, x as u32));
        } else {
            points.push((x as u32, y as u32));
        }
        err -= dy;
        if err < 0 {
            y += ystep;
            err += dx;
        }
    }

    if reversed {
        for i in 0..(points.len() / 2) {
            let end = points.len() - 1;
            points.swap(i, end - i);
        }
    }
    points
}
