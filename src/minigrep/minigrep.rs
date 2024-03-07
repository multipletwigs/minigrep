use std::fs;


struct Config {
  query: String,
  file_path: String,
}

impl Config {
  fn new(args: &[String]) -> Result<Config, String> {
    if !(args.len() == 3){
      let given_args: usize = args.len() - 1;
      return Err(format!("Require exactly 2 args but given {given_args}")); 
    } 
  
    let query: String = args.get(1).unwrap().clone(); 
    let file_path: String = args.get(2).unwrap().clone(); 
  
    let returned_config = Config { 
      query, file_path
    }; 
  
    Ok(returned_config)
  }
}


pub fn minigrep() {
  // it's okay to use collect into a vec
  let args: Vec<String> = std::env::args().collect();

  let configs = Config::new(&args); 

  match configs {
    Ok(config_ok) => {
      println!("Query: {}", config_ok.query);
      println!("File path: {}", config_ok.file_path);

      let content = fs::read_to_string(config_ok.file_path).unwrap(); 
      println!("Content of file is:\n{content}")
    }, 
    Err(config_err) => {
      println!("{}", format!("Something went wrong: {config_err}"))
    }
  }
}
