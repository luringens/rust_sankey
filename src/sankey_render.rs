use crate::{text_render::TextRenderer, Edge, Node};
use image::{DynamicImage, ImageBuffer, Rgba};
use std::{collections::HashMap, ops::DerefMut};

#[derive(Debug, Clone)]
struct PositionedNode {
    pub name: String,
    pub value: u32,
    pub col: u32,
    pub row: u32,
    pub x1: u32,
    pub x2: u32,
    pub y1: u32,
    pub y2: u32,
    pub used_left: u32,
    pub used_right: u32,
}

impl PositionedNode {
    pub fn from_node(n: &Node, x1: u32, x2: u32, y1: u32, y2: u32) -> Self {
        Self {
            name: n.name.clone(),
            value: n.value,
            col: n.col,
            row: n.row,
            x1,
            x2,
            y1,
            y2,
            used_left: 0,
            used_right: 0,
        }
    }
}

pub(crate) fn render_graph(nodes: Vec<Node>, edges: Vec<Edge>) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let image_width: u32 = 600;
    let image_height: u32 = 600;
    let padding: u32 = 14;
    let node_width: u32 = 10;

    // Calculate a bunch of handy values.
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
    let height_per_value: f32 = {
        let padding_space = (max_height_cols as u32 - 1 + 2) * padding;
        (image_height - padding_space) as f32 / max_height as f32
    };

    let width = nodes.iter().map(|node| node.col).max().expect("No nodes?");
    let col_separation = (image_width - 2 * padding - node_width) / width;

    let mut image = DynamicImage::new_rgba8(image_width, image_height).to_rgba();

    // Find the positions of all the nodes.
    let mut nodes = position_nodes(
        nodes,
        width,
        padding,
        node_width,
        col_separation,
        height_per_value,
    );

    render_edges(edges, &mut nodes, &mut image, height_per_value);
    render_nodes(nodes.values(), &mut image, padding, image_width, node_width);

    image
}

fn position_nodes(
    mut nodes: Vec<Node>,
    cols: u32,
    padding: u32,
    node_width: u32,
    col_separation: u32,
    height_per_value: f32,
) -> HashMap<String, PositionedNode> {
    nodes.sort_unstable_by_key(|e| e.row);
    nodes.sort_by_key(|e| e.col);

    let mut positioned = HashMap::new();
    for col in 0..=cols {
        let mut prev_y = padding;

        for node in nodes.iter().filter(|node| node.col == col) {
            let x1 = padding + node.col * col_separation;
            let x2 = x1 + node_width;
            let y1 = prev_y;
            let y2 = y1 + (node.value as f32 * height_per_value) as u32;
            prev_y = y2 + padding;
            positioned.insert(
                node.name.clone(),
                PositionedNode::from_node(node, x1, x2, y1, y2),
            );
        }
    }
    positioned
}

fn render_nodes<'a, T>(
    nodes: impl Iterator<Item = &'a PositionedNode>,
    image: &mut ImageBuffer<Rgba<u8>, T>,
    padding: u32,
    image_width: u32,
    node_width: u32,
) where
    T: DerefMut<Target = [<Rgba<u8> as image::Pixel>::Subpixel]>,
{
    let textrenderer = TextRenderer::default();
    for node in nodes.into_iter() {
        for (x, y) in (node.y1..=node.y2).flat_map(|y| (node.x1..=node.x2).map(move |x| (x, y))) {
            image.put_pixel(x, y, image::Rgba([0u8, 127u8, 255u8, 255u8]));
        }

        let text = format!("{}: {}", node.name, node.value);
        let y_mid = node.y1 + (node.y2 - node.y1) / 2;
        if node.x2 + 2 * padding > image_width {
            textrenderer.render_text(&text, node.x1 - node_width * 3, y_mid, true, image);
        } else {
            textrenderer.render_text(&text, node.x1, y_mid, false, image);
        }
    }
}
fn render_edges<T>(
    mut edges: Vec<Edge>,
    nodes: &mut HashMap<String, PositionedNode>,
    image: &mut ImageBuffer<Rgba<u8>, T>,
    height_per_value: f32,
) where
    T: DerefMut<Target = [<Rgba<u8> as image::Pixel>::Subpixel]>,
{
    edges.sort_unstable_by_key(|e| nodes[&e.target].y1);
    edges.sort_by_key(|e| nodes[&e.source].y1);
    for edge in edges.iter() {
        nodes.get_mut(&edge.source).unwrap().used_right += edge.value;
        nodes.get_mut(&edge.target).unwrap().used_left += edge.value;
        let source = &nodes[&edge.source];
        let target = &nodes[&edge.target];
        let x1 = source.x2 + 1;
        let x2 = target.x1 - 1;
        let y1_top =
            source.y1 + ((source.used_right - edge.value) as f32 * height_per_value) as u32;
        let y1_bot = y1_top + (edge.value as f32 * height_per_value) as u32;
        let y2_top = target.y1 + ((target.used_left - edge.value) as f32 * height_per_value) as u32;
        let y2_bot = y2_top + (edge.value as f32 * height_per_value) as u32;

        let mut top_line = get_line((x1, y1_top), (x2, y2_top)).into_iter();
        let mut bot_line = get_line((x1, y1_bot), (x2, y2_bot)).into_iter();
        let mut top = top_line.next();
        let mut bot = bot_line.next();

        while top.is_some() || bot.is_some() {
            if top.is_none() || top.unwrap().0 > bot.unwrap().0 {
                let (x, y) = bot.unwrap();
                image.put_pixel(x, y, Rgba([125u8, 190u8, 255u8, 255u8]));
                bot = bot_line.next();
                continue;
            }
            if bot.is_none() || top.unwrap().0 < bot.unwrap().0 {
                let (x, y) = top.unwrap();
                image.put_pixel(x, y, Rgba([125u8, 190u8, 255u8, 255u8]));
                top = top_line.next();
                continue;
            }

            for (x, y) in get_line(top.unwrap(), bot.unwrap()) {
                image.put_pixel(x, y, Rgba([125u8, 190u8, 255u8, 255u8]));
            }
            top = top_line.next();
            bot = bot_line.next();
        }
    }
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
