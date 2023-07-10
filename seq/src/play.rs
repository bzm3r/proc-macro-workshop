use rand::rngs::SmallRng;
use rand::seq::index::sample as index_sample;
use rand::SeedableRng;
use std::boxed::Box;
use std::iter::Peekable;
use std::rc::Rc;
use std::vec;

type Data = Vec<usize>;
type Trees = Vec<Tree>;

enum Tree {
    Branch(Trees),
    Leaf(Data),
}

impl Tree {
    fn into_iter(self) -> TreeIter {
        match self {
            Tree::Branch(trees) => TreeIter::Branch(trees.into()),
            Tree::Leaf(data) => TreeIter::Leaf(data.into_iter()),
        }
    }

    fn from_trees(trees: Trees) -> Self {
        Tree::Branch(trees)
    }

    fn from_data(data: Data) -> Self {
        Tree::Leaf(data.into())
    }
}

struct BranchIter {
    trees: vec::IntoIter<Tree>,
    current_iter: Box<Peekable<TreeIter>>,
}

impl From<Tree> for Box<Peekable<TreeIter>> {
    fn from(value: Tree) -> Self {
        value.into_iter().peekable().into()
    }
}

impl From<TreeIter> for Box<Peekable<TreeIter>> {
    fn from(value: TreeIter) -> Self {
        value.peekable().into()
    }
}

impl From<Trees> for BranchIter {
    fn from(value: Trees) -> Self {
        let mut trees = value.into_iter();
        let current_iter = trees
            .next()
            .map(|tree| tree.into())
            .unwrap_or(TreeIter::Empty.into());

        BranchIter {
            trees,
            current_iter,
        }
    }
}

impl Iterator for BranchIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_iter.peek().is_some() {
            self.current_iter.next()
        } else if let Some(tree) = self.trees.next() {
            self.current_iter = tree.into();
            self.next()
        } else {
            None
        }
    }
}

enum TreeIter {
    Branch(BranchIter),
    Leaf(vec::IntoIter<usize>),
    Empty,
}

impl Iterator for TreeIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            TreeIter::Branch(branch_iter) => branch_iter.next(),
            TreeIter::Leaf(leaf_iter) => leaf_iter.next(),
            TreeIter::Empty => None,
        }
    }
}

#[derive(Clone)]
struct BounderSampler<'a, N>
where
    N: SampleUniform,
{
    dist: Uniform<N>,
    rng: Rc<SmallRng>,
    locking_iter: Option<DistIter<Uniform<N>, &'a mut SmallRng, N>>,
}

impl<'a, N> Iterator for BounderSampler<'a, N>
where
    N: SampleUniform,
{
    type Item = N;

    fn next(&mut self) -> Option<Self::Item> {
        if Some(dist_iter) = self.locking_iter {
            dist_iter.next()
        } else {
            self.dist.sample(self.get_mut_rng()).into()
        }
    }

    fn take(mut self, n: usize) -> Take<Self> {
        self.use_locked_iter();
        Take::new(self, n)
    }
}

impl Drop for &mut BoundedSampler {
    fn drop(&mut self) {
        self.drop_locked_iter();
    }
}

impl<'a, N> BounderSampler<'a, N>
where
    N: SampleUniform,
{
    fn new(rng: Rc<SmallRng>, value_range: Range<N>) -> BounderSampler<N> {
        let dist = Uniform::new(value_range.start, value_range.end);

        BounderSampler {
            rng,
            dist,
            ..default()
        }
    }

    fn get_mut_rng(&mut self) -> &mut SmallRng {
        Rc::get_mut(&self.rng).expect("could not lock RNG")
    }

    fn use_locked_iter(&mut self) {
        self.locking_iter
            .replace(self.dist.sample_iter(self.get_mut_rng()));
    }

    fn drop_locked_iter(&mut self) {
        let _ = self.locked_iter.take();
    }
}

const MAX_LEAF_WIDTH: usize = 5;
const MAX_BRANCH_WIDTH: usize = 5;
const MAX_TREE_DEPTH: u32 = 5;
const MAX_RAND_UINT: usize = 10;

fn tree_gen_helper(
    rng: Rc<SmallRng>,
    data_iter: &mut impl Iterator<Item = usize>,
    leaf_width_sampler: &mut BounderSampler,
    branch_width_sampler: &mut BoundedSampler,
    current_height: usize,
    max_child_depth: usize,
) -> Tree {
    if current_height > MAX_TREE_DEPTH - max_depth {
        let max_child_depth = self.current_height.min(MAX_TREE_DEPTH);
        let branch_width = branch_width_sampler.next();
        (0..branch_width).map(|| {
            tree_gen_helper(
                rng.clone(),
                data_iter,
                leaf_width_sampler,
                branch_width_sampler,
            )
        })
    } else {
        let leaf_width = leaf_width_sampler.next();
        Tree::Leaf(data_iter.take())
    }
}

fn gen_rand_tree() -> Tree {
    let mut rng = Rc::new(SmallRng::from_entropy());

    let data_size = MAX_BRANCH_WIDTH.pow(MAX_TREE_DEPTH - 1) * MAX_LEAF_WIDTH;
    let data = BounderSampler::new(Rc::get_mut(&rng), 0..MAX_RAND_UINT)
        .take(data_size)
        .collect::<Vec<usize>>()
        .into_iter();

    Tree::from_trees(vec![])
}

fn main() {
    let tree = Tree::from_trees(vec![
        Tree::from_trees(vec![
            Tree::from_data(vec![0]),
            Tree::from_trees(vec![Tree::from_data(vec![1, 2]), Tree::from_data(vec![])]),
        ]),
        Tree::from_trees(vec![]),
        Tree::from_trees(vec![Tree::from_data(vec![3, 4])]),
    ]);

    let flattened = tree.into_iter().collect::<Data>();
    println!("{:?}", flattened);
}
