use i18n_rs::{lang, i18n};

fn main() {
    lang!("en-us");
    print();
    lang!("zh-cn"; loose);
    print();

    // i18n!("this.nonexistent") or lang!("this-nonexistent") will cause a compile-time error.
    assert_eq!(i18n!("this.nonexistent"; loose), "this.nonexistent");
    lang!("this-nonexistent"; loose);
    assert_eq!(i18n!("hello-world"; loose), "hello-world");
}

fn print() {
    println!("{} {} {} {}",
             i18n!("hello-world"),
             i18n!("log.level"; loose),
             i18n!("words.is"),
             i18n!("log.debug")
    );
}
