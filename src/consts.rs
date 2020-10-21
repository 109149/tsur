pub const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION"); // compile even without cargo;
pub const AUTHOR: &'static str = "109149 <109149qwe@gmail.com>";
pub const ABOUT: &'static str = "Just Rusting...You know...O_O";

// #[allow(dead_code, non_camel_case_types)]
// #[derive(Debug)]
// pub struct Sort_By<'a> {
//     pub va: &'a str,
//     pub vd: &'a str,
//     pub ka: &'a str,
//     pub kd: &'a str,
// }
// pub const SORT_BY: Sort_By<'static> = Sort_By {
//     va: "vasc",
//     vd: "vdesc",
//     ka: "kasc",
//     kd: "kdesc",
// };

pub const VA: &'static str = "vasc";
pub const VD: &'static str = "vdesc";
pub const KA: &'static str = "kasc";
pub const KD: &'static str = "kdesc";
