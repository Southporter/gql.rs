use crate::document::Document;
use crate::error::{ParseError, ParseResult};
use crate::lexer::Lexer;
use crate::nodes::object_type_extension::ObjectTypeExtensionNode;
use crate::nodes::*;
use crate::token::{Location, Token};
use std::iter::{Iterator, Peekable};
use std::rc::Rc;

pub struct AST<'i> {
    lexer: Peekable<Lexer<'i>>,
}

use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

impl<'i> Display for AST<'i> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "AST")
    }
}
impl<'i> Debug for AST<'i> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "AST")
    }
}

impl<'i> AST<'i> {
    pub fn new(input: &'i str) -> ParseResult<AST<'i>> {
        let lexer = Lexer::new(input).peekable();
        Ok(AST { lexer })
    }

    pub fn parse(&'i mut self) -> ParseResult<Document> {
        let definitions = self.parse_definitions()?;
        Ok(Document::new(definitions))
    }

    fn parse_description(&mut self) -> ParseResult<Description> {
        match self.unwrap_peeked_token()? {
            Token::BlockStr(_, _) | Token::Str(_, _) => {
                let tok = self.unwrap_next_token()?;
                Ok(Some(StringValueNode::new(tok)?))
            }
            _ => Ok(None),
        }
    }

    fn parse_input_value(&mut self) -> ParseResult<InputValueDefinitionNode> {
        let description = self.parse_description()?;
        let name_tok = self.unwrap_next_token()?;
        self.expect_token(Token::Colon(Location::ignored()))?;
        let type_node = self.parse_field_type()?;
        let default_value = self.parse_default_value()?;
        let directives = self.parse_directives()?;
        let mut input_value = InputValueDefinitionNode::new(name_tok, type_node, description)?;
        input_value.with_default_value(default_value);
        input_value.with_directives(directives);
        Ok(input_value)
    }

    fn parse_arguments_definition(&mut self) -> ParseResult<Option<ArgumentDefinitions>> {
        match self.expect_optional_token(&Token::OpenParen(Location::ignored())) {
            Some(_) => {
                if let Some(token) =
                    self.expect_optional_token(&Token::CloseParen(Location::ignored()))
                {
                    return Err(ParseError::ArgumentEmpty(token.location()));
                }
                let mut args: ArgumentDefinitions = Vec::new();
                loop {
                    args.push(self.parse_input_value()?);
                    if let Some(_) =
                        self.expect_optional_token(&Token::CloseParen(Location::ignored()))
                    {
                        break;
                    }
                }
                Ok(Some(args))
            }
            None => Ok(None),
        }
    }

    fn parse_argument(&mut self) -> ParseResult<Argument> {
        let name = self.unwrap_next_token()?;
        self.expect_token(Token::Colon(Location::ignored()))?;
        let value = self.parse_value()?;
        Ok(Argument {
            name: NameNode::new(name)?,
            value,
        })
    }

    fn parse_arguments(&mut self) -> ParseResult<Option<Arguments>> {
        match self.expect_optional_token(&Token::OpenParen(Location::ignored())) {
            Some(_) => {
                let mut args: Arguments = Vec::new();
                loop {
                    if let Some(token) =
                        self.expect_optional_token(&Token::CloseParen(Location::ignored()))
                    {
                        if args.is_empty() {
                            return Err(ParseError::ArgumentEmpty(token.location()));
                        }
                        break;
                    }
                    args.push(self.parse_argument()?);
                }
                Ok(Some(args))
            }
            None => Ok(None),
        }
    }

    fn parse_directive(&mut self) -> ParseResult<DirectiveNode> {
        self.expect_token(Token::At(Location::ignored()))?;
        let name = self.unwrap_next_token()?;
        let arguments = self.parse_arguments()?;
        Ok(DirectiveNode::new(name, arguments)?)
    }

    fn parse_directives(&mut self) -> ParseResult<Option<Vec<DirectiveNode>>> {
        let mut directives: Vec<DirectiveNode> = Vec::new();
        loop {
            if let Token::At(_) = self.unwrap_peeked_token()? {
                directives.push(self.parse_directive()?);
            } else {
                break;
            }
        }
        if !directives.is_empty() {
            Ok(Some(directives))
        } else {
            Ok(None)
        }
    }

    fn parse_definitions(&'i mut self) -> ParseResult<Vec<DefinitionNode>> {
        self.expect_token(Token::Start)?;
        if let Some(_) = self.expect_optional_token(&Token::End) {
            Err(ParseError::DocumentEmpty)
        } else {
            let mut nodes: Vec<DefinitionNode> = Vec::new();
            loop {
                nodes.push(self.parse_definition()?);
                if let Some(_) = self.expect_optional_token(&Token::End) {
                    break;
                }
            }
            Ok(nodes)
        }
    }

    fn parse_definition(&mut self) -> ParseResult<DefinitionNode> {
        let description = self.parse_description()?;
        let tok = self.unwrap_peeked_token()?;
        match tok {
            Token::Name(_, val) => match *val {
                "type" | "enum" | "union" | "interface" | "input" | "scalar" => {
                    Ok(DefinitionNode::TypeSystem(TypeSystemDefinitionNode::Type(
                        self.parse_type(description)?,
                    )))
                }
                "extend" => Ok(DefinitionNode::Extension(
                    self.parse_type_extension(description)?,
                )),
                "query" | "fragment" => Ok(DefinitionNode::Executable(self.parse_executable()?)),
                _ => Err(ParseError::BadValue),
            },
            Token::OpenBrace(_) => Ok(DefinitionNode::Executable(self.parse_executable()?)),
            _ => Err(ParseError::UnexpectedToken {
                expected: String::from("Token<Name> or Token<OpenBrace>"),
                received: tok.to_string().to_owned(),
                location: tok.location(),
            }),
        }
    }

    fn parse_type(&mut self, description: Description) -> ParseResult<TypeDefinitionNode> {
        let tok = self.unwrap_next_token()?;
        if let Token::Name(_, val) = tok {
            match val {
                "type" => Ok(TypeDefinitionNode::Object(
                    self.parse_object_type(description)?,
                )),
                "enum" => Ok(TypeDefinitionNode::Enum(self.parse_enum_type(description)?)),
                "union" => Ok(TypeDefinitionNode::Union(
                    self.parse_union_type(description)?,
                )),
                "interface" => Ok(TypeDefinitionNode::Interface(
                    self.parse_interface_type(description)?,
                )),
                "input" => Ok(TypeDefinitionNode::Input(
                    self.parse_input_type(description)?,
                )),
                "scalar" => Ok(TypeDefinitionNode::Scalar(
                    self.parse_scalar_type(description)?,
                )),
                _ => Err(ParseError::BadValue),
            }
        } else {
            Err(ParseError::UnexpectedToken {
                expected: String::from("Token::Name"),
                received: tok.to_string(),
                location: tok.location(),
            })
        }
    }

    fn parse_type_extension(
        &mut self,
        description: Description,
    ) -> ParseResult<TypeSystemExtensionNode> {
        self.unwrap_next_token()?; // Discard "extend"
        match self.unwrap_next_token()? {
            Token::Name(_, "type") => Ok(TypeSystemExtensionNode::Object(
                self.parse_object_type_extension(description)?,
            )),
            tok => Err(ParseError::UnexpectedToken {
                expected: String::from("Token::Name"),
                received: tok.to_string().to_owned(),
                location: tok.location(),
            }),
        }
    }

    fn parse_object_type(
        &mut self,
        description: Description,
    ) -> ParseResult<ObjectTypeDefinitionNode> {
        let name_tok = self.expect_token(Token::Name(Location::ignored(), ""))?;
        let interfaces = self.parse_object_interfaces()?;
        let directives = self.parse_directives()?;
        let fields = self.parse_fields()?;

        let mut obj = ObjectTypeDefinitionNode::new(name_tok, description, fields)?;
        obj.with_interfaces(interfaces);
        obj.with_directives(directives);
        Ok(obj)
    }

    fn parse_object_type_extension(
        &mut self,
        description: Description,
    ) -> ParseResult<ObjectTypeExtensionNode> {
        let name_tok = self.unwrap_next_token()?;
        let interfaces = self.parse_object_interfaces()?;
        let directives = self.parse_directives()?;

        let mut type_extension = ObjectTypeExtensionNode::new(name_tok, description)?;
        type_extension.with_interfaces(interfaces);
        type_extension.with_directives(directives);

        if let Token::OpenBrace(_) = self.unwrap_peeked_token()? {
            let fields = self.parse_fields()?;
            type_extension.with_fields(fields);
        }

        Ok(type_extension)
    }

    fn parse_interface_type(
        &mut self,
        description: Description,
    ) -> ParseResult<InterfaceTypeDefinitionNode> {
        let name_tok = self.expect_token(Token::Name(Location::ignored(), ""))?;
        let directives = self.parse_directives()?;
        let fields = self.parse_fields()?;

        let mut interface = InterfaceTypeDefinitionNode::new(name_tok, description)?;
        interface.with_directives(directives);
        interface.with_fields(fields);
        Ok(interface)
    }

    fn parse_input_type(
        &mut self,
        description: Description,
    ) -> ParseResult<InputTypeDefinitionNode> {
        let name_tok = self.expect_token(Token::Name(Location::ignored(), ""))?;
        let mut input_type = InputTypeDefinitionNode::new(name_tok, description)?;
        let fields = self.parse_input_fields()?;
        input_type.with_fields(fields);
        Ok(input_type)
    }

    fn parse_scalar_type(
        &mut self,
        description: Description,
    ) -> ParseResult<ScalarTypeDefinitionNode> {
        let name_tok = self.expect_token(Token::Name(Location::ignored(), ""))?;
        let directives = self.parse_directives()?;
        let mut scalar_type = ScalarTypeDefinitionNode::new(name_tok, description)?;
        scalar_type.with_directives(directives);
        Ok(scalar_type)
    }

    fn parse_enum_type(&mut self, description: Description) -> ParseResult<EnumTypeDefinitionNode> {
        let name_tok = self.expect_token(Token::Name(Location::ignored(), "enum"))?;
        if name_tok == Token::Name(Location::ignored(), "true")
            || name_tok == Token::Name(Location::ignored(), "false")
            || name_tok == Token::Name(Location::ignored(), "null")
        {
            return Err(ParseError::BadValue);
        }
        let directives = self.parse_directives()?;
        let values = self.parse_enum_values()?;
        Ok(EnumTypeDefinitionNode::new(
            name_tok,
            description,
            directives,
            values,
        )?)
    }

    fn parse_union_type(
        &mut self,
        description: Description,
    ) -> ParseResult<UnionTypeDefinitionNode> {
        let name_tok = self.expect_token(Token::Name(Location::ignored(), "union"))?;
        let directives = self.parse_directives()?;
        self.expect_token(Token::Equals(Location::ignored()))?;
        let types = self.parse_union_types()?;
        Ok(UnionTypeDefinitionNode::new(
            name_tok,
            description,
            directives,
            types,
        )?)
    }

    fn parse_object_interfaces(&mut self) -> ParseResult<Option<Vec<NamedTypeNode>>> {
        if let Some(name_tok) = self.expect_optional_token(&Token::Name(Location::ignored(), "")) {
            match name_tok {
                Token::Name(_, "implements") => {
                    let mut interface_names: Vec<NamedTypeNode> = Vec::new();
                    loop {
                        let interface_name =
                            self.expect_token(Token::Name(Location::ignored(), ""))?;
                        interface_names.push(NamedTypeNode::new(interface_name)?);
                        if let None = self.expect_optional_token(&Token::Amp(Location::ignored())) {
                            break;
                        }
                    }
                    Ok(Some(interface_names))
                }
                Token::Name(_, keyword) => Err(ParseError::UnexpectedKeyword {
                    expected: String::from("implements"),
                    received: keyword.to_owned(),
                    location: name_tok.location(),
                }),
                tok => Err(ParseError::UnexpectedToken {
                    expected: String::from("Token<Name>"),
                    received: tok.to_string(),
                    location: tok.location(),
                }),
            }
        } else {
            Ok(None)
        }
    }

    fn parse_fields(&mut self) -> ParseResult<Vec<FieldDefinitionNode>> {
        let mut fields: Vec<FieldDefinitionNode> = Vec::new();
        self.expect_token(Token::OpenBrace(Location::ignored()))?;
        loop {
            if let Some(_) = self.expect_optional_token(&Token::CloseBrace(Location::ignored())) {
                break;
            }
            fields.push(self.parse_field()?);
        }
        Ok(fields)
    }

    fn parse_field(&mut self) -> ParseResult<FieldDefinitionNode> {
        let description = self.parse_description()?;
        let name = self.expect_token(Token::Name(Location::ignored(), ""))?;
        let arguments = self.parse_arguments_definition()?;
        println!("arguments, {:?}", arguments);
        self.expect_token(Token::Colon(Location::ignored()))?;
        let field_type = self.parse_field_type()?;
        FieldDefinitionNode::new(name, field_type, description, arguments)
    }

    fn parse_field_type(&mut self) -> ParseResult<TypeNode> {
        let mut field_type: TypeNode;
        if let Some(_) = self.expect_optional_token(&Token::OpenSquare(Location::ignored())) {
            field_type = TypeNode::List(ListTypeNode::new(self.parse_field_type()?));
            self.expect_token(Token::CloseSquare(Location::ignored()))?;
        } else {
            field_type = TypeNode::Named(NamedTypeNode::new(
                self.expect_token(Token::Name(Location::ignored(), ""))?,
            )?);
        }
        if let Some(_) = self.expect_optional_token(&Token::Bang(Location::ignored())) {
            field_type = TypeNode::NonNull(Rc::new(field_type));
        }
        Ok(field_type)
    }

    fn parse_input_fields(&mut self) -> ParseResult<Vec<InputValueDefinitionNode>> {
        let mut fields: Vec<InputValueDefinitionNode> = Vec::new();
        let tok = self.expect_token(Token::OpenBrace(Location::ignored()))?;
        loop {
            if let Some(_) = self.expect_optional_token(&Token::CloseBrace(Location::ignored())) {
                break;
            }
            fields.push(self.parse_input_value()?);
        }
        if !fields.is_empty() {
            Ok(fields)
        } else {
            Err(ParseError::ObjectEmpty(tok.location()))
        }
    }

    fn parse_enum_values(&mut self) -> ParseResult<Vec<EnumValueDefinitionNode>> {
        let mut values: Vec<EnumValueDefinitionNode> = Vec::new();
        self.expect_token(Token::OpenBrace(Location::ignored()))?;
        loop {
            if let Some(_) = self.expect_optional_token(&Token::CloseBrace(Location::ignored())) {
                break;
            }
            let description = self.parse_description()?;
            let name = self.expect_token(Token::Name(Location::ignored(), ""))?;
            let directives = self.parse_directives()?;
            values.push(EnumValueDefinitionNode::new(name, description, directives)?);
        }
        Ok(values)
    }

    fn parse_union_types(&mut self) -> ParseResult<Vec<NamedTypeNode>> {
        let mut types: Vec<NamedTypeNode> = Vec::new();
        // First Pipe is truely optional
        self.expect_optional_token(&Token::Pipe(Location::ignored()));
        types.push(NamedTypeNode::new(self.unwrap_next_token()?)?);
        loop {
            if let Some(_) = self.expect_optional_token(&Token::Pipe(Location::ignored())) {
                types.push(NamedTypeNode::new(self.unwrap_next_token()?)?);
            } else {
                break;
            }
        }
        Ok(types)
    }

    fn parse_default_value(&mut self) -> ParseResult<Option<ValueNode>> {
        match self.expect_optional_token(&Token::Equals(Location::ignored())) {
            Some(_) => Ok(Some(self.parse_value()?)),
            None => Ok(None),
        }
    }

    fn parse_value(&mut self) -> ParseResult<ValueNode> {
        let tok = self.unwrap_peeked_token()?;
        match *tok {
            Token::Name(_, value) => {
                self.unwrap_next_token()?;
                match value {
                    "true" => Ok(ValueNode::Bool(BooleanValueNode { value: true })),
                    "false" => Ok(ValueNode::Bool(BooleanValueNode { value: false })),
                    "null" => Ok(ValueNode::Null),
                    _ => Ok(ValueNode::Enum(EnumValueNode {
                        value: value.to_owned(),
                    })),
                }
            }
            Token::Int(_, value) => {
                self.unwrap_next_token()?;
                Ok(ValueNode::Int(IntValueNode { value }))
            }
            Token::Float(_, value) => {
                self.unwrap_next_token()?;
                Ok(ValueNode::Float(FloatValueNode { value }))
            }
            Token::Str(_, _) | Token::BlockStr(_, _) => {
                let str_tok = self.unwrap_next_token()?;
                Ok(ValueNode::Str(StringValueNode::new(str_tok)?))
            }
            Token::Dollar(_) => {
                let variable = self.parse_variable()?;
                Ok(ValueNode::Variable(variable))
            }
            Token::OpenSquare(_) => {
                let list_value = self.parse_list_value()?;
                Ok(ValueNode::List(list_value))
            }
            Token::OpenBrace(_) => {
                let obj_value = self.parse_object_value()?;
                Ok(ValueNode::Object(obj_value))
            }
            _ => Err(ParseError::UnexpectedToken {
                expected: String::from(
                    "One of (Name, Int, Float, Str, Dollar, OpenSquare, OpenBrace)",
                ),
                received: tok.to_owned().to_string(),
                location: tok.location(),
            }),
        }
    }

    fn parse_list_value(&mut self) -> ParseResult<ListValueNode> {
        self.expect_token(Token::OpenSquare(Location::ignored()))?;
        let mut values: Vec<ValueNode> = Vec::new();
        loop {
            if let Some(_) = self.expect_optional_token(&Token::CloseSquare(Location::ignored())) {
                break;
            }
            values.push(self.parse_value()?);
        }
        Ok(ListValueNode { values })
    }

    fn parse_object_value(&mut self) -> ParseResult<ObjectValueNode> {
        self.expect_token(Token::OpenBrace(Location::ignored()))?;
        let mut fields: Vec<ObjectFieldNode> = Vec::new();
        loop {
            if let Some(_) = self.expect_optional_token(&Token::CloseBrace(Location::ignored())) {
                break;
            }
            let name = self.unwrap_next_token()?;
            self.expect_token(Token::Colon(Location::ignored()))?;
            let value = self.parse_value()?;
            fields.push(ObjectFieldNode {
                name: NameNode::new(name)?,
                value,
            });
        }
        Ok(ObjectValueNode { fields })
    }

    fn parse_variable(&mut self) -> ParseResult<VariableNode> {
        self.expect_token(Token::Dollar(Location::ignored()))?;
        let name = self.unwrap_next_token()?;
        Ok(VariableNode {
            name: NameNode::new(name)?,
        })
    }

    fn parse_executable(&mut self) -> ParseResult<ExecutableDefinitionNode> {
        let tok = self.unwrap_peeked_token()?;
        match tok {
            Token::Name(_, val) => match *val {
                "query" /* | "mutation" | "subscription" */ => Ok(ExecutableDefinitionNode::Operation(self.parse_operation_type()?)),
                "fragment" =>
                    Ok(ExecutableDefinitionNode::Fragment(self.parse_fragment_definition()?))
                ,
                _ => Err(ParseError::BadValue),
            },
            Token::OpenBrace(_) => Ok(ExecutableDefinitionNode::Operation(
                OperationTypeNode::Query(self.parse_anonymous_query()?),
            )),
            tok => Err(ParseError::UnexpectedToken {
                expected: String::from(
                    "One of 'query', 'mutation', 'subscription', 'fragment', or anonymous query",
                ),
                received: tok.to_string(),
                location: tok.location(),
            }),
        }
    }

    fn parse_operation_type(&mut self) -> ParseResult<OperationTypeNode> {
        let keyword = self.unwrap_next_token()?;
        if let Token::Name(loc, name) = keyword {
            match name {
                "query" => Ok(OperationTypeNode::Query(self.parse_query()?)),
                _ => Err(ParseError::UnexpectedKeyword {
                    expected: String::from("One of 'query'"),
                    received: String::from("name"),
                    location: loc,
                }),
            }
        } else {
            Err(ParseError::UnexpectedToken {
                expected: "Token<Name>".into(),
                received: keyword.to_string(),
                location: keyword.location(),
            })
        }
    }

    fn parse_query(&mut self) -> ParseResult<QueryDefinitionNode> {
        let name = self.unwrap_next_token()?;
        let variables = self.parse_variables()?;
        let selections = self.parse_selection_set()?;
        Ok(QueryDefinitionNode {
            name: Some(NameNode::new(name)?),
            variables,
            selections,
        })
    }

    fn parse_variables(&mut self) -> ParseResult<Variables> {
        let mut variables = Vec::new();
        if let Some(_) = self.expect_optional_token(&Token::OpenParen(Location::ignored())) {
            loop {
                if let Some(_) = self.expect_optional_token(&Token::CloseParen(Location::ignored()))
                {
                    break;
                }
                variables.push(self.parse_variable_definition()?);
            }
        }
        Ok(variables)
    }

    fn parse_variable_definition(&mut self) -> ParseResult<VariableDefinitionNode> {
        let variable = self.parse_variable()?;
        self.expect_token(Token::Colon(Location::ignored()))?;
        let variable_type = self.parse_field_type()?;
        let mut var = VariableDefinitionNode {
            variable,
            variable_type,
            default_value: None,
        };
        if let Some(_) = self.expect_optional_token(&Token::Equals(Location::ignored())) {
            let value = self.parse_value()?;
            var.default_value = Some(value);
        }
        Ok(var)
    }

    fn parse_anonymous_query(&mut self) -> ParseResult<QueryDefinitionNode> {
        let selections = self.parse_selection_set()?;
        Ok(QueryDefinitionNode {
            name: None,
            variables: vec![],
            selections,
        })
    }

    fn parse_selection_set(&mut self) -> ParseResult<Vec<Selection>> {
        self.expect_token(Token::OpenBrace(Location::ignored()))?;
        let mut selections = Vec::new();
        loop {
            if let Some(_) = self.expect_optional_token(&Token::CloseBrace(Location::ignored())) {
                break;
            }
            selections.push(self.parse_selection()?);
        }
        Ok(selections)
    }

    fn parse_selection(&mut self) -> ParseResult<Selection> {
        match self.unwrap_peeked_token()? {
            Token::Name(_, _) => Ok(Selection::Field(self.parse_field_node()?)),
            Token::Spread(_) => Ok(Selection::Fragment(self.parse_fragment_spread()?)),
            _ => Err(ParseError::NotImplemented),
        }
    }

    fn parse_field_node(&mut self) -> ParseResult<FieldNode> {
        let mut field: FieldNode;

        let name = self.unwrap_next_token()?;
        if let Some(_) = self.expect_optional_token(&Token::Colon(Location::ignored())) {
            let root = self.unwrap_next_token()?;
            field = FieldNode::new(root)?;
            field.with_alias(name)?;
        } else {
            field = FieldNode::new(name)?;
        }

        let arguments = self.parse_arguments()?;
        field.with_arguments(arguments);

        let directives = self.parse_directives()?;
        field.with_directives(directives);

        if let &Token::OpenBrace(_) = self.unwrap_peeked_token()? {
            let selections = self.parse_selection_set()?;
            field.with_selections(selections);
        }

        Ok(field)
    }

    fn parse_fragment_definition(&mut self) -> ParseResult<FragmentDefinitionNode> {
        let keyword = self.unwrap_next_token()?;
        if let Token::Name(loc, name) = keyword {
            match name {
                "fragment" => {
                    let name = self.unwrap_next_token()?;
                    let _on = self.unwrap_next_token()?;
                    let node_type = self.unwrap_next_token()?;
                    let frag_def = FragmentDefinitionNode::new(name, node_type)?
                        .with_directives(self.parse_directives()?)
                        .with_selections(self.parse_selection_set()?);

                    Ok(frag_def)
                }
                _ => Err(ParseError::UnexpectedKeyword {
                    expected: String::from("fragment"),
                    received: String::from(name),
                    location: loc,
                }),
            }
        } else {
            Err(ParseError::UnexpectedToken {
                expected: "Token<Name>".into(),
                received: keyword.to_string(),
                location: keyword.location(),
            })
        }
    }

    fn parse_fragment_spread(&mut self) -> ParseResult<FragmentSpread> {
        self.expect_token(Token::Spread(Location::ignored()))?;
        match self.unwrap_peeked_token()? {
            &Token::Name(_, "on") => {
                Ok(FragmentSpread::Inline(self.parse_inline_fragment_spread()?))
            }
            &Token::At(_) => Ok(FragmentSpread::Inline(
                self.parse_anonymous_inline_fragmen_spread()?,
            )),
            &Token::Name(_, _) => Ok(FragmentSpread::Node(self.parse_fragment_spread_node()?)),
            tok => Err(ParseError::UnexpectedToken {
                location: tok.location(),
                expected: String::from("One of Token::Name or Token::At"),
                received: tok.to_string(),
            }),
        }
    }

    fn parse_fragment_spread_node(&mut self) -> ParseResult<FragmentSpreadNode> {
        let name = self.unwrap_next_token()?;
        let directives = self.parse_directives()?;
        Ok(FragmentSpreadNode {
            name: NameNode::new(name)?,
            directives,
        })
    }

    fn parse_inline_fragment_spread(&mut self) -> ParseResult<InlineFragmentSpreadNode> {
        let _on_tok = self.unwrap_next_token()?;
        let name = self.unwrap_next_token()?;
        let directives = self.parse_directives()?;
        let selections = self.parse_selection_set()?;
        Ok(InlineFragmentSpreadNode {
            node_type: Some(NamedTypeNode::new(name)?),
            directives,
            selections,
        })
    }

    fn parse_anonymous_inline_fragmen_spread(&mut self) -> ParseResult<InlineFragmentSpreadNode> {
        let directives = self.parse_directives()?;
        let selections = self.parse_selection_set()?;
        Ok(InlineFragmentSpreadNode {
            node_type: None,
            directives,
            selections,
        })
    }

    fn expect_token(&mut self, tok: Token<'i>) -> ParseResult<Token<'i>> {
        if let Some(next) = self.lexer.next() {
            match next {
                Ok(actual) => {
                    if actual.is_same_type(&tok) {
                        Ok(actual)
                    } else {
                        Err(ParseError::UnexpectedToken {
                            expected: tok.to_string(),
                            received: actual.to_string().to_owned(),
                            location: actual.location(),
                        })
                    }
                }
                Err(e) => Err(ParseError::LexError(e)),
            }
        } else {
            Err(ParseError::EOF)
        }
    }

    fn expect_optional_token(&mut self, tok: &Token<'i>) -> Option<Token<'i>> {
        if let Some(next) = self.lexer.peek() {
            match next {
                Ok(actual) => {
                    if actual.is_same_type(tok) {
                        Some(self.lexer.next().unwrap().unwrap())
                    } else {
                        None
                    }
                }
                Err(_) => None,
            }
        } else {
            None
        }
    }

    fn unwrap_peeked_token(&mut self) -> ParseResult<&Token<'i>> {
        match self.lexer.peek() {
            Some(res) => match res {
                Ok(tok) => Ok(tok),
                Err(lex_error) => Err(ParseError::LexError(*lex_error)),
            },
            None => Err(ParseError::EOF),
        }
    }

    fn unwrap_next_token(&mut self) -> ParseResult<Token<'i>> {
        match self.lexer.next() {
            Some(res) => match res {
                Ok(tok) => Ok(tok),
                Err(lex_error) => Err(ParseError::LexError(lex_error)),
            },
            None => Err(ParseError::EOF),
        }
    }
}

// struct Location<'a> {
//     start: Token<'a>,
//     end: Token<'a>,
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_constructs() {
        let ast = AST::new("test");
        assert!(ast.is_ok());
    }

