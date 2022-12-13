#[derive(Debug)]
pub struct Log {
    pub actions: Vec<Action>,
}
#[derive(Debug)]
pub enum Action {
    ChangeDir(String),
    List(Vec<Entry>),
}
#[derive(Debug)]
pub enum Entry {
    Dir(String),
    File(i32, String),
}

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub parser); // synthesized by LALRPOP

#[cfg(test)]
mod test {
    use super::*;
    use expect_test::expect;

    #[test]
    fn parse_log() {
        let ast = parser::LogParser::new().parse(
            r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
"#,
        );

        expect![[r#"
            Ok(
                Log {
                    actions: [
                        ChangeDir(
                            "/",
                        ),
                        List(
                            [
                                Dir(
                                    "a",
                                ),
                                File(
                                    14848514,
                                    "b.txt",
                                ),
                                File(
                                    8504156,
                                    "c.dat",
                                ),
                                Dir(
                                    "d",
                                ),
                            ],
                        ),
                        ChangeDir(
                            "a",
                        ),
                        List(
                            [
                                Dir(
                                    "e",
                                ),
                                File(
                                    29116,
                                    "f",
                                ),
                                File(
                                    2557,
                                    "g",
                                ),
                                File(
                                    62596,
                                    "h.lst",
                                ),
                            ],
                        ),
                        ChangeDir(
                            "e",
                        ),
                        List(
                            [
                                File(
                                    584,
                                    "i",
                                ),
                            ],
                        ),
                        ChangeDir(
                            "..",
                        ),
                        ChangeDir(
                            "..",
                        ),
                        ChangeDir(
                            "d",
                        ),
                        List(
                            [
                                File(
                                    4060174,
                                    "j",
                                ),
                                File(
                                    8033020,
                                    "d.log",
                                ),
                                File(
                                    5626152,
                                    "d.ext",
                                ),
                                File(
                                    7214296,
                                    "k",
                                ),
                            ],
                        ),
                    ],
                },
            )
        "#]]
        .assert_debug_eq(&ast);
    }
}
