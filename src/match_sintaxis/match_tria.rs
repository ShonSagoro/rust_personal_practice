pub fn match_do_something(select:i32 )->&'static str {
    match select  {
        1 => return "Hola",
        2 => return  "Hello",
        3 => return  "Nose",
        4..=10 => return  "jsjsjs, ya le entendi",
        _=> return "No, en realidad no entendi"
    }
}