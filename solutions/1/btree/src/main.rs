enum BTreeNode {
    BTreeNodeInternal,
    BTreeNodeLeaf,
}

struct KeyPointerPair {
    k: i32,
    v: Box<BTreeNode>,
}

#[derive(Debug)]
struct KeyDataPair {
    k: i32,
    v: Box<Data>,
}

impl PartialEq for KeyDataPair {
    fn eq(&self, other: &Self) -> bool {
        self.k == other.k
    }
}

impl Eq for KeyDataPair {}

impl PartialOrd for KeyDataPair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for KeyDataPair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.k.cmp(&other.k)
    }
}

#[derive(Debug)]
struct Data {
    data: String
}

#[derive(Default)]
struct BTreeNodeInternal {
    keys: [Option<Box<KeyPointerPair>>; 9],
}

struct BTreeNodeLeaf {
    keys: [Option<Box<KeyDataPair>>; 8],
}



fn main() {
    let mut data = [
                KeyDataPair{k: 3, v: Box::new(Data{data: String::from("fizz")})}, 
                KeyDataPair{k: 1, v: Box::new(Data{data: String::from("foo")})}, 
                KeyDataPair{k: 2, v: Box::new(Data{data: String::from("bar")})},
                KeyDataPair{k: 4, v: Box::new(Data{data: String::from("something")})},
                KeyDataPair{k: 5, v: Box::new(Data{data: String::from("else")})}
                ];

    build_b_tree(&mut data);
}

fn build_b_tree(data: &mut [KeyDataPair]) -> BTreeNodeInternal{

    data.sort();
    println!("{:#?}", data);

    // Initial storage utilization - what percentage of slots to fill up.
    const INIT_STORAGE_UTIL: f32 = 0.5;
    const NUM_CHILDREN: f32 = 9.0;

    let m = data.len() as f32;
    // number of things we have to put in when we consider desired empty space
    let num_data_with_empties = (1.0 / INIT_STORAGE_UTIL) * m;
    let num_nodes: f32 = ( num_data_with_empties / NUM_CHILDREN ).ceil();
    // should height include root?
    let height: f32 = num_nodes.log(NUM_CHILDREN).ceil();

    println!("num_nodes: {:#?}, height {:#?}, num_data_with_empties: {:#?}", num_nodes, height, num_data_with_empties);

    Default::default()


}

fn add_data(tree: &BTreeNodeInternal, data: KeyDataPair) -> &BTreeNodeInternal{
    Default::default()
}

fn delete_data(tree: &BTreeNodeInternal, key: i32) -> &BTreeNodeInternal{
    Default::default()
}

fn view_data(tree: &BTreeNodeInternal, key: i32) -> &BTreeNodeInternal{
    Default::default()
}
