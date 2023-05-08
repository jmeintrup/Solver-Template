use crate::graph::Graph;
use std::io::{BufRead, ErrorKind};

pub struct UniversalReader<T: BufRead>(pub T);

impl<T: BufRead> TryFrom<UniversalReader<T>> for Graph {
    type Error = std::io::Error;

    fn try_from(reader: UniversalReader<T>) -> Result<Self, Self::Error> {
        let reader = reader.0;
        let mut graph: Option<Graph> = None;
        let mut order: Option<u32> = None;
        for line in reader.lines() {
            let line = line?;
            let elements: Vec<_> = line.split(' ').collect();
            match elements[0].chars().next().unwrap() {
                'c' | '%' => {
                    // comments
                }
                'p' => {
                    order = Some(parse_order_dimacs(&elements)?);
                    graph = Some(Graph::new(order.unwrap()));
                }
                _ => match graph.as_mut() {
                    Some(graph) => {
                        let len = elements.len();
                        let u = parse_vertex(elements[len - 1], order.unwrap())?;
                        let v = parse_vertex(elements[len - 2], order.unwrap())?;
                        graph.add_edge(u, v);
                    }
                    None => {
                        order = Some(parse_order_mtx(&elements)?);
                        graph = Some(Graph::new(order.unwrap()));
                    }
                },
            };
        }
        match graph {
            Some(graph) => Ok(graph),
            None => Err(std::io::Error::new(
                ErrorKind::Other,
                "No graph created during parsing",
            )),
        }
    }
}

fn parse_vertex(v: &str, order: u32) -> Result<u32, std::io::Error> {
    match v.parse::<u32>() {
        Ok(u) => {
            if u == 0 || u > order {
                Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Invalid vertex label",
                ))
            } else {
                Ok(u - 1)
            }
        }
        Err(_) => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid vertex label",
        )),
    }
}

fn parse_order_dimacs(elements: &[&str]) -> Result<u32, std::io::Error> {
    if elements.len() < 3 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid line received starting with p",
        ));
    }
    match elements[2].parse::<u32>() {
        Ok(order) => Ok(order),
        Err(_) => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid order of graph",
        )),
    }
}

fn parse_order_mtx(elements: &[&str]) -> Result<u32, std::io::Error> {
    if elements.len() < 3 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid line received when parsing mtx header",
        ));
    }
    match elements[0].parse::<u32>() {
        Ok(order) => Ok(order),
        Err(_) => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid order of graph",
        )),
    }
}
