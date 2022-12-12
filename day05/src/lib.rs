#[derive(Debug)]
pub struct File {
    pub pile: Pile,
    pub commands: Vec<Move>,
}
#[derive(Debug)]
pub struct Pile {
    pub rows: Vec<Row>,
}
#[derive(Debug)]
pub struct Move {
    pub count: usize,
    pub from: usize,
    pub to: usize,
}
#[derive(Debug)]
pub struct Row {
    pub terms: Vec<Term>,
}
#[derive(Debug, Clone)]
pub enum Term {
    NullCrate,
    Crate(char),
    Label(i32),
}

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub parser); // synthesized by LALRPOP

#[cfg(test)]
mod test {
    use super::*;
    use expect_test::expect;
    fn parse(src: &str) -> File {
        parser::FileParser::new().parse(src).unwrap()
    }

    #[test]
    fn parse_row() {
        let ast = parser::RowParser::new().parse("[A] [B]     [C]\n");

        expect![[r#"
            Ok(
                Row {
                    terms: [
                        Crate(
                            'A',
                        ),
                        Crate(
                            'B',
                        ),
                        NullCrate,
                        Crate(
                            'C',
                        ),
                    ],
                },
            )
        "#]]
        .assert_debug_eq(&ast);
    }
    #[test]
    fn parse_pile() {
        let ast = parser::PileParser::new().parse(
            r"[A] [B]     [C]
 1   2   3   4 
",
        );

        expect![[r#"
            Ok(
                Pile {
                    rows: [
                        Row {
                            terms: [
                                Crate(
                                    'A',
                                ),
                                Crate(
                                    'B',
                                ),
                                NullCrate,
                                Crate(
                                    'C',
                                ),
                            ],
                        },
                        Row {
                            terms: [
                                Label(
                                    1,
                                ),
                                Label(
                                    2,
                                ),
                                Label(
                                    3,
                                ),
                                Label(
                                    4,
                                ),
                            ],
                        },
                    ],
                },
            )
        "#]]
        .assert_debug_eq(&ast);
    }
    #[test]
    fn parse_file() {
        let ast = parser::FileParser::new().parse(
            r"[A] [B]     [C]
 1   2   3   4 

move 1 from 2 to 3
",
        );
        expect![[r#"
            Ok(
                File {
                    pile: Pile {
                        rows: [
                            Row {
                                terms: [
                                    Crate(
                                        'A',
                                    ),
                                    Crate(
                                        'B',
                                    ),
                                    NullCrate,
                                    Crate(
                                        'C',
                                    ),
                                ],
                            },
                            Row {
                                terms: [
                                    Label(
                                        1,
                                    ),
                                    Label(
                                        2,
                                    ),
                                    Label(
                                        3,
                                    ),
                                    Label(
                                        4,
                                    ),
                                ],
                            },
                        ],
                    },
                    commands: [
                        Move {
                            count: 1,
                            from: 2,
                            to: 3,
                        },
                    ],
                },
            )
        "#]]
        .assert_debug_eq(&ast);
    }
}
