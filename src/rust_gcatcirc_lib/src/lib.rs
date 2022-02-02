
pub mod graph_circ;
pub mod graph_code;

pub mod code;

pub fn say_some() -> String {


        let _code = match code::CircCode::new_from_vec(vec!["ADB".to_string(), "BA".to_string(), "".to_string()]) {
            Ok(code) => code,
            Err(e) => {
                eprintln!("{}", e);
                code::CircCode::default()
            } ,
        };



    let a = code::CircCode::default();
    a.get_code().push("A".to_string());
    return a.to_string();
}
