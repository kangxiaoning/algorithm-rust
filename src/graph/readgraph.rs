use super::Graph;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn parse_line(line: String) -> (usize, usize) {
    let mut header = line.split_whitespace();
    let v1: usize = header.next().unwrap().parse().unwrap();
    let v2: usize = header.next().unwrap().parse().unwrap();
    (v1, v2)
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

    // 读取第一条边的信息
    for _ in 0..e {
        let mut line = String::new();
        let _ = reader.read_line(&mut line);

        let (v1, v2) = parse_line(line);
        assert!(v1 < v && v2 < v);
        graph.add_edge(v1, v2);
    }

    Ok(())
}
