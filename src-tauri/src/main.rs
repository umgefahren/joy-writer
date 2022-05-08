#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use lazy_static::lazy_static;

lazy_static! {
  static ref COMRAK_WORKER: std::sync::Mutex<std::sync::mpsc::Sender<SenderMessage>> = {
    let (sender, receiver) = std::sync::mpsc::channel::<SenderMessage>();

    std::thread::spawn(move || {
      let syntect_adapter = comrak::plugins::syntect::SyntectAdapter::new("base16-ocean.dark");

      let mut render_plugins = comrak::ComrakRenderPlugins::default();

      render_plugins.codefence_syntax_highlighter = Some(&syntect_adapter);

      let mut plug = comrak::ComrakPlugins::default();
      plug.render = render_plugins;

      let options = Default::default();

      loop {
        let message = receiver.recv().unwrap();

        let result_string =
          comrak::markdown_to_html_with_plugins(&message.markdown, &options, &plug);

        message.back.send(result_string).unwrap();
      }
    });

    std::sync::Mutex::new(sender)
  };
}

pub struct SenderMessage {
  markdown: String,
  back: std::sync::mpsc::Sender<String>,
}

#[tauri::command]
fn markdown_to_html(markdown: String) -> String {
  let (sender, receiver) = std::sync::mpsc::channel();

  let message = SenderMessage {
    markdown,
    back: sender,
  };

  {
    let lock = COMRAK_WORKER.lock().unwrap();

    lock.send(message).unwrap();
  }

  receiver.recv().unwrap()
}

use tauri::*;

fn main() {
  let quit = CustomMenuItem::new("quit".to_string(), "Quit");
  let close = CustomMenuItem::new("close".to_string(), "Close");
  let submenu = Submenu::new("File", Menu::new().add_item(quit).add_item(close));
  let menu = Menu::new()
    .add_native_item(MenuItem::Copy)
    .add_item(CustomMenuItem::new("hide", "Hide"))
    .add_submenu(submenu);

  tauri::Builder::default()
    .menu(menu)
    .on_menu_event(|event| match event.menu_item_id() {
      "quit" => {
        std::process::exit(0);
      }
      "close" => {
        event.window().close().unwrap();
      }
      _ => {}
    })
    .invoke_handler(tauri::generate_handler![markdown_to_html])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
