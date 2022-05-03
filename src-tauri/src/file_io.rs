use anyhow::{anyhow, Result};
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct TokenJson {
    personal_token: String,
}

pub fn load_env(file_path: PathBuf) -> Result<String> {
    let input_str = fs::read_to_string(file_path);

    let input_str = match input_str {
        Ok(str) => str,
        Err(e) => return Err(anyhow!(e)),
    };

    let input_json: TokenJson = serde_json::from_str(&input_str)?;

    Ok(input_json.personal_token.into())
}

pub fn create_env_file(file_path: PathBuf, personal_token: String) -> Result<()> {
    let personal_token_json = serde_json::json!(TokenJson {
        personal_token: personal_token.clone()
    });

    let dir_path = &file_path.parent().unwrap();

    if dir_path.exists() {
    } else {
        let dir_created = fs::create_dir(dir_path);

        match dir_created {
            Ok(dir) => dir,
            Err(e) => return Err(anyhow!(e)),
        };
    }

    let output_file = OpenOptions::new().create(true).write(true).open(file_path);

    let mut result_output_file = match output_file {
        Ok(file) => file,
        Err(e) => return Err(anyhow!(e)),
    };

    let out_str = serde_json::to_string(&personal_token_json).expect("json toStr error");

    let out_buf = out_str.as_bytes();
    result_output_file.write_all(out_buf)?;
    result_output_file.flush()?;

    Ok(())
}
