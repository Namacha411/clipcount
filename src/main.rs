use winrt_notification::{Duration, Sound, Toast};
use clipboard_win::{formats, get_clipboard};

fn main() {
    let text: String = get_clipboard(formats::Unicode).unwrap();
    let lines = text.lines().count();
    let len = text.len();
    Toast::new(Toast::POWERSHELL_APP_ID)
        .title(&text)
        .text1(&format!("文字数: {}", len))
        .text2(&format!("行数: {}", lines))
        .sound(Some(Sound::Default))
        .duration(Duration::Short)
        .show()
        .expect("unable to toast");
}
