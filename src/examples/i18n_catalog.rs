use glory::i18n::{Args, Catalog, Error, Locale};
use std::rc::Rc;

pub fn catalog() -> Rc<Catalog> {
    let catalog = Catalog::new(
        "en",
        [
            ("en", include_str!("../../locales/demo.en.ftl")),
            ("zh-CN", include_str!("../../locales/demo.zh-CN.ftl")),
        ],
    )
    .expect("valid embedded Fluent resources");
    assert!(catalog.validate_complete().is_empty());
    Rc::new(catalog)
}

pub fn greeting(catalog: &Catalog, locale: &Locale, name: &str) -> Result<String, Error> {
    let mut args = Args::new();
    args.set("name", name);
    catalog.format_exact(locale, "greeting", Some(&args))
}
