#[derive(Debug, Clone)]
pub struct ParseConfig {
    pub case_sensitive: bool,
    pub allow_loop_control: bool,
    pub allow_func_return: bool,
    pub allow_undefined_vars: bool,
    pub allow_plus_on_string: bool,
    // keywords: KeywordTable,
    // operators: OperatorTable,
}

impl ParseConfig {
    pub fn normal() -> Self {
        Self {
            case_sensitive: true,
            allow_loop_control: true,
            allow_func_return: true,
            allow_undefined_vars: false,
            allow_plus_on_string: true,
        }
    }

    pub fn strict() -> Self {
        Self {
            case_sensitive: false,
            allow_loop_control: false,
            allow_func_return: true,
            allow_undefined_vars: false,
            allow_plus_on_string: false,
        }
    }
}

impl Default for ParseConfig {
    fn default() -> Self {
        Self::normal()
    }
}
