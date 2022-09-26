use crate::Builder;
use inkwell::{
    types::{BasicMetadataTypeEnum, FunctionType},
    AddressSpace,
};
use util::{
    scope::{Scope, ScopeType},
    zx_type::ZXTyped, ast::Statement,
};

impl<'a> Builder<'a> {
    pub fn scope(&self, scope: &Scope) {
        match &scope.scope_type {
            ScopeType::DefFunction {
                parameters,
                block,
                return_type,
            } => self.build_function(&scope.name, parameters, return_type, block),
            _ => {}
        }
    }

    pub fn build_function(&self, name: &String, parameters: &Vec<Scope>, ret_type: &ZXTyped, block: &Statement) {
        let function = self.module.add_function(
            &name,
            self.function_type(
                ret_type,
                self.function_parameters_type(parameters.clone()).as_slice(),
            ),
            None,
        );
        let basic_block = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(basic_block);
        self.statements(block, function);
    }

    pub fn function_type(
        &self,
        ret_type: &ZXTyped,
        param_types: &[BasicMetadataTypeEnum<'a>],
    ) -> FunctionType<'a> {
        match ret_type {
            ZXTyped::Integer { .. } => self.context.i32_type().fn_type(param_types, false),
            ZXTyped::Float { .. } => self.context.f32_type().fn_type(param_types, false),
            ZXTyped::Char { .. } => self.context.i8_type().fn_type(param_types, false),
            ZXTyped::String { .. } => self
                .context
                .i8_type()
                .ptr_type(AddressSpace::Generic)
                .fn_type(param_types, false),
            ZXTyped::Void => self.context.void_type().fn_type(param_types, false),
            _ => self.context.void_type().fn_type(param_types, false),
        }
    }

    pub fn function_parameters_type(
        &self,
        parameters: Vec<Scope>,
    ) -> Vec<BasicMetadataTypeEnum<'a>> {
        parameters
            .iter()
            .map(|parameter| {
                if let ScopeType::DefVariable { var_type } = &parameter.scope_type {
                    Ok(match var_type {
                        ZXTyped::Char { .. } => self.context.i8_type().into(),
                        ZXTyped::String { .. } => self
                            .context
                            .i8_type()
                            .ptr_type(AddressSpace::Generic)
                            .into(),
                        ZXTyped::Integer { .. } => self.context.i32_type().into(),
                        ZXTyped::Float { .. } => self.context.f32_type().into(),
                        ZXTyped::Other(name) => self.structs.find(&name).into(),
                        ZXTyped::Void => todo!("error"),
                    })
                } else {
                    Err(())
                }
            })
            .collect::<Result<Vec<BasicMetadataTypeEnum>, ()>>()
            .unwrap()
    }
}
