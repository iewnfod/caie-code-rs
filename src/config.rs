#[derive(Debug, Clone)]
pub struct ParseConfig {
    pub case_sensitive: bool,
    pub allow_loop_control: bool,
    pub allow_func_return: bool,
    pub allow_undefined_vars: bool,
    pub allow_plus_on_string: bool,
    pub allow_equal_assign: bool,  // = as assignment operator, == as equality operator
    pub allow_new_line_in_string: bool,
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
            allow_equal_assign: true,
            allow_new_line_in_string: false,
        }
    }

    pub fn strict() -> Self {
        Self {
            case_sensitive: false,
            allow_loop_control: false,
            allow_func_return: true,
            allow_undefined_vars: false,
            allow_plus_on_string: false,
            allow_equal_assign: false,
            allow_new_line_in_string: false,
        }
    }
}

impl Default for ParseConfig {
    fn default() -> Self {
        Self::normal()
    }
}
