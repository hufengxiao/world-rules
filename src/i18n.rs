//! 国际化支持
//!
//! 为规则内容提供中英双语支持。

/// 语言设置
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    Chinese,
    English,
}

/// 双语文本
#[derive(Debug, Clone)]
pub struct LocalizedText {
    pub zh: &'static str,
    pub en: &'static str,
}

impl LocalizedText {
    pub const fn new(zh: &'static str, en: &'static str) -> Self {
        Self { zh, en }
    }

    pub fn get(&self, lang: Language) -> &'static str {
        match lang {
            Language::Chinese => self.zh,
            Language::English => self.en,
        }
    }
}

/// 双语规则元数据
#[derive(Debug, Clone)]
pub struct LocalizedMetadata {
    pub name: LocalizedText,
    pub description: LocalizedText,
}

impl LocalizedMetadata {
    pub const fn new(
        name_zh: &'static str,
        name_en: &'static str,
        desc_zh: &'static str,
        desc_en: &'static str,
    ) -> Self {
        Self {
            name: LocalizedText::new(name_zh, name_en),
            description: LocalizedText::new(desc_zh, desc_en),
        }
    }
}

/// 支持双语的规则 trait 扩展
pub trait LocalizedRule {
    fn localized_metadata(&self) -> Option<&LocalizedMetadata> {
        None
    }

    fn explain_in(&self, lang: Language) -> String {
        match self.localized_metadata() {
            Some(meta) => {
                format!(
                    "【{}】\n{}",
                    meta.name.get(lang),
                    meta.description.get(lang)
                )
            }
            None => String::new(),
        }
    }
}

/// 示例双语元数据
pub const FOOTBALL_I18N: LocalizedMetadata = LocalizedMetadata::new(
    "足球规则",
    "Football Rules",
    "FIFA 足球比赛规则",
    "FIFA Football Match Rules",
);

pub const BASKETBALL_I18N: LocalizedMetadata = LocalizedMetadata::new(
    "篮球规则",
    "Basketball Rules",
    "NBA/FIBA 篮球比赛规则",
    "NBA/FIBA Basketball Match Rules",
);

pub const MAHJONG_I18N: LocalizedMetadata = LocalizedMetadata::new(
    "麻将规则",
    "Mahjong Rules",
    "中国麻将游戏规则",
    "Chinese Mahjong Game Rules",
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_localized_text() {
        let text = LocalizedText::new("中文", "English");
        assert_eq!(text.get(Language::Chinese), "中文");
        assert_eq!(text.get(Language::English), "English");
    }

    #[test]
    fn test_localized_metadata() {
        assert_eq!(FOOTBALL_I18N.name.get(Language::Chinese), "足球规则");
        assert_eq!(FOOTBALL_I18N.name.get(Language::English), "Football Rules");
    }
}
