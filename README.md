e gql.rs
Purely GraphQL database written in Rust

![Rust](https://github.com/ssedrick/gql.rs/workflows/Rust/badge.svg)

### Structure
##### Lexer
A general purpose GraphQL tokenize

##### Parser
A general purpose GraphQL AST generator

##### DB
The main code for handling the data storage and retrieval



### Built-in Types
GQL will have the following built in types:

##### Strings
* TinyString (Maximum 255 bytes)
* String (Maximum 65,535 bytes)
* MediumString (Maximum 16,777,215 bytes)
* LongString (Maximum 4,294,967,295 or 4GB bytes)

##### Ints
* Int (equivalent to i32)
* Short (i8)
* Long (i64)
* BigInt (undetermined)

##### Floats
* Float (equivalent to f32)
* Double (f64)

##### Date/Time
* Date
* Time
