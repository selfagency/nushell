use nu_protocol::ast::Call;
use nu_protocol::engine::{Command, EngineState, Stack};
use nu_protocol::{
    Category, Example, PipelineData, ShellError, Signature, SyntaxShape, Type, Value,
};

#[derive(Clone)]
pub struct ExportConst;

impl Command for ExportConst {
    fn name(&self) -> &str {
        "export const"
    }

    fn usage(&self) -> &str {
        "Use parse-time constant from a module and export them from this module."
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build("export const")
            .input_output_types(vec![(Type::Nothing, Type::Nothing)])
            .allow_variants_without_examples(true)
            .required("const_name", SyntaxShape::VarWithOptType, "constant name")
            .required(
                "initial_value",
                SyntaxShape::Keyword(b"=".to_vec(), Box::new(SyntaxShape::MathExpression)),
                "equals sign followed by constant value",
            )
            .category(Category::Core)
    }

    fn extra_usage(&self) -> &str {
        r#"This command is a parser keyword. For details, check:
  https://www.nushell.sh/book/thinking_in_nu.html"#
    }

    fn is_parser_keyword(&self) -> bool {
        true
    }

    fn run(
        &self,
        _engine_state: &EngineState,
        _stack: &mut Stack,
        _call: &Call,
        _input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        Ok(PipelineData::empty())
    }

    fn examples(&self) -> Vec<Example> {
        vec![Example {
            description: "Re-export a command from another module",
            example: r#"module spam { export const foo = 3; }
    module eggs { export use spam foo }
    use eggs foo
    foo
            "#,
            result: Some(Value::test_int(3)),
        }]
    }

    fn search_terms(&self) -> Vec<&str> {
        vec!["reexport", "import", "module"]
    }
}