    #[test]
    fn it_parses_int_value() {
        let mut ast = AST::new("42").unwrap();
        ast.expect_token(Token::Start).unwrap();
        let value = ast.parse_value();
        println!("IntValue: {:?}", value);
        assert!(value.is_ok());
        assert_eq!(value.unwrap(), ValueNode::Int(IntValueNode { value: 42 }));
    }

    #[test]
    fn it_parses_float_value() {
        let mut ast = AST::new("3.1415926").unwrap();
        ast.expect_token(Token::Start).unwrap();
        let value = ast.parse_value();
        println!("FloatValue: {:?}", value);
        assert!(value.is_ok());
        assert_eq!(
            value.unwrap(),
            ValueNode::Float(FloatValueNode { value: 3.1415926 })
        );
    }

    #[test]
    fn it_parses_block_string_values() {
        let mut ast = AST::new(r#""""BlockStrValue""""#).unwrap();
        ast.expect_token(Token::Start).unwrap();
        let value = ast.parse_value();
        assert!(value.is_ok());
        assert_eq!(
            value.unwrap(),
            ValueNode::Str(
                StringValueNode::new(Token::BlockStr(Location::ignored(), "BlockStrValue"))
                    .unwrap()
            )
        );
    }

    #[test]
    fn it_parses_string_values() {
        let mut ast = AST::new(r#""StrValue""#).unwrap();
        ast.expect_token(Token::Start).unwrap();
        let value = ast.parse_value();
        assert!(value.is_ok());
        assert_eq!(
            value.unwrap(),
            ValueNode::Str(
                StringValueNode::new(Token::Str(Location::ignored(), "StrValue")).unwrap()
            )
        );
    }

    #[test]
    fn it_parses_bool_values() {
        let mut ast = AST::new("true, false").unwrap();
        ast.expect_token(Token::Start).unwrap();
        let value = ast.parse_value();
        assert!(value.is_ok());
        assert_eq!(
            value.unwrap(),
            ValueNode::Bool(BooleanValueNode { value: true })
        );
        let value = ast.parse_value();
        assert!(value.is_ok());
        assert_eq!(
            value.unwrap(),
            ValueNode::Bool(BooleanValueNode { value: false })
        );
    }

    #[test]
    fn it_parses_null_value() {
        let mut ast = AST::new("null").unwrap();
        ast.expect_token(Token::Start).unwrap();
        let value = ast.parse_value();
        assert!(value.is_ok());
        assert_eq!(value.unwrap(), ValueNode::Null);
    }

    #[test]
    fn it_parses_list_value() {
        let mut ast = AST::new("[true, false], [[1,2,3],[4,5,6]]").unwrap();
        ast.expect_token(Token::Start).unwrap();
        let value = ast.parse_value();
        assert!(value.is_ok());
        assert_eq!(
            value.unwrap(),
            ValueNode::List(ListValueNode {
                values: vec![
                    ValueNode::Bool(BooleanValueNode { value: true }),
                    ValueNode::Bool(BooleanValueNode { value: false }),
                ]
            })
        );
        let value = ast.parse_value();
        assert!(value.is_ok());
        assert_eq!(
            value.unwrap(),
            ValueNode::List(ListValueNode {
                values: vec![
                    ValueNode::List(ListValueNode {
                        values: vec![
                            ValueNode::Int(IntValueNode { value: 1 }),
                            ValueNode::Int(IntValueNode { value: 2 }),
                            ValueNode::Int(IntValueNode { value: 3 }),
                        ]
                    }),
                    ValueNode::List(ListValueNode {
                        values: vec![
                            ValueNode::Int(IntValueNode { value: 4 }),
                            ValueNode::Int(IntValueNode { value: 5 }),
                            ValueNode::Int(IntValueNode { value: 6 }),
                        ]
                    })
                ]
            })
        )
    }

    #[test]
    fn it_parses_object_value() {
        let mut ast = AST::new(r#"{}, { id: 42, name: "Obj"}"#).unwrap();
        ast.expect_token(Token::Start).unwrap();
        let value = ast.parse_value();
        assert!(value.is_ok());
        assert_eq!(
            value.unwrap(),
            ValueNode::Object(ObjectValueNode { fields: vec![] })
        );

        let value = ast.parse_value();
        assert!(value.is_ok());
        assert_eq!(
            value.unwrap(),
            ValueNode::Object(ObjectValueNode {
                fields: vec![
                    ObjectFieldNode {
                        name: NameNode::from("id"),
                        value: ValueNode::Int(IntValueNode { value: 42 }),
                    },
                    ObjectFieldNode {
                        name: NameNode::from("name"),
                        value: ValueNode::Str(
                            StringValueNode::new(Token::Str(Location::ignored(), "Obj")).unwrap()
                        ),
                    }
                ]
            })
        )
    }

    #[test]
    fn parses_a_variable() {
        let mut ast = AST::new("$myVariable").unwrap();
        ast.expect_token(Token::Start).unwrap();
        let value = ast.parse_value();
        assert!(value.is_ok());
        assert_eq!(
            value.unwrap(),
            ValueNode::Variable(VariableNode {
                name: NameNode::from("myVariable")
            })
        );
    }

    #[test]
    fn parses_a_directive() {
        let mut ast = AST::new("@deprecated").unwrap();
        ast.expect_token(Token::Start).unwrap();
        let value = ast.parse_directives();
        assert!(value.is_ok());
        assert_eq!(
            value.unwrap().unwrap(),
            vec![DirectiveNode {
                name: NameNode::from("deprecated"),
                arguments: None,
            }]
        )
    }

    #[test]
    fn parses_directive_with_arguments() {
        let mut ast = AST::new("@include(if: true)").unwrap();
        ast.expect_token(Token::Start).unwrap();
        let value = ast.parse_directives();
        assert!(value.is_ok());
        assert_eq!(
            value.unwrap().unwrap(),
            vec![DirectiveNode {
                name: NameNode::from("include"),
                arguments: Some(vec![Argument {
                    name: NameNode::from("if"),
                    value: ValueNode::Bool(BooleanValueNode { value: true })
                }]),
            }]
        )
    }

    #[test]
    fn parses_directive_with_multiple_arguments() {
        let mut ast = AST::new("@size(height: 100, width: 50)").unwrap();
        ast.expect_token(Token::Start).unwrap();
        let value = ast.parse_directives();
        assert!(value.is_ok());
        assert_eq!(
            value.unwrap().unwrap(),
            vec![DirectiveNode {
                name: NameNode::from("size"),
                arguments: Some(vec![
                    Argument {
                        name: NameNode::from("height"),
                        value: ValueNode::Int(IntValueNode { value: 100 })
                    },
                    Argument {
                        name: NameNode::from("width"),
                        value: ValueNode::Int(IntValueNode { value: 50 })
                    }
                ]),
            }]
        )
    }

    #[test]
    fn parses_enum_with_directives() {
        let mut ast = AST::new("enum BadDirection @depricated { NORTH SWEST @badValue EAST WOUTH @badValue(allow: true) }").unwrap();
        ast.expect_token(Token::Start).unwrap();
        let value = ast.parse_type(None);
        println!("Value: {:?}", value);
        assert!(value.is_ok());
        assert_eq!(
            value.unwrap(),
            TypeDefinitionNode::Enum(EnumTypeDefinitionNode {
                description: None,
                name: NameNode::from("BadDirection"),
                directives: Some(vec![DirectiveNode {
                    name: NameNode::from("depricated"),
                    arguments: None
                }]),
                values: vec![
                    EnumValueDefinitionNode {
                        description: None,
                        name: NameNode::from("NORTH"),
                        directives: None,
                    },
                    EnumValueDefinitionNode {
                        description: None,
                        name: NameNode::from("SWEST"),
                        directives: Some(vec![DirectiveNode {
                            name: NameNode::from("badValue"),
                            arguments: None
                        }])
                    },
                    EnumValueDefinitionNode {
                        description: None,
                        name: NameNode::from("EAST"),
                        directives: None,
                    },
                    EnumValueDefinitionNode {
                        description: None,
                        name: NameNode::from("WOUTH"),
                        directives: Some(vec![DirectiveNode {
                            name: NameNode::from("badValue"),
                            arguments: Some(vec![Argument {
                                name: NameNode::from("allow"),
                                value: ValueNode::Bool(BooleanValueNode { value: true })
                            }])
                        }])
                    },
                ]
            })
        )
    }
}
