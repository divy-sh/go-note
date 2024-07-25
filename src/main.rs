use druid::commands::{SHOW_OPEN_PANEL, SHOW_SAVE_PANEL};
use druid::widget::{Button, Flex, Label, TextBox};
use druid::{AppLauncher, Data, Env, Lens, Widget, WidgetExt, WindowDesc, Command, FileDialogOptions, FileSpec, Target};
use std::fs::File;
use std::io::{Read, Write};
use std::sync::Arc;
use std::path::PathBuf;

#[derive(Clone, Data, Lens)]
struct AppState {
    text: String,
    path: Option<Arc<PathBuf>>,
}

fn build_ui() -> impl Widget<AppState> {
    let textbox = TextBox::multiline()
        .with_placeholder("Enter your text here...")
        .lens(AppState::text)
        .expand();

    let load_button = Button::new("Load")
        .on_click(|ctx, _data: &mut AppState, _env| {
            let options = FileDialogOptions::new()
                .allowed_types(vec![FileSpec::new("Text file", &["txt"])])
                .default_type(FileSpec::new("Text file", &["txt"]))
                .name_label("File Name")
                .title("Choose a file to open")
                .button_text("Open");

            ctx.submit_command(Command::new(SHOW_OPEN_PANEL, options, Target::Auto));
        });

    let save_button = Button::new("Save")
        .on_click(|ctx, data: &mut AppState, _env| {
            if let Some(path) = &data.path {
                save_to_file(path, &data.text);
            } else {
                let options = FileDialogOptions::new()
                    .allowed_types(vec![FileSpec::new("Text file", &["txt"])])
                    .default_type(FileSpec::new("Text file", &["txt"]))
                    .name_label("File Name")
                    .title("Save file as")
                    .button_text("Save");

                ctx.submit_command(Command::new(SHOW_SAVE_PANEL, options, Target::Auto));
            }
        });

    let title_bar = Flex::row()
        .with_child(load_button)
        .with_child(save_button)
        .padding(5.0);

    Flex::column()
        .with_child(title_bar)
        .with_flex_child(textbox, 1.0)
}

fn save_to_file(path: &PathBuf, text: &str) {
    if let Ok(mut file) = File::create(path) {
        let _ = file.write_all(text.as_bytes());
    }
}

fn main() {
    let main_window = WindowDesc::new(build_ui)
        .title("Rust Text Editor")
        .window_size((800.0, 600.0));

    let initial_state = AppState {
        text: String::new(),
        path: None,
    };

    AppLauncher::with_window(main_window)
        .delegate(Delegate)
        .launch(initial_state)
        .expect("Failed to launch application");
}

struct Delegate;

impl druid::AppDelegate<AppState> for Delegate {
    fn command(
        &mut self,
        _ctx: &mut druid::DelegateCtx,
        _target: druid::Target,
        cmd: &druid::Command,
        data: &mut AppState,
        _env: &Env,
    ) -> druid::Handled {
        if let Some(file_info) = cmd.get(druid::commands::OPEN_FILE) {
            if let Ok(mut file) = File::open(&file_info.path()) {
                let mut contents = String::new();
                if file.read_to_string(&mut contents).is_ok() {
                    data.text = contents;
                    data.path = Some(Arc::new(file_info.path().to_path_buf()));
                }
            }
            return druid::Handled::Yes;
        }

        if let Some(file_info) = cmd.get(druid::commands::SAVE_FILE_AS) {
            if let Ok(mut file) = File::create(&file_info.path()) {
                let _ = file.write_all(data.text.as_bytes());
                data.path = Some(Arc::new(file_info.path().to_path_buf()));
            }
            return druid::Handled::Yes;
        }

        druid::Handled::No
    }
}
