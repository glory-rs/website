use glory::i18n::{Args, Catalog, I18n};
use std::rc::Rc;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Locale {
    #[default]
    En,
    ZhCn,
}

thread_local! {
    // Only immutable resources are shared. The current language lives in SiteRoute.
    static CATALOG: Rc<Catalog> = {
        let catalog = Catalog::new("en", [
            ("en", include_str!("../locales/en.ftl")),
            ("zh-CN", include_str!("../locales/zh-CN.ftl")),
        ]).expect("valid embedded Fluent resources");
        assert!(catalog.validate_complete().is_empty(), "incomplete website translations");
        Rc::new(catalog)
    };
}

impl Locale {
    pub const ALL: [Self; 2] = [Self::En, Self::ZhCn];
    pub fn tag(self) -> &'static str {
        match self {
            Self::En => "en",
            Self::ZhCn => "zh-CN",
        }
    }
    pub fn other(self) -> Self {
        match self {
            Self::En => Self::ZhCn,
            Self::ZhCn => Self::En,
        }
    }
    pub fn native_name(self) -> &'static str {
        match self {
            Self::En => "English",
            Self::ZhCn => "简体中文",
        }
    }
    pub fn context(self) -> I18n {
        I18n::new(catalog(), self.tag().parse::<glory::i18n::Locale>().expect("static locale"))
    }
    pub fn text(self, key: &str) -> String {
        self.format(key, None)
    }
    pub fn format(self, key: &str, args: Option<&Args<'_>>) -> String {
        catalog()
            .format_exact(&self.tag().parse().expect("static locale"), key, args)
            .unwrap_or_else(|error| panic!("website translation {key}: {error}"))
    }
}

pub fn catalog() -> Rc<Catalog> {
    CATALOG.with(Rc::clone)
}
