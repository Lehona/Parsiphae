pub mod zCPar_Symbol;
pub mod collector;
pub mod zSymb_Table;


fn format_name(name: &str, scope: Option<&String>) -> String {
    if scope.is_none() {
        name.to_string()
    } else {
        format!("{}.{}", scope.unwrap(), name)
    }
}