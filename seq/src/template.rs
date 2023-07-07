use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use proc_macro2::TokenTree;
use std::borrow::Cow;
use std::cell::Ref;
use std::cell::RefCell;
use std::ops::Deref;
use syn::Ident;

#[derive(Clone)]
struct Meta<'a, T: Clone>(Cow<'a, RefCell<T>>);

impl<'a, T: Clone> Meta<'a, T> {
    fn new(value: T) -> Self {
        Self(Cow::Owned(RefCell::new(value)))
    }

    fn duplicate(&'a self) -> Meta<'a, T> {
        Self(match self.0 {
            Cow::Borrowed(b) => Cow::Borrowed(b),
            Cow::Owned(_) => Cow::Borrowed(self.0.as_ref()),
        })
    }

    fn replace(&self, value: T) {
        self.0.replace(value);
    }

    fn inner_clone(&self) -> T {
        self.0.borrow().clone()
    }

    fn inner_ref(&'a self) -> Ref<'a, T> {
        self.0.borrow()
    }
}

impl<'a> Meta<'a, Ident> {
    fn matches_id(&self, id: &Ident) -> bool {
        self.inner_ref().deref() == id
    }

    fn stream(&self) -> TokenStream2 {
        TokenStream2::from(TokenTree::from(self.inner_ref().clone()))
    }
}

pub(crate) struct Template<'a> {
    // stream: PartialStream<'a>,
    meta_var: Meta<'a, Ident>,
    meta_stream: Meta<'a, TokenStream2>,
    partial_stream: Vec<Option<TokenStream2>>,
}
struct PartialStream {
    vec: Vec<Option<TokenStream2>>,
}

// enum Partial {
//     Group(Vec<Partial>),
//     MetaVar,
//     Stream(TokenStream2),
// }

enum PartialStream {
    MetaVar,
    Stream(TokenStream2),
}

impl<'a> From<&Template<'a>> for TokenStream2 {
    fn from(value: &Template<'a>) -> TokenStream2 {
        value.stream()
    }
}

impl From<Vec<Partial>> for Partial {
    fn from(value: Vec<Partial>) -> Self {
        Partial::Group(value)
    }
}

impl From<TokenStream2> for Partial {
    fn from(value: TokenStream2) -> Self {
        Partial::Stream(value)
    }
}

impl From<TokenTree> for Partial {
    fn from(value: TokenTree) -> Self {
        Partial::Stream(TokenStream2::from(value))
    }
}

impl<'a> Template<'a> {
    pub(crate) fn new(value: TokenStream2, metavar: Ident) -> Self {
        let meta_var = Meta::new(metavar);
        let meta_stream = Meta::new(meta_var.stream());
        let partials = value
            .into_iter()
            .map(|tt| Partial::new(tt, &meta_var))
            .collect::<Vec<Partial>>()
            .into();

        Self {
            meta_var,
            meta_stream,
            partial: partials,
        }
    }

    pub(crate) fn set_meta_var(&self, id: Ident) {
        self.meta_var.replace(id);
        self.meta_stream.replace(self.meta_var.stream())
    }

    fn stream(&self) -> TokenStream2 {
        self.partial.stream(&self.meta_stream)
    }
}

impl Partial {
    fn new(value: TokenTree, metavar: &Meta<Ident>) -> Self {
        match value {
            TokenTree::Group(g) => g
                .stream()
                .into_iter()
                .map(|tt| Partial::new(tt, metavar))
                .collect::<Vec<Partial>>()
                .into(),
            TokenTree::Ident(id) if metavar.matches_id(&id) => Partial::MetaVar,
            value => Partial::Stream(value.into()),
        }
    }

    fn new2(value: TokenTree, meta_var: &Meta<Ident>) -> Vec<Partial> {
        match value {
            TokenTree::Group(g) => g
                .stream()
                .into_iter()
                .map(|tt| Partial::new2(tt, meta_var))
                .collect::<Vec<Partial>>()
                .into(),
            TokenTree::Ident(id) if meta_var.matches_id(&id) => Partial::MetaVar,
            value => Partial::Stream(value.into()),
        }
    }

