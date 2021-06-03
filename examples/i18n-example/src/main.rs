use i18n_rs::{lang, i18n};

fn main() {
    lang!("en-us");
    print();
    lang!("zh-cn"; loose);
    print();
}

fn print() {
    println!("{} {} {} {}",
             i18n!("hello-world"),
             i18n!("log.level"; loose),
             i18n!("words.is"),
             i18n!("log.debug")
    );
}
