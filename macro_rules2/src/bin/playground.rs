use rand::{rngs::SmallRng, seq::index::sample as index_sample, SeedableRng, distributions::{Uniform, uniform::SampleUniform}};
use std::{boxed::Box, iter::Peekable, rc::Rc, vec, ops::Range};
use paste::paste;

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

    fn from_trees(trees: impl Into<Iterator<Item = Tree>>) -> Self {
        Tree::Branch(trees.into_iter().collect())
    }

    fn from_data(data: impl Into<Iterator<Item = usize>>) -> Self {
        Tree::Leaf(data.into_iter().collect())
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

fn uniform_from_range<N: SampleUniform>(range: Range<N>) -> Uniform<N> {
    Uniform::new(range.start, range.end)
}

macro_rules! generate_distribs {
    ($($var_id:ident),*) => {
        paste! {
            let ($([< $var_id _distrib >]),*) =  
                ($(uniform_from_range([< $var_id _range >])),*);
        }
        
    }
}

macro_rules! tree_param_generator {
    (
        $(
            $var_id:ident 
            $(
                (
                    $count_dependency:ident 
                    $(; $new_range_bound:ident)?
                )
            )?
        ),*
    ) => {
        paste! {
        
            #[derive(Clone)]
            struct TreeParamGenerator<N>
            where
                N: SampleUniform,
            {
                $([< $var_id _distrib >]: Uniform<N>,)*
                rng: SmallRng,
            }
            
            impl<N: SampleUniform> TreeParamGenerator<N> {
                fn new($([< $var_id _range >]: Range<N>),*) -> TreeParamGenerator<N> {
                    let ($([< $var_id _distrib >]),*) =  
                        ($(uniform_from_range([< $var_id _range >])),*);
                    TreeParamGenerator {
                        $([< $var_id _distrib >],)*
                        rng: SmallRng::from_entropy(),
                    }
                }
                
                $(
                    param_gen_methods!(
                        $var_id:ident 
                        $(
                            (
                                $count_dependency 
                                $(; $new_range_bound)?
                            )
                        )?
                    );
                )*
            }
        }
    }
}

macro_rules! param_gen_single {
    ( $var_id:ident, $($new_range_bound:ident)? ) => {
        paste! {
            fn [< $var_id _single>](&mut self $(, $new_range_bound: Range<N>)?) -> N {
                $(
                    self.[< update_ $var_id _bounds>]($new_range_bound);
                )?
                self.[< $var_id _distrib >].sample(&mut self.rng)
            }
        }
    }
}

macro_rules! param_gen_multiple {
    (
        $var_id:ident, $count_dep:ident
    ) => {
        paste! {
            fn [< $var_id >](&mut self) -> Vec<usize> {
                self.[< $var_id _distrib >]
                    .sample_iter(&mut self.rng)
                    .take(self.[< $count_dependency _single>]())
                    .collect()
            }
        }
    };
}

macro_rules! param_gen_methods {
    ( 
        $var_id:ident
        $(
            $(c: $count_dep:ident), u
        )? 
    ) => {}
    (
        $var_id:ident($count_dependency:ident ; $new_range_bound:ident)
    ) => {
        paste! {
            fn [< $var_id >](&mut self, ) -> Vec<usize> {
                self.[< $var_id _distrib >]
                    .sample_iter(&mut self.rng)
                    .take(self.[< $count_dependency _single>]())
                    .collect()
            }
        }
    };
        ( 
        $var_id:ident
        $(
            $(c: $count_dep:ident)
        )? 
    ) => {}
    (
        $var_id:ident($count_dependency:ident ; $new_range_bound:ident)
    ) => {
        paste! {
            fn [< $var_id >](&mut self, ) -> Vec<usize> {
                self.[< $var_id _distrib >]
                    .sample_iter(&mut self.rng)
                    .take(self.[< $count_dependency _single>]())
                    .collect()
            }
        }
    };
}

tree_param_generator!(
    leaf_width, 
    branch_width, 
    min_child_height(c:branch_width, u), 
    leaf_data(c:leaf_width)
);

const MAX_LEAF_WIDTH: usize = 5;
const MAX_BRANCH_WIDTH: usize = 5;
const MAX_TREE_DEPTH: usize = 5;
const MAX_HEIGHT: usize = MAX_TREE_DEPTH - 1;
const MAX_RAND_UINT: usize = 10;

fn tree_gen_helper(
    tree_param_generator: &mut TreeParamGenerator,
    current_height: usize,
    min_height: usize,
) -> Tree {
    if current_height > min_height {
        let min_child_heights = tree_param_generator
            .min_child_height().into_iter()
            .map(
                |child_depth| 
                    current_height.checked_sub(child_height).unwrap_or(0)
                        .min(min_height)
            );
        Tree::Branch(min_child_heights.map(|min_height| tree_gen_helper(tree_param_generator, current_height.checked_sub(1).unwrap_or(0), min_height)))
    } else {
        Tree::Leaf(tree_param_generator.leaf_data())
    }
}

fn gen_rand_tree() -> Tree {
    let param_generator = TreeParamGenerator::new(0..MAX_LEAF_WIDTH, 0..MAX_BRANCH_WIDTH, 0..MAX_HEIGHT);

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
