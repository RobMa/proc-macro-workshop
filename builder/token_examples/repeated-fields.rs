#[derive(Builder)]
pub struct Command {
    executable: String,
    #[builder(each = "arg")]
    args: Vec<String>,
    #[builder(each = "env")]
    env: Vec<String>,
    current_dir: Option<String>,
}

DeriveInput {
    attrs: [],
    vis: Public(
        VisPublic {
            pub_token: Pub,
        },
    ),
    ident: Ident {
        ident: "Command",
        span: #0 bytes(1428..1435),
    },
    generics: Generics {
        lt_token: None,
        params: [],
        gt_token: None,
        where_clause: None,
    },
    data: Struct(
        DataStruct {
            struct_token: Struct,
            fields: Named(
                FieldsNamed {
                    brace_token: Brace,
                    named: [
                        Field {
                            attrs: [],
                            vis: Inherited,
                            ident: Some(
                                Ident {
                                    ident: "executable",
                                    span: #0 bytes(1442..1452),
                                },
                            ),
                            colon_token: Some(
                                Colon,
                            ),
                            ty: Path(
                                TypePath {
                                    qself: None,
                                    path: Path {
                                        leading_colon: None,
                                        segments: [
                                            PathSegment {
                                                ident: Ident {
                                                    ident: "String",
                                                    span: #0 bytes(1454..1460),
                                                },
                                                arguments: None,
                                            },
                                        ],
                                    },
                                },
                            ),
                        },
                        Comma,
                        Field {
                            attrs: [
                                Attribute {
                                    pound_token: Pound,
                                    style: Outer,
                                    bracket_token: Bracket,
                                    path: Path {
                                        leading_colon: None,
                                        segments: [
                                            PathSegment {
                                                ident: Ident {
                                                    ident: "builder",
                                                    span: #0 bytes(1468..1475),
                                                },
                                                arguments: None,
                                            },
                                        ],
                                    },
                                    tokens: TokenStream [
                                        Group {
                                            delimiter: Parenthesis,
                                            stream: TokenStream [
                                                Ident {
                                                    ident: "each",
                                                    span: #0 bytes(1476..1480),
                                                },
                                                Punct {
                                                    ch: '=',
                                                    spacing: Alone,
                                                    span: #0 bytes(1481..1482),
                                                },
                                                Literal { lit: Lit { kind: Str, symbol: "arg", suffix: None }, span: Span { lo: BytePos(1483), hi: BytePos(1488), ctxt: #0 } },
                                            ],
                                            span: #0 bytes(1475..1489),
                                        },
                                    ],
                                },
                            ],
                            vis: Inherited,
                            ident: Some(
                                Ident {
                                    ident: "args",
                                    span: #0 bytes(1495..1499),
                                },
                            ),
                            colon_token: Some(
                                Colon,
                            ),
                            ty: Path(
                                TypePath {
                                    qself: None,
                                    path: Path {
                                        leading_colon: None,
                                        segments: [
                                            PathSegment {
                                                ident: Ident {
                                                    ident: "Vec",
                                                    span: #0 bytes(1501..1504),
                                                },
                                                arguments: AngleBracketed(
                                                    AngleBracketedGenericArguments {
                                                        colon2_token: None,
                                                        lt_token: Lt,
                                                        args: [
                                                            Type(
                                                                Path(
                                                                    TypePath {
                                                                        qself: None,
                                                                        path: Path {
                                                                            leading_colon: None,
                                                                            segments: [
                                                                                PathSegment {
                                                                                    ident: Ident {
                                                                                        ident: "String",
                                                                                        span: #0 bytes(1505..1511),
                                                                                    },
                                                                                    arguments: None,
                                                                                },
                                                                            ],
                                                                        },
                                                                    },
                                                                ),
                                                            ),
                                                        ],
                                                        gt_token: Gt,
                                                    },
                                                ),
                                            },
                                        ],
                                    },
                                },
                            ),
                        },
                        Comma,
                        Field {
                            attrs: [
                                Attribute {
                                    pound_token: Pound,
                                    style: Outer,
                                    bracket_token: Bracket,
                                    path: Path {
                                        leading_colon: None,
                                        segments: [
                                            PathSegment {
                                                ident: Ident {
                                                    ident: "builder",
                                                    span: #0 bytes(1520..1527),
                                                },
                                                arguments: None,
                                            },
                                        ],
                                    },
                                    tokens: TokenStream [
                                        Group {
                                            delimiter: Parenthesis,
                                            stream: TokenStream [
                                                Ident {
                                                    ident: "each",
                                                    span: #0 bytes(1528..1532),
                                                },
                                                Punct {
                                                    ch: '=',
                                                    spacing: Alone,
                                                    span: #0 bytes(1533..1534),
                                                },
                                                Literal { lit: Lit { kind: Str, symbol: "env", suffix: None }, span: Span { lo: BytePos(1535), hi: BytePos(1540), ctxt: #0 } },
                                            ],
                                            span: #0 bytes(1527..1541),
                                        },
                                    ],
                                },
                            ],
                            vis: Inherited,
                            ident: Some(
                                Ident {
                                    ident: "env",
                                    span: #0 bytes(1547..1550),
                                },
                            ),
                            colon_token: Some(
                                Colon,
                            ),
                            ty: Path(
                                TypePath {
                                    qself: None,
                                    path: Path {
                                        leading_colon: None,
                                        segments: [
                                            PathSegment {
                                                ident: Ident {
                                                    ident: "Vec",
                                                    span: #0 bytes(1552..1555),
                                                },
                                                arguments: AngleBracketed(
                                                    AngleBracketedGenericArguments {
                                                        colon2_token: None,
                                                        lt_token: Lt,
                                                        args: [
                                                            Type(
                                                                Path(
                                                                    TypePath {
                                                                        qself: None,
                                                                        path: Path {
                                                                            leading_colon: None,
                                                                            segments: [
                                                                                PathSegment {
                                                                                    ident: Ident {
                                                                                        ident: "String",
                                                                                        span: #0 bytes(1556..1562),
                                                                                    },
                                                                                    arguments: None,
                                                                                },
                                                                            ],
                                                                        },
                                                                    },
                                                                ),
                                                            ),
                                                        ],
                                                        gt_token: Gt,
                                                    },
                                                ),
                                            },
                                        ],
                                    },
                                },
                            ),
                        },
                        Comma,
                        Field {
                            attrs: [],
                            vis: Inherited,
                            ident: Some(
                                Ident {
                                    ident: "current_dir",
                                    span: #0 bytes(1569..1580),
                                },
                            ),
                            colon_token: Some(
                                Colon,
                            ),
                            ty: Path(
                                TypePath {
                                    qself: None,
                                    path: Path {
                                        leading_colon: None,
                                        segments: [
                                            PathSegment {
                                                ident: Ident {
                                                    ident: "Option",
                                                    span: #0 bytes(1582..1588),
                                                },
                                                arguments: AngleBracketed(
                                                    AngleBracketedGenericArguments {
                                                        colon2_token: None,
                                                        lt_token: Lt,
                                                        args: [
                                                            Type(
                                                                Path(
                                                                    TypePath {
                                                                        qself: None,
                                                                        path: Path {
                                                                            leading_colon: None,
                                                                            segments: [
                                                                                PathSegment {
                                                                                    ident: Ident {
                                                                                        ident: "String",
                                                                                        span: #0 bytes(1589..1595),
                                                                                    },
                                                                                    arguments: None,
                                                                                },
                                                                            ],
                                                                        },
                                                                    },
                                                                ),
                                                            ),
                                                        ],
                                                        gt_token: Gt,
                                                    },
                                                ),
                                            },
                                        ],
                                    },
                                },
                            ),
                        },
                        Comma,
                    ],
                },
            ),
            semi_token: None,
        },
    ),
}