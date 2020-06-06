use super::{Graph, WeightedGraph};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;

fn parse_line(line: String) -> (usize, usize) {
    let mut header = line.split_whitespace();
    let v1: usize = header.next().unwrap().parse().unwrap();
    let v2: usize = header.next().unwrap().parse().unwrap();
    (v1, v2)
}

fn parse_weighted_edge<T>(line: String) -> (usize, usize, T)
where
    T: FromStr,
{
    let mut header = line.split_whitespace();
    let v1: usize = header.next().unwrap().parse().unwrap();
    let v2: usize = header.next().unwrap().parse().unwrap();
    let weight = match header.next().unwrap().parse() {
        Ok(v) => v,
        Err(_) => panic!("parse weight failed"),
    };
    (v1, v2, weight)
}

pub fn read<P, G>(graph: &mut G, filename: P) -> Result<(), io::Error>
where
    P: AsRef<Path>,
    G: Graph,
{
    // 读取文件内容到 reader
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);

    // 第一行读取图中的节点个数和边的个数
    let mut header = String::new();
    let _ = reader.read_line(&mut header);
    let (v, e) = parse_line(header);

    assert_eq!(v, graph.v());

    // 读取每一条边的信息
    for _ in 0..e {
        let mut line = String::new();
        let _ = reader.read_line(&mut line);

        let (v1, v2) = parse_line(line);
        assert!(v1 < v && v2 < v);
        graph.add_edge(v1, v2);
    }

    Ok(())
}

pub fn read_weighted_graph<P, G, T>(graph: &mut G, filename: P) -> Result<(), io::Error>
where
    P: AsRef<Path>,
    G: WeightedGraph<T>,
    T: FromStr,
{
    // 读取文件内容到 reader
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);

    // 第一行读取图中的节点个数和边的个数
    let mut header = String::new();
    let _ = reader.read_line(&mut header);
    let (v, e) = parse_line(header);

    assert_eq!(v, graph.v());

    // 读取每一条边的信息
    for _ in 0..e {
        let mut line = String::new();
        let _ = reader.read_line(&mut line);

        let (v1, v2, weight) = parse_weighted_edge(line);
        assert!(v1 < v && v2 < v);
        graph.add_edge(v1, v2, weight);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::{DenseGraph, SparseGraph};
    use crate::graph::{DenseWeightedGraph, SparseWeightedGraph};

    #[test]
    fn parse() {
        // parse line
        let line = "8 10".to_string();
        let (v1, v2) = parse_line(line);
        assert_eq!(v1, 8);
        assert_eq!(v2, 10);

        // parse weighted edge
        let line = "8 10 2.2".to_string();
        let (v1, v2, weight): (usize, usize, f32) = parse_weighted_edge(line);
        assert_eq!(v1, 8);
        assert_eq!(v2, 10);
        assert_eq!(weight, 2.2);

        let line = "8 10 2".to_string();
        let (v1, v2, weight): (usize, usize, usize) = parse_weighted_edge(line);
        assert_eq!(v1, 8);
        assert_eq!(v2, 10);
        assert_eq!(weight, 2);

        let line = "8 10 abc".to_string();
        let (v1, v2, weight): (usize, usize, String) = parse_weighted_edge(line);
        assert_eq!(v1, 8);
        assert_eq!(v2, 10);
        assert_eq!(weight, "abc".to_string());
    }

    #[test]
    #[should_panic(expected = "parse weight failed")]
    fn parse_panic() {
        let line = "8 10 abc".to_string();
        let (v1, v2, weight): (usize, usize, f32) = parse_weighted_edge(line);
        assert_eq!(v1, 8);
        assert_eq!(v2, 10);
        // assert_eq!(weight, "abc");
    }

    #[test]
    fn read_graph() {
        // read unweighted graph
        let filename = Path::new("./src/files/graph/test1.txt");
        let mut g = SparseGraph::new(13, false);
        let result = read(&mut g, filename).unwrap();
        assert_eq!(result, ());

        let filename = Path::new("./src/files/graph/test1.txt");
        let mut g = DenseGraph::new(13, false);
        let result = read(&mut g, filename).unwrap();
        assert_eq!(result, ());

        // read weighted graph
        let filename = Path::new("./src/files/graph/test3.txt");
        let mut g: SparseWeightedGraph<f32> = SparseWeightedGraph::new(8, false);
        let result = read_weighted_graph(&mut g, filename).unwrap();
        assert_eq!(result, ());

        let filename = Path::new("./src/files/graph/test3.txt");
        let mut g: DenseWeightedGraph<f32> = DenseWeightedGraph::new(8, false);
        let result = read_weighted_graph(&mut g, filename).unwrap();
        assert_eq!(result, ());
    }

    #[test]
    #[should_panic]
    fn read_graph_panic() {
        // read unweighted graph
        // No such file or directory
        let filename = Path::new("./src/files/graph/no_such_file.txt");
        let mut g = SparseGraph::new(13, false);
        read(&mut g, filename).unwrap();
    }
}
