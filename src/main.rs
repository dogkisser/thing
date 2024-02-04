#![warn(clippy::pedantic)]
// use anyhow::Result;
// use windows::Win32::UI::Input::KeyboardAndMouse::BlockInput;
// use enigo::{Enigo, Key, KeyboardControllable};
// use std::{path::PathBuf, time::Duration};

fn main() {
}

// #[tokio::main]
// async fn main() -> anyhow::Result<()> {
//     println!("executed as exe");
//     // let all_dirs = std::fs::read_dir("C:/")?
//     //     .flatten()
//     //     .filter(|p| p.metadata().is_ok_and(|m| m.is_dir()))
//     //     .map(|p| p.path())
//     //     .collect::<Vec<PathBuf>>();

//     // #[cfg(debug_assertions)]
//     // let delay = Duration::from_secs(5);
//     // #[cfg(not(debug_assertions))]
//     // let delay = Duration::from_secs(rand::thread_rng().gen_range(30..120) * 60);
//     // tokio::time::sleep(delay).await;

//     // type_out("test!")?;

//     Ok(())
// }

// fn type_out(inp: &str) -> Result<()> {
//     unsafe { BlockInput(true) }?;

//     let mut enigo = Enigo::new();
//     enigo.key_sequence_parse(&inp.to_lowercase());
//     enigo.key_click(Key::Return);

//     unsafe { BlockInput(false) }?;
//     Ok(())
// }

// fn open_search(query: &str) -> Result<()> {
    
// }

// async fn save_random_porn() -> Result<()> {

// }

// async fn get_random_porn() -> Result<Vec<u8>> {
//     let resp = reqwest::
// }