mod graph;
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
    tree::bst::run();
    sorting::merge::run();
    map::hashmap::run();
    uf::unionfind::run();
}
