use super::*;

#[tokio::test(flavor = "multi_thread")]
async fn test_move_parent_node_end() -> anyhow::Result<()> {
    let tests = vec![
        // single cursor stays single cursor, first goes to end of current
        // node, then parent
        (
            helpers::platform_line(indoc! {r##"
                fn foo() {
                    let result = if true {
                        "yes"
                    } else {
                        "no#["|]#
                    }
                }
            "##}),
            "<A-e>",
            helpers::platform_line(indoc! {"\
                fn foo() {
                    let result = if true {
                        \"yes\"
                    } else {
                        \"no\"#[\n|]#
                    }
                }
            "}),
        ),
        (
            helpers::platform_line(indoc! {"\
                fn foo() {
                    let result = if true {
                        \"yes\"
                    } else {
                        \"no\"#[\n|]#
                    }
                }
            "}),
            "<A-e>",
            helpers::platform_line(indoc! {"\
                fn foo() {
                    let result = if true {
                        \"yes\"
                    } else {
                        \"no\"
                    }#[\n|]#
                }
            "}),
        ),
        // select mode extends
        (
            helpers::platform_line(indoc! {r##"
                fn foo() {
                    let result = if true {
                        "yes"
                    } else {
                        #["no"|]#
                    }
                }
            "##}),
            "v<A-e><A-e>",
            helpers::platform_line(indoc! {"\
                fn foo() {
                    let result = if true {
                        \"yes\"
                    } else {
                        #[\"no\"
                    }\n|]#
                }
            "}),
        ),
    ];

    for test in tests {
        test_with_config(AppBuilder::new().with_file("foo.rs", None), test).await?;
    }

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_move_parent_node_start() -> anyhow::Result<()> {
    let tests = vec![
        // single cursor stays single cursor, first goes to end of current
        // node, then parent
        (
            helpers::platform_line(indoc! {r##"
                fn foo() {
                    let result = if true {
                        "yes"
                    } else {
                        "no#["|]#
                    }
                }
            "##}),
            "<A-b>",
            helpers::platform_line(indoc! {"\
                fn foo() {
                    let result = if true {
                        \"yes\"
                    } else {
                        #[\"|]#no\"
                    }
                }
            "}),
        ),
        (
            helpers::platform_line(indoc! {"\
                fn foo() {
                    let result = if true {
                        \"yes\"
                    } else {
                        \"no\"#[\n|]#
                    }
                }
            "}),
            "<A-b>",
            helpers::platform_line(indoc! {"\
                fn foo() {
                    let result = if true {
                        \"yes\"
                    } else #[{|]#
                        \"no\"
                    }
                }
            "}),
        ),
        (
            helpers::platform_line(indoc! {"\
                fn foo() {
                    let result = if true {
                        \"yes\"
                    } else #[{|]#
                        \"no\"
                    }
                }
            "}),
            "<A-b>",
            helpers::platform_line(indoc! {"\
                fn foo() {
                    let result = if true {
                        \"yes\"
                    } #[e|]#lse {
                        \"no\"
                    }
                }
            "}),
        ),
        // select mode extends
        (
            helpers::platform_line(indoc! {r##"
                fn foo() {
                    let result = if true {
                        "yes"
                    } else {
                        #["no"|]#
                    }
                }
            "##}),
            "v<A-b><A-b>",
            helpers::platform_line(indoc! {"\
                fn foo() {
                    let result = if true {
                        \"yes\"
                    } else #[|{
                        ]#\"no\"
                    }
                }
            "}),
        ),
        (
            helpers::platform_line(indoc! {r##"
                fn foo() {
                    let result = if true {
                        "yes"
                    } else {
                        #["no"|]#
                    }
                }
            "##}),
            "v<A-b><A-b><A-b>",
            helpers::platform_line(indoc! {"\
                fn foo() {
                    let result = if true {
                        \"yes\"
                    } #[|else {
                        ]#\"no\"
                    }
                }
            "}),
        ),
    ];

    for test in tests {
        test_with_config(AppBuilder::new().with_file("foo.rs", None), test).await?;
    }

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_smart_tab_move_parent_node_end() -> anyhow::Result<()> {
    let tests = vec![
        // single cursor stays single cursor, first goes to end of current
        // node, then parent
        (
            helpers::platform_line(indoc! {r##"
                fn foo() {
                    let result = if true {
                        "yes"
                    } else {
                        "no#["|]#
                    }
                }
            "##}),
            "i<tab>",
            helpers::platform_line(indoc! {"\
                fn foo() {
                    let result = if true {
                        \"yes\"
                    } else {
                        \"no\"#[|\n]#
                    }
                }
            "}),
        ),
        (
            helpers::platform_line(indoc! {"\
                fn foo() {
                    let result = if true {
                        \"yes\"
                    } else {
                        \"no\"#[\n|]#
                    }
                }
            "}),
            "i<tab>",
            helpers::platform_line(indoc! {"\
                fn foo() {
                    let result = if true {
                        \"yes\"
                    } else {
                        \"no\"
                    }#[|\n]#
                }
            "}),
        ),
        // appending to the end of a line should still look at the current
        // line, not the next one
        (
            helpers::platform_line(indoc! {"\
                fn foo() {
                    let result = if true {
                        \"yes\"
                    } else {
                        \"no#[\"|]#
                    }
                }
            "}),
            "a<tab>",
            helpers::platform_line(indoc! {"\
                fn foo() {
                    let result = if true {
                        \"yes\"
                    } else {
                        \"no\"
                    }#[\n|]#
                }
            "}),
        ),
        // before cursor is all whitespace, so insert tab
        (
            helpers::platform_line(indoc! {"\
                fn foo() {
                    let result = if true {
                        \"yes\"
                    } else {
                        #[\"no\"|]#
                    }
                }
            "}),
            "i<tab>",
            helpers::platform_line(indoc! {"\
                fn foo() {
                    let result = if true {
                        \"yes\"
                    } else {
                            #[|\"no\"]#
                    }
                }
            "}),
        ),
        // if selection spans multiple lines, it should still only look at the
        // line on which the head is
        (
            helpers::platform_line(indoc! {"\
                fn foo() {
                    let result = if true {
                        #[\"yes\"
                    } else {
                        \"no\"|]#
                    }
                }
            "}),
            "a<tab>",
            helpers::platform_line(indoc! {"\
                fn foo() {
                    let result = if true {
                        \"yes\"
                    } else {
                        \"no\"
                    }#[\n|]#
                }
            "}),
        ),
        (
            helpers::platform_line(indoc! {"\
                fn foo() {
                    let result = if true {
                        #[\"yes\"
                    } else {
                        \"no\"|]#
                    }
                }
            "}),
            "i<tab>",
            helpers::platform_line(indoc! {"\
                fn foo() {
                    let result = if true {
                            #[|\"yes\"
                    } else {
                        \"no\"]#
                    }
                }
            "}),
        ),
        (
            helpers::platform_line(indoc! {"\
                fn foo() {
                    #[l|]#et result = if true {
                        #(\"yes\"
                    } else {
                        \"no\"|)#
                    }
                }
            "}),
            "i<tab>",
            helpers::platform_line(indoc! {"\
                fn foo() {
                        #[|l]#et result = if true {
                            #(|\"yes\"
                    } else {
                        \"no\")#
                    }
                }
            "}),
        ),
        (
            helpers::platform_line(indoc! {"\
                fn foo() {
                    let result = if true {
                        \"yes\"#[\n|]#
                    } else {
                        \"no\"#(\n|)#
                    }
                }
            "}),
            "i<tab>",
            helpers::platform_line(indoc! {"\
                fn foo() {
                    let result = if true {
                        \"yes\"
                    }#[| ]#else {
                        \"no\"
                    }#(|\n)#
                }
            "}),
        ),
        (
            helpers::platform_line(indoc! {"\
                fn foo() {
                    let result = if true {
                        #[\"yes\"|]#
                    } else {
                        #(\"no\"|)#
                    }
                }
            "}),
            "i<tab>",
            helpers::platform_line(indoc! {"\
                fn foo() {
                    let result = if true {
                            #[|\"yes\"]#
                    } else {
                            #(|\"no\")#
                    }
                }
            "}),
        ),
        // if any cursors are not preceded by all whitespace, then do the
        // smart_tab action
        (
            helpers::platform_line(indoc! {"\
                fn foo() {
                    let result = if true {
                        #[\"yes\"\n|]#
                    } else {
                        \"no#(\"\n|)#
                    }
                }
            "}),
            "i<tab>",
            helpers::platform_line(indoc! {"\
                fn foo() {
                    let result = if true {
                        \"yes\"
                    }#[| ]#else {
                        \"no\"
                    }#(|\n)#
                }
            "}),
        ),
        // Ctrl-tab always inserts a tab
        (
            helpers::platform_line(indoc! {"\
                fn foo() {
                    let result = if true {
                        #[\"yes\"\n|]#
                    } else {
                        \"no#(\"\n|)#
                    }
                }
            "}),
            "i<S-tab>",
            helpers::platform_line(indoc! {"\
                fn foo() {
                    let result = if true {
                            #[|\"yes\"\n]#
                    } else {
                        \"no    #(|\"\n)#
                    }
                }
            "}),
        ),
    ];

    for test in tests {
        test_with_config(AppBuilder::new().with_file("foo.rs", None), test).await?;
    }

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn expand_shrink_selection() -> anyhow::Result<()> {
    let tests = vec![
        // single range
        (
            helpers::platform_line(indoc! {r##"
                Some(#[thing|]#)
            "##}),
            "<A-o><A-o>",
            helpers::platform_line(indoc! {r##"
                #[Some(thing)|]#
            "##}),
        ),
        // multi range
        (
            helpers::platform_line(indoc! {r##"
                Some(#[thing|]#)
                Some(#(other_thing|)#)
            "##}),
            "<A-o>",
            helpers::platform_line(indoc! {r##"
                Some#[(thing)|]#
                Some#((other_thing)|)#
            "##}),
        ),
        // multi range collision merges
        (
            helpers::platform_line(indoc! {r##"
                (
                    Some(#[thing|]#),
                    Some(#(other_thing|)#),
                )
            "##}),
            "<A-o><A-o><A-o>",
            helpers::platform_line(indoc! {r##"
                #[(
                    Some(thing),
                    Some(other_thing),
                )|]#
            "##}),
        ),
        // multi range collision merges, then shrinks back to original
        (
            helpers::platform_line(indoc! {r##"
                (
                    Some(#[thing|]#),
                    Some(#(other_thing|)#),
                )
            "##}),
            "<A-o><A-o><A-o><A-i>",
            helpers::platform_line(indoc! {r##"
                (
                    #[Some(thing)|]#,
                    #(Some(other_thing)|)#,
                )
            "##}),
        ),
        (
            helpers::platform_line(indoc! {r##"
                (
                    Some(#[thing|]#),
                    Some(#(other_thing|)#),
                )
            "##}),
            "<A-o><A-o><A-o><A-i><A-i>",
            helpers::platform_line(indoc! {r##"
                (
                    Some#[(thing)|]#,
                    Some#((other_thing)|)#,
                )
            "##}),
        ),
        (
            helpers::platform_line(indoc! {r##"
                (
                    Some(#[thing|]#),
                    Some(#(other_thing|)#),
                )
            "##}),
            "<A-o><A-o><A-o><A-i><A-i><A-i>",
            helpers::platform_line(indoc! {r##"
                (
                    Some(#[thing|]#),
                    Some(#(other_thing|)#),
                )
            "##}),
        ),
        // shrink with no expansion history defaults to first child
        (
            helpers::platform_line(indoc! {r##"
                #[(
                    Some(thing),
                    Some(other_thing),
                )|]#
            "##}),
            "<A-i>",
            helpers::platform_line(indoc! {r##"
                (
                    #[Some(thing)|]#,
                    Some(other_thing),
                )
            "##}),
        ),
        // any movement cancels selection history and falls back to first child
        (
            helpers::platform_line(indoc! {r##"
                (
                    Some(#[thing|]#),
                    Some(#(other_thing|)#),
                )

            "##}),
            "<A-o><A-o><A-o>jkvkkk<A-i>",
            helpers::platform_line(indoc! {r##"
                (
                    #[|Some(thing)]#,
                    Some(other_thing),
                )

            "##}),
        ),
    ];

    for test in tests {
        test_with_config(AppBuilder::new().with_file("foo.rs", None), test).await?;
    }

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn expand_selection_around() -> anyhow::Result<()> {
    let tests = vec![
        // single cursor stays single cursor, first goes to end of current
        // node, then parent
        (
            helpers::platform_line(indoc! {r##"
                Some(#[thing|]#)
            "##}),
            "<A-O><A-O>",
            helpers::platform_line(indoc! {r##"
                #[Some(|]#thing#()|)#
            "##}),
        ),
        // shrinking restores previous selection
        (
            helpers::platform_line(indoc! {r##"
                Some(#[thing|]#)
            "##}),
            "<A-O><A-O><A-i><A-i>",
            helpers::platform_line(indoc! {r##"
                Some(#[thing|]#)
            "##}),
        ),
        // multi range collision merges expand as normal, except with the
        // original selection removed from the result
        (
            helpers::platform_line(indoc! {r##"
                (
                    Some(#[thing|]#),
                    Some(#(other_thing|)#),
                )
            "##}),
            "<A-O><A-O><A-O>",
            helpers::platform_line(indoc! {r##"
                #[(
                    Some(|]#thing#(),
                    Some(|)#other_thing#(),
                )|)#
            "##}),
        ),
        (
            helpers::platform_line(indoc! {r##"
                (
                    Some(#[thing|]#),
                    Some(#(other_thing|)#),
                )
            "##}),
            "<A-O><A-O><A-O><A-i><A-i><A-i>",
            helpers::platform_line(indoc! {r##"
                (
                    Some(#[thing|]#),
                    Some(#(other_thing|)#),
                )
            "##}),
        ),
    ];

    for test in tests {
        test_with_config(AppBuilder::new().with_file("foo.rs", None), test).await?;
    }

    Ok(())
}
