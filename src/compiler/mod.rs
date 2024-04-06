mod cpp_compiler;

pub enum LangType {
    Cpp
}

pub fn compile(lang: LangType, code: &str) -> Result<String, String> {
    match lang {
        LangType::Cpp => cpp_compiler::compile(code),
    }
}