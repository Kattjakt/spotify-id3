use colored::Colorize;
use id3::Tag;

pub fn print(tag: &Tag) {
    println!("");

    for frame in tag.frames() {
        let id = frame.id();
        let content = frame.content();

        if let Some(text) = content.text() {
          let value = text.replace("\0", ", ");
          println!("{0} {1}",id.dimmed(), &value)
        }

        if let Some(text) = content.link() {
          let value = text.replace("\0", ", ");
          println!("{0} {1}", id.dimmed(), &value.blue())
        }
    }

    println!("");

    for frame in tag.frames() {
        let id = frame.id();
        let content = frame.content();

        if let Some(text) = content.extended_text() {
            println!("{0} {1: <18} {2}", id.dimmed(), text.description.magenta(), text.value)
        }
    }

    println!("");
}