    // fn stream(&self, meta_stream: &Meta<TokenStream2>) -> Vec<TokenStream2> {
    //     match self {
    //         Partial::Group(g) => {
    //             TokenStream2::from_iter(g.iter().map(|partial| partial.stream(meta_stream)))
    //         },
    //         Partial::MetaVar => meta_stream.inner_clone(),
    //         Partial::Stream(ts) => ts.clone(),
    //     }
    // }

    fn is_meta(&self) -> bool {
        match self {
            Partial::MetaVar => true,
            _ => false,
        }
    }
}

// #[derive(Clone)]
// enum RefTemplateTree<'a> {
//     Vector(Vec<RefTemplateTree<'a>>),
//     Group(Vec<RefTemplateTree<'a>>),
//     Ident(&'a RefCell<Ident>),
//     Other,
// }

// impl<'a> From<&'a TemplateTree> for RefTemplateTree<'a> {
//     fn from(value: &TemplateTree) -> Self {
//         match value {
//             TemplateTree::Vector(v) | TemplateTree::Group(v) => {
//                 RefTemplateTree::Group(v.iter().map(RefTemplateTree::from).collect())
//             }
//             TemplateTree::Ident(id) => RefTemplateTree::Ident(id),
//             TemplateTree::Other(_) => RefTemplateTree::Other,
//         }
//     }
// }

// impl<'a> RefTemplateTree<'a> {
//     fn replace_matches(&mut self, metavar_id: Ident) -> Option<RefTemplateTree> {
//         self.find_and_replace(metavar_id, None)
//     }

//     fn find_and_replace(
//         &mut self,
//         metavar_id: Ident,
//         mut metavar: Option<RefTemplateTree>,
//     ) -> Option<RefTemplateTree> {
// match self {
//     RefTemplateTree::Vector(v) | RefTemplateTree::Group(v) => {
//         v.iter_mut().for_each(|ref_tt| {
//             metavar = ref_tt.find_and_replace(metavar_id, metavar);
//         });
//     }
//     RefTemplateTree::Ident(id) => {
//         if *(id.borrow().deref()) == metavar_id {
//             if let Some(replacement) = metavar {
//                 *self = replacement;
//             } else {
//                 metavar.replace(self.clone());
//             }
//         }
//     }
//     RefTemplateTree::Other => {}
// }
// metavar
//     }
// }

// impl<'a> Into<TokenStream2> for RefTemplateTree<'a> {
//     fn into(self) -> TokenStream2 {
//         todo!()
//     }
// }

// enum TokenTreeType {
//     RefCell(Template),
//     Regular(TokenTree),
// }

// impl From<TokenTree> for TokenTreeType {
//     fn from(value: TokenTree) -> Self {
//         TokenTreeType::Regular(value)
//     }
// }

// impl From<Template> for TokenTreeType {
//     fn from(value: Template) -> Self {
//         TokenTreeType::RefCell(value)
//     }
// }

// fn find_and_replace(
//     tts: impl Iterator<Item = TokenTreeType>,
//     template_stream: &TokenStream2,
//     metavar: Ident,
// ) -> (Vec<Template>, Vec<&Template>) {
//     let metavar_ref: Option<&Ident> = None;
//     tts.map(|tt| match tt {
//         TokenTreeType::RefCell(ref_cell_tt) => {}
//         TokenTreeType::Regular(tt) => match tt {
//             TokenTree::Group(g) => {
//                 find_and_replace(tts.map(TokenTreeType::from), template_stream, metavar)
//             }
//             TokenTree::Ident(id) => {
//                 if id == metavar {
//                     if let Some(metavar_ref) = metavar_ref {
//                     } else {
//                         metavar_ref.replace(value)
//                     }
//                 }
//             }
//             _ => {}
//         },
//     })
// }

// impl<'a> StreamPieces<'a> {
//     fn new(to_replace: Ident, template_stream: &TokenStream2) -> Self {
//         let template: Vec<RefCell<TokenTree>> =
//             template_stream.into_iter().map(RefCell::new).collect();
//         let ref_template = template.iter().collect();

//         StreamPieces {
//             template,
//             ref_template,
//             to_replace,
//             replace_handle: None,
//         }
//     }

// fn recursive_replace(teplate: &Vec<RefCell<TokenTree>>) -> Vec<&RefCell<<TokenTree>> {
//     let ref_template = template.iter().collect();
//     let metavar = None;

//     for
// }

// fn setup_replace(self) -> Self {
//     let replace_indices: Vec<usize> = self.ref_template
//     .iter()
//     .enumerate()
//     .filter_map(|(ix, tree)| match tree.borrow().deref() {
//         TokenTree::Ident(id) => (&self.to_replace == id).then_some(ix),
//         _ => None,
//     })
//     .collect();
//     eprintln!("found replace indices: {:?}", &replace_indices);

//     if !replace_indices.is_empty() {
//         let first_ix = replace_indices[0];
//         println!("non-trivial body");
//         let first = self.ref_template[first_ix];
//         for ix in replace_indices {
//             ref_template[ix] = first;
//         }
//     }
// }
// }

// impl ToTokens for StreamPieces {
//     fn to_tokens(&self, tokens: &mut TokenStream2) {
//         tokens.extend(self.as_ref_cell().borrow().to_token_stream());
//     }
// }

// impl VisitMut for StreamPiece {
//     fn visit_ident_mut(&mut self, i: &mut Ident) {
//         if &self.to_replace == i {
//             i =
//         }
//     }
// }

// fn simple_replace(
//     template_stream: TokenStream2,
//     to_replace: Ident,
//     range_start: LitInt,
//     range_end: LitInt,
// ) -> TokenStream2 {
//     eprintln!("beginning replacement. to_replace: {:?}", to_replace);
//     let mut result_stream = TokenStream2::new();

//     let tree_pieces: Vec<StreamPiece> = ;
//     eprintln!("built tree pieces");
//     eprintln!("tree pieces:\n{:#?}", tree_pieces);
//     let mut ref_tree_pieces: Vec<&StreamPiece> = tree_pieces.iter().collect();
//     eprintln!("built ref tree pieces");
//     let replace_indices: Vec<usize> = ref_tree_pieces
//         .iter()
//         .enumerate()
//         .filter_map(|(ix, tree)| match tree.as_ref_cell().borrow().deref() {
//             TokenTree::Ident(id) => (&to_replace == id).then_some(ix),
//             _ => None,
//         })
//         .collect();
//     eprintln!("found replace indices: {:?}", &replace_indices);

//     if !replace_indices.is_empty() {
//         let first_ix = replace_indices[0];
//         println!("non-trivial body");
//         let first = ref_tree_pieces[first_ix];
//         for ix in replace_indices {
//             ref_tree_pieces[ix] = first;
//         }

//         for n in
//             range_start.base10_parse::<usize>().unwrap()..range_end.base10_parse::<usize>().unwrap()
//         {
//             println!("changing first item");
//             tree_pieces[first_ix]
//                 .as_ref_cell()
//                 .replace(TokenTree::Literal(Literal::usize_unsuffixed(n)));
//             result_stream.extend(quote! {
//                 #(#ref_tree_pieces)*
//             });
//         }
//     }

//     result_stream
// }

// fn simple_replace(
//     template: TokenStream2,
//     loop_var: Ident,
//     range_start: LitInt,
//     range_end: LitInt,
// ) -> TokenStream {
//     let result_stream = TokenStream2::new();

//     quote! {
//         #body
//     }
// }
