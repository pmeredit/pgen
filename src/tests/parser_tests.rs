#[test]
fn parse_topexpr_test() {
    assert_eq!(
        &format!("{:?}", ::parser::parse_TopExpr("22 * 44 + 66").unwrap()),
        "Op(Op(Number(22), Mul, Number(44)), Add, Number(66))"
    );
    assert_eq!(
        &format!(
            "{:?}",
            ::parser::parse_TopExpr("3 + (let x=1 in x) + $y + (if foo(x,y,z) then false else x)",)
                .unwrap()
        ),
        "Op(Op(Op(Number(3), Add, Let(Let { assignments: [(\"x\", Number(1))], expr: ID(\"x\") \
         })), Add, Col(\"$y\")), Add, Cond(Cond { cond: App(\"foo\", [ID(\"x\"), ID(\"y\"), \
         ID(\"z\")]), then: Bool(false), otherwise: ID(\"x\") }))"
    );

    assert_eq!(
        &format!(
            "{:?}",
            ::parser::parse_TopExpr("let x = 2 in switch{ x+4 => x, true => y default: '42'}")
                .unwrap()
        ),
        "Let(Let { assignments: [(\"x\", Number(2))], expr: Switch(Switch { cases: \
         [(Op(ID(\"x\"), Add, Number(4)), ID(\"x\")), (Bool(true), ID(\"y\"))], default: \
         Str(\"42\") }) })"
    );

    assert_eq!(
        &format!(
            "{:?}",
            ::parser::parse_TopExpr("foo($x==3 || $y && $z <= null, let z=3,y=42 in z+foo(41+y))",)
                .unwrap()
        ),
        "App(\"foo\", [Op(Op(Col(\"$x\"), Eq, Number(3)), Or, Op(Col(\"$y\"), And, \
         Op(Col(\"$z\"), Lte, Null))), Let(Let { assignments: [(\"z\", Number(3)), (\"y\", \
         Number(42))], expr: Op(ID(\"z\"), Add, App(\"foo\", [Op(Number(41), Add, ID(\"y\"))])) \
         })])"
    );

    assert_eq!(
        &format!(
            "{:?}",
            ::parser::parse_TopExpr("{'zed' : [1,let x = 3 in x+42,3+x], 'hello' : 'world'}")
                .unwrap()
        ),
        "Object([(\"zed\", Array([Number(1), Let(Let { assignments: [(\"x\", Number(3))], expr: \
         Op(ID(\"x\"), Add, Number(42)) }), Op(Number(3), Add, ID(\"x\"))])), (\"hello\", \
         Str(\"world\"))])"
    );
}

#[test]
fn parse_pipeline_test() {
    assert_eq!(
        &format!(
            "{:?}",
            ::parser::parse_Pipeline(
                "project: {'x':true, 'y': 3 + (let z = $y in foo(z,3.4,-23,[1,2,3,4])) + \
                 23};match: {'hello' : 42}; sort: {'world' : -1}",
            ).unwrap()
        ),
        "Pipeline { stages: [PipelineItem { stage_name: \"project\", object: Object([(\"x\", \
         Bool(true)), (\"y\", Op(Op(Number(3), Add, Let(Let { assignments: [(\"z\", \
         Col(\"$y\"))], expr: App(\"foo\", [ID(\"z\"), Float(3.4), Number(-23), Array([Number(1), \
         Number(2), Number(3), Number(4)])]) })), Add, Number(23)))]) }, PipelineItem { \
         stage_name: \"match\", object: Object([(\"hello\", Number(42))]) }, PipelineItem { \
         stage_name: \"sort\", object: Object([(\"world\", Number(-1))]) }] }"
    );
}
