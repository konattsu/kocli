use copypasta::{ClipboardContext, ClipboardProvider};

pub fn run() {
    let mut ctx = ClipboardContext::new().expect("Failed to create clipboard content");

    let clipboard_text = ctx
        .get_contents()
        .expect("Failed to get clipboard contents");

    let msg = more_delimiters(&clipboard_text);
    ctx.set_contents(msg.to_owned())
        .expect("Failed to set clipboard contents");
    println!("Clipboard set to: {}", msg)
}

fn more_delimiters(clipboard_text: &str) -> String {
    const FROM: &str = "\\";
    const TO: &str = "/";

    clipboard_text.trim().replace(FROM, TO)
}
