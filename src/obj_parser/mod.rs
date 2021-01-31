use anyhow::Result;
use std::{
    collections::HashMap,
    f64::{INFINITY, NEG_INFINITY},
    fs,
    path::Path,
};

use crate::{
    geometry::shape::{Group, SmoothTriangle, Triangle},
    point::Point,
    vector::Vector,
};

pub struct Parser {
    ignored: usize,
    vertices: Vec<Point>,
    vertex_normals: Vec<Vector>,
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
            vertex_normals: vec![Vector::new(0, 0, 0)],
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
        let mut items = line.split_ascii_whitespace();
        let kind = items.next();
        if let Some(kind) = kind {
            match kind {
                "v" => {
                    let numbers: Vec<_> =
                        items.map(str::parse::<f64>).map(Result::unwrap).collect();
                    self.vertices
                        .push(Point::new(numbers[0], numbers[1], numbers[2]));
                }

                "vn" => {
                    let numbers: Vec<_> =
                        items.map(str::parse::<f64>).map(Result::unwrap).collect();
                    self.vertex_normals
                        .push(Vector::new(numbers[0], numbers[1], numbers[2]));
                }
                "f" => {
                    if !line.contains('/') {
                        let indices: Vec<_> =
                            items.map(str::parse::<usize>).map(Result::unwrap).collect();

                        for triangle in self.fan_triangulation(&indices) {
                            let group = self.groups.get_mut(&self.selected_group).unwrap();
                            group.add_child(Box::new(triangle));
                        }
                    } else {
                        let faces: Vec<_> = items
                            .map(|item| {
                                let mut split = item.split('/');
                                (split.next().unwrap(), split.last().unwrap())
                            })
                            .map(|(index, normal)| {
                                (
                                    str::parse::<usize>(index).unwrap(),
                                    str::parse::<usize>(normal).unwrap(),
                                )
                            })
                            .collect();
                        for triangle in self.smooth_fan_triangulation(&faces) {
                            let group = self.groups.get_mut(&self.selected_group).unwrap();
                            group.add_child(Box::new(triangle));
                        }
                    }
                }
                "g" => {
                    let name = items.next().unwrap();

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

    fn smooth_fan_triangulation(&self, indexes: &[(usize, usize)]) -> Vec<SmoothTriangle> {
        let mut triangles = vec![];

        for i in 1..indexes.len() - 1 {
            let triangle = SmoothTriangle::new(
                self.vertices[indexes[0].0],
                self.vertices[indexes[i].0],
                self.vertices[indexes[i + 1].0],
                self.vertex_normals[indexes[0].1],
                self.vertex_normals[indexes[i].1],
                self.vertex_normals[indexes[i + 1].1],
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

    pub fn print_bounds(&self) {
        let mut min_x = INFINITY;
        let mut max_x = NEG_INFINITY;
        let mut min_y = INFINITY;
        let mut max_y = NEG_INFINITY;
        let mut min_z = INFINITY;
        let mut max_z = NEG_INFINITY;

        for vertex in &self.vertices {
            if vertex.x < min_x {
                min_x = vertex.x;
            }
            if vertex.x > max_x {
                max_x = vertex.x;
            }
            if vertex.y < min_y {
                min_y = vertex.y;
            }
            if vertex.y > max_y {
                max_y = vertex.y;
            }
            if vertex.z < min_z {
                min_z = vertex.z;
            }
            if vertex.z > max_z {
                max_z = vertex.z;
            }
        }

        println!("Model bounds:");
        println!("x: ({},{})", min_x, max_x);
        println!("y: ({},{})", min_y, max_y);
        println!("z: ({},{})", min_z, max_z);
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
    use crate::{
        geometry::shape::{SmoothTriangle, Triangle},
        vector::Vector,
    };

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

    #[test]
    fn parse_vertex_normals() {
        let parser =
            parse_obj_file(Path::new("./src/obj_parser/test_data/vertex_normals.obj")).unwrap();
        assert_eq!(parser.vertex_normals[1], Vector::new(0, 0, 1));
        assert_eq!(parser.vertex_normals[2], Vector::new(0.707, 0.0, -0.707));
        assert_eq!(parser.vertex_normals[3], Vector::new(1, 2, 3));
    }

    #[test]
    fn parse_faces_with_normals() {
        let parser = parse_obj_file(Path::new(
            "./src/obj_parser/test_data/faces_with_normals.obj",
        ))
        .unwrap();

        let g = parser.groups.get("default").unwrap();
        let t1 = g.children[0]
            .as_any()
            .downcast_ref::<SmoothTriangle>()
            .unwrap();
        let t2 = g.children[1]
            .as_any()
            .downcast_ref::<SmoothTriangle>()
            .unwrap();

        assert_eq!(t1.p1, parser.vertices[1]);
        assert_eq!(t1.p2, parser.vertices[2]);
        assert_eq!(t1.p3, parser.vertices[3]);
        assert_eq!(t1.n1, parser.vertex_normals[3]);
        assert_eq!(t1.n2, parser.vertex_normals[1]);
        assert_eq!(t1.n3, parser.vertex_normals[2]);
        assert_eq!(t1, t2);
    }

    #[test]
    fn test_parse_line() {
        let s = "v  7.0000 0.0000 12.0000";
        let mut parser = Parser::new();
        parser.parse_line(s);
    }
}
