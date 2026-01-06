// use std::fmt::Display;
//
// /* PgVector supported distances
// <-> - L2 distance
// <#> - (negative) inner product
// <=> - cosine distance
// <+> - L1 distance (added in 0.7.0)
// <~> - Hamming distance (binary vectors, added in 0.7.0)
// <%> - Jaccard distance (binary vectors, added in 0.7.0)
//  */
// pub enum PgVectorDistanceFunction {
//     L2,
//     InnerProduct,
//     Cosine,
//     L1,
//     Hamming,
//     Jaccard,
// }
//
// impl Display for PgVectorDistanceFunction {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         match self {
//             PgVectorDistanceFunction::L2 => write!(f, "<->"),
//             PgVectorDistanceFunction::InnerProduct => write!(f, "<#>"),
//             PgVectorDistanceFunction::Cosine => write!(f, "<=>"),
//             PgVectorDistanceFunction::L1 => write!(f, "<+>"),
//             PgVectorDistanceFunction::Hamming => write!(f, "<~>"),
//             PgVectorDistanceFunction::Jaccard => write!(f, "<%>"),
//         }
//     }
// }