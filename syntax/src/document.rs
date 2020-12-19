//! A parsed GraphQL [`Document`].
//!
//! [`Document`]: ../struct.Document.html
use crate::nodes::DefinitionNode;

/// The Document is the root of a GraphQL schema and/or query. It contains a list of GraphQL
/// definitions. These can be anything from types, enums, unions, etc. to a query.
///
/// This struct will also provide validation methods and other ways to manipulate the GraphQL
/// syntax tree.
#[derive(Debug, PartialEq)]
pub struct Document {
    /// A list of GraphQL definitions
    pub definitions: Vec<DefinitionNode>,
}

impl Document {
    /// Create a new document with the provided definitions
    pub fn new(definitions: Vec<DefinitionNode>) -> Document {
        Document { definitions }
    }
}

use std::fmt;
impl fmt::Display for Document {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Document<{} definitions>", self.definitions.len())
    }
}

use crate::gql;
use std::default::Default;
impl Default for Document {
    fn default() -> Self {
        gql!(&format!(
            r#"
"""Int
A signed, 32-bit, non-fractional number.
Min: {i32_min}
Max:  {i32_max}
"""
scalar Int

"""TinyInt
A signed, 8-bit, non-fractional number.
Min: {i8_min}
Max:  {i8_max}
"""
scalar TinyInt

"""ShortInt
A signed, 16-bit, non-fractional number.
Min: {i16_min}
Max:  {i16_max}
"""
scalar ShortInt

"""LongInt
A signed, 64-bit, non-fractional number.
Min: {i64_min}
Max:  {i64_max}
"""
scalar LongInt

"""BigInt
A signed, 128-bit, non-fractional number.
Min: {i128_min}
Max:  {i128_max}
"""
scalar BigInt

"""Uint
An unsigned, 32-bit, non-fractional number.
Min: {u32_min}
Max:  {u32_max}
"""
scalar Uint

"""TinyUint
An unsigned, 8-bit, non-fractional number.
Min: {u8_min}
Max:  {u8_max}
"""
scalar TinyUint

"""ShortUint
An unsigned, 16-bit, non-fractional number.
Min: {u16_min}
Max:  {u16_max}
"""
scalar ShortUint

"""LongUint
An unsigned, 64-bit, non-fractional number.
Min: {u64_min}
Max:  {u64_max}
"""
scalar LongUint

"""BigUint
An unsigned, 128-bit, non-fractional number.
Min: {u128_min}
Max:  {u128_max}
"""
scalar BigUint

"""Float
A signed, 32-bit, fractional number.
For more information see [f32 docs](https://doc.rust-lang.org/std/primitive.f32.html).
"""
scalar Float

"""Double
A signed, 64-bit, fractional number.
For more information see [f64 docs](https://doc.rust-lang.org/std/primitive.f64.html).
"""
scalar Float

"""DateTime
A field used to represent a date and time.
"""
scalar DateTime

"""Date
A field used to represent a date.
"""
scalar Date

"""Time
A field used to represent a time.
"""
scalar Time

"""Boolean
Used to represent true and false
"""
scalar Boolean

"""ID
Used as a unique identifier.
"""
scalar ID


"""Schema
The root of any interaction with the database.
"""
schema Schema {{
    query: Query
    mutation: Mutation
}}

type Query {{}}
type Mutation {{}}
"#,
            i8_min = i8::MIN,
            i8_max = i8::MAX,
            i16_min = i16::MIN,
            i16_max = i16::MAX,
            i32_min = i32::MIN,
            i32_max = i32::MAX,
            i64_min = i64::MIN,
            i64_max = i64::MAX,
            i128_min = i128::MIN,
            i128_max = i128::MAX,
            u8_min = u8::MIN,
            u8_max = u8::MAX,
            u16_min = u16::MIN,
            u16_max = u16::MAX,
            u32_min = u32::MIN,
            u32_max = u32::MAX,
            u64_min = u64::MIN,
            u64_max = u64::MAX,
            u128_min = u128::MIN,
            u128_max = u128::MAX,
        ))
        .expect("Default schema is invalid")
    }
}
