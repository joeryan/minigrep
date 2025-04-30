// pub fn run (query: string, file_path: string) -> string{
//     println!("searching for {query}");
//     println!("In file {file_path}");

//     let contents = fs::read_to_string(file_path)
//         .expect("Should have been able to read file");

//     println!("With Text:\n{contents}");
// }

// fn parse_config(args: &[String]) -> (&str, &str)  {
//     // if args.len() < 3 {panic!("not enough args")};
//     let query = &args[1]; 
//     let file_path = &args[2];
//     (query, file_path)    
// }