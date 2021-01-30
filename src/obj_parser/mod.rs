use anyhow::Result;
use std::{collections::HashMap, fs, path::Path};

use crate::{
    geometry::shape::{Group, Triangle},
    point::Point,
};

pub struct Parser {
    ignored: usize,
    vertices: Vec<Point>,
    groups: HashMap<String, Group>,
    selected_group: String,
}

impl Parser {
    fn new() -> Self {
        let default_group = Group::default();
        let mut groups = HashMap::new();
        groups.insert("default".to_string(), default_group);

        Self {
            ignored: 0,
            vertices: vec![Point::origin()],
            groups,
            selected_group: "default".to_string(),
        }
    }

    fn parse(&mut self, contents: &str) {
        for line in contents.lines() {
            self.parse_line(line);
        }
    }

    fn parse_line(&mut self, line: &str) {
        if let Some(first) = line.bytes().next() {
            match first {
                b'v' => {
                    let numbers: Vec<_> = line
                        .split(' ')
                        .skip(1)
                        .map(str::parse::<f64>)
                        .map(Result::unwrap)
                        .collect();
                    self.vertices
                        .push(Point::new(numbers[0], numbers[1], numbers[2]));
                }
                b'f' => {
                    let indices: Vec<_> = line
                        .split(' ')
                        .skip(1)
                        .map(str::parse::<usize>)
                        .map(Result::unwrap)
                        .collect();

                    for triangle in self.fan_triangulation(&indices) {
                        let group = self.groups.get_mut(&self.selected_group).unwrap();
                        group.add_child(Box::new(triangle));
                    }
                }
                b'g' => {
                    let name = line.split(' ').nth(1).unwrap();

                    self.selected_group = name.to_string();
                    self.groups.insert(name.to_string(), Group::default());
                }
                _ => {
                    self.ignored += 1;
                }
            }
        }
    }

    fn fan_triangulation(&self, vertices: &[usize]) -> Vec<Triangle> {
        let mut triangles = vec![];

        for i in 1..vertices.len() - 1 {
            let triangle = Triangle::new(
                self.vertices[vertices[0]],
                self.vertices[vertices[i]],
                self.vertices[vertices[i + 1]],
            );
            triangles.push(triangle);
        }

        triangles
    }

    pub fn as_group(&mut self) -> Group {
        if self.groups.len() == 1 {
            return self.groups.remove("default").unwrap();
        }

        let mut group = Group::default();

        for (_, child) in self.groups.drain().filter(|(_, g)| !g.children.is_empty()) {
            group.add_child(Box::new(child));
        }

        group
    }
}

pub fn parse_obj_file(path: &Path) -> Result<Parser> {
    let mut p = Parser::new();
    let contents = fs::read_to_string(path)?;
    p.parse(&contents);
    Ok(p)
}

#[cfg(test)]
mod tests {
    use crate::geometry::shape::Triangle;

    use super::*;

    #[test]
    fn ignore_unrecognized_lines() {
        let p = parse_obj_file(Path::new("./src/obj_parser/test_data/gibberish.obj")).unwrap();
        assert_eq!(p.ignored, 5);
    }

    #[test]
    fn parse_vertex_records() {
        let p = parse_obj_file(Path::new("./src/obj_parser/test_data/vertex_records.obj")).unwrap();
        assert_eq!(p.vertices.len(), 5);
        assert_eq!(p.vertices[1], Point::new(-1, 1, 0));
        assert_eq!(p.vertices[2], Point::new(-1.0, 0.5, 0.0));
        assert_eq!(p.vertices[3], Point::new(1, 0, 0));
        assert_eq!(p.vertices[4], Point::new(1, 1, 0));
    }

    #[test]
    fn parse_triangle_faces() {
        let parser =
            parse_obj_file(Path::new("./src/obj_parser/test_data/triangle_faces.obj")).unwrap();
        let g = parser.groups.get("default").unwrap();
        let t1 = g.children[0].as_any().downcast_ref::<Triangle>().unwrap();
        let t2 = g.children[1].as_any().downcast_ref::<Triangle>().unwrap();
        assert_eq!(t1.p1, parser.vertices[1]);
        assert_eq!(t1.p2, parser.vertices[2]);
        assert_eq!(t1.p3, parser.vertices[3]);
        assert_eq!(t2.p1, parser.vertices[1]);
        assert_eq!(t2.p2, parser.vertices[3]);
        assert_eq!(t2.p3, parser.vertices[4]);
    }

    #[test]
    fn parse_polygon() {
        let parser = parse_obj_file(Path::new(
            "./src/obj_parser/test_data/triangulate_polygons.obj",
        ))
        .unwrap();
        let g = parser.groups.get("default").unwrap();
        let t1 = g.children[0].as_any().downcast_ref::<Triangle>().unwrap();
        let t2 = g.children[1].as_any().downcast_ref::<Triangle>().unwrap();
        let t3 = g.children[2].as_any().downcast_ref::<Triangle>().unwrap();

        assert_eq!(t1.p1, parser.vertices[1]);
        assert_eq!(t1.p2, parser.vertices[2]);
        assert_eq!(t1.p3, parser.vertices[3]);
        assert_eq!(t2.p1, parser.vertices[1]);
        assert_eq!(t2.p2, parser.vertices[3]);
        assert_eq!(t2.p3, parser.vertices[4]);
        assert_eq!(t3.p1, parser.vertices[1]);
        assert_eq!(t3.p2, parser.vertices[4]);
        assert_eq!(t3.p3, parser.vertices[5]);
    }

    #[test]
    fn parse_triangles_in_groups() {
        let parser = parse_obj_file(Path::new("./src/obj_parser/test_data/triangles.obj")).unwrap();

        let g1 = parser.groups.get("FirstGroup").unwrap();
        let g2 = parser.groups.get("SecondGroup").unwrap();

        let t1 = &g1.children[0].as_any().downcast_ref::<Triangle>().unwrap();
        let t2 = g2.children[0].as_any().downcast_ref::<Triangle>().unwrap();

        assert_eq!(t1.p1, parser.vertices[1]);
        assert_eq!(t1.p2, parser.vertices[2]);
        assert_eq!(t1.p3, parser.vertices[3]);
        assert_eq!(t2.p1, parser.vertices[1]);
        assert_eq!(t2.p2, parser.vertices[3]);
        assert_eq!(t2.p3, parser.vertices[4]);
    }

    #[test]
    fn convert_obj_file_to_group() {
        let mut parser =
            parse_obj_file(Path::new("./src/obj_parser/test_data/triangles.obj")).unwrap();

        let g = parser.as_group();

        let g1 = g.children[0].as_any().downcast_ref::<Group>().unwrap();
        let g2 = g.children[1].as_any().downcast_ref::<Group>().unwrap();

        let t1 = g1.children[0].as_any().downcast_ref::<Triangle>().unwrap();
        let t2 = g2.children[0].as_any().downcast_ref::<Triangle>().unwrap();

        assert_eq!(t1.p1, parser.vertices[1]);
        assert_eq!(t2.p1, parser.vertices[1]);
        assert!((t1.p2 == parser.vertices[2] || t2.p2 == parser.vertices[2]));
        assert!((t1.p3 == parser.vertices[3] || t2.p3 == parser.vertices[3]));
        assert!((t1.p3 == parser.vertices[4] || t2.p3 == parser.vertices[4]));
    }
}
