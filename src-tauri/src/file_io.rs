  use std::path::PathBuf;
  use std::fs;
  use std::fs::OpenOptions;
  use std::io::Write;
  
pub fn load_env(
  file_path:PathBuf
) -> String {
  let input_str = fs::read_to_string(file_path)
    .expect("file read error");

  let input_json :serde_json::Value = 
    serde_json::from_str(&input_str)
    .expect("str toJson error");

  input_json["personal_token"].as_str().unwrap().into()
}

pub fn create_env_file(
  file_path:PathBuf,
  personal_token:String,
) {
  let personal_token_json = serde_json::json!(
    {
      "personal_token" : personal_token.clone(),
    }
  );

let dir_path = &file_path.parent().unwrap();

if dir_path.exists() {
} else {
  fs::create_dir(dir_path)
    .unwrap_or_else(|why| {
      println!("!dir: {:?}", why.kind())});
  }

  let mut output_file = 
    OpenOptions::new()
    .create(true)
    .write(true)
    .open(file_path)
    .expect("file create error");

  let out_str =
    serde_json::to_string(&personal_token_json)
    .expect("json toStr error");
  
  let out_buf = out_str.as_bytes();
  output_file.write_all(out_buf).expect("write file error");
  output_file.flush().expect("flush file error");
}
