use heraclitus_compiler::prelude::*;
use crate::fragments;
use crate::modules::prelude::*;
use crate::modules::block::Block;
use crate::modules::types::Type;
use crate::modules::variable::variable_name_extensions;

#[derive(Debug, Clone)]
pub struct Then {
    pub is_parsed: bool,
    block: Box<Block>,
    param_name: String,
    param_global_id: Option<usize>
}

impl SyntaxModule<ParserMetadata> for Then {
    syntax_name!("Then Expression");

    fn new() -> Self {
        Then {
            is_parsed: false,
            block: Box::new(Block::new().with_needs_noop().with_condition()),
            param_name: String::new(),
            param_global_id: None
        }
    }

    fn parse(&mut self, meta: &mut ParserMetadata) -> SyntaxResult {
        match token(meta, "then") {
            Ok(_) => {
                context!({
                    // Parse the parameter in parentheses
                    token(meta, "(")?;
                    let param_tok = meta.get_current_token();

                    // Check if we immediately hit a closing paren (empty parameter)
                    if token(meta, ")").is_ok() {
                        let pos = PositionInfo::from_between_tokens(meta, param_tok, meta.get_current_token());
                        return error_pos!(meta, pos, "Parameter name cannot be empty");
                    }

                    self.param_name = variable(meta, variable_name_extensions())?;
                    token(meta, ")")?;

                    // Add the parameter variable to the scope and parse the block
                    meta.with_push_scope(|meta| {
                        self.param_global_id = meta.add_var(&self.param_name, Type::Int, false);
                        syntax(meta, &mut *self.block)?;
                        Ok(())
                    })?;

                    if self.block.is_empty() {
                        let message = Message::new_warn_at_token(meta, meta.get_current_token())
                            .message("Empty then block")
                            .comment("You should use 'trust' modifier to run commands without handling errors");
                        meta.add_message(message);
                    }
                    self.is_parsed = true;
                    Ok(())
                }, |pos| {
                    error_pos!(meta, pos, "Failed to parse then block")
                })
            },
            Err(err) => {
                // If we're in a trust context, mark as parsed
                if meta.context.is_trust_ctx {
                    self.is_parsed = true;
                    Ok(())
                } else {
                    Err(err)
                }
            }
        }
    }
}

impl TranslateModule for Then {
    fn translate(&self, meta: &mut TranslateMetadata) -> FragmentKind {
        if self.is_parsed {
            let block = self.block.translate(meta);
            // the condition of '$?' clears the status code thus we need to store it in a variable
            let status_variable_stmt = VarStmtFragment::new("__status", Type::Int, fragments!("$?"));
            let status_variable_expr = VarExprFragment::from_stmt(&status_variable_stmt);

            match &block {
                FragmentKind::Empty => {
                    status_variable_stmt.to_frag()
                },
                FragmentKind::Block(block) if block.statements.is_empty() => {
                    status_variable_stmt.to_frag()
                },
                _ => {
                    let param_assignment = VarStmtFragment::new(&self.param_name, Type::Int, status_variable_expr.to_frag())
                        .with_global_id(self.param_global_id);

                    BlockFragment::new(vec![
                        status_variable_stmt.to_frag(),
                        param_assignment.to_frag(),
                        block,
                    ], false).to_frag()
                }
            }
        } else {
            FragmentKind::Empty
        }
    }
}
