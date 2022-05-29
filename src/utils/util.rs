
pub fn is_empty(s: &Option<String>) -> bool{

    match s{
        Some(s) => {
            if s.trim().len() == 0 {
                true
            }else{
                false
            }
        },
        None => true,
    }
}