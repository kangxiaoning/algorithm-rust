mod graph;
mod heap;
mod leetcode;
mod map;
mod sorting;
mod tree;
mod uf;
mod util;

fn main() {
    sorting::selection::run();
    sorting::insertion::run();
    sorting::bubble::run();
    sorting::merge::run();
    sorting::heap::run();
    sorting::quick::run();
    tree::bst::run();
    map::hashmap::run();
    uf::unionfind::run();
    graph::examples::run();
    heap::heap::run();
}
