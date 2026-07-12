//! 国际化支持
//!
//! 为规则内容提供中英双语支持。

/// 语言设置
///
/// 支持中英双语，用于规则内容的国际化。
///
/// # Examples
/// ```
/// use world_rules::i18n::Language;
///
/// assert_eq!(Language::Chinese, Language::Chinese);
/// assert_ne!(Language::Chinese, Language::English);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    /// 中文
    Chinese,
    /// 英文
    English,
}

/// 双语文本
///
/// 包含中英两种语言版本的静态文本，用于规则说明的国际化。
///
/// # Examples
/// ```
/// use world_rules::i18n::{LocalizedText, Language};
///
/// let text = LocalizedText::new("中文内容", "English content");
/// assert_eq!(text.get(Language::Chinese), "中文内容");
/// assert_eq!(text.get(Language::English), "English content");
/// ```
#[derive(Debug, Clone)]
pub struct LocalizedText {
    /// 中文版本文本
    pub zh: &'static str,
    /// 英文版本文本
    pub en: &'static str,
}

impl LocalizedText {
    /// 创建新的双语文本
    ///
    /// # Arguments
    ///
    /// * `zh` - 中文文本
    /// * `en` - 英文文本
    ///
    /// # Examples
    /// ```
    /// use world_rules::i18n::LocalizedText;
    ///
    /// let text = LocalizedText::new("你好", "Hello");
    /// assert_eq!(text.zh, "你好");
    /// assert_eq!(text.en, "Hello");
    /// ```
    pub const fn new(zh: &'static str, en: &'static str) -> Self {
        Self { zh, en }
    }

    /// 根据语言获取对应的文本
    ///
    /// # Arguments
    ///
    /// * `lang` - 语言选择（Chinese 或 English）
    ///
    /// # Examples
    /// ```
    /// use world_rules::i18n::{LocalizedText, Language};
    ///
    /// let text = LocalizedText::new("规则", "Rule");
    /// assert_eq!(text.get(Language::Chinese), "规则");
    /// assert_eq!(text.get(Language::English), "Rule");
    /// ```
    pub fn get(&self, lang: Language) -> &'static str {
        match lang {
            Language::Chinese => self.zh,
            Language::English => self.en,
        }
    }
}

/// 双语规则元数据
///
/// 包含中英双语版本的规则名称和描述，用于国际化规则展示。
///
/// # Examples
/// ```
/// use world_rules::i18n::LocalizedMetadata;
///
/// let meta = LocalizedMetadata::new(
///     "足球规则", "Football Rules",
///     "FIFA足球比赛规则", "FIFA Football Match Rules"
/// );
/// assert_eq!(meta.name.zh, "足球规则");
/// assert_eq!(meta.name.en, "Football Rules");
/// ```
#[derive(Debug, Clone)]
pub struct LocalizedMetadata {
    /// 规则名称（双语）
    pub name: LocalizedText,
    /// 规则描述（双语）
    pub description: LocalizedText,
}

impl LocalizedMetadata {
    /// 创建新的双语元数据
    ///
    /// # Arguments
    ///
    /// * `name_zh` - 中文规则名称
    /// * `name_en` - 英文规则名称
    /// * `desc_zh` - 中文规则描述
    /// * `desc_en` - 英文规则描述
    ///
    /// # Examples
    /// ```
    /// use world_rules::i18n::LocalizedMetadata;
    ///
    /// let meta = LocalizedMetadata::new(
    ///     "麻将", "Mahjong",
    ///     "中国麻将规则", "Chinese Mahjong Rules"
    /// );
    /// ```
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
///
/// 为规则实现双语支持，提供国际化版本的元数据和说明。
///
/// # Examples
///
/// ```rust
/// use world_rules::i18n::{LocalizedRule, LocalizedMetadata, Language};
///
/// struct MyLocalizedRule {
///     localized_meta: LocalizedMetadata,
/// }
///
/// impl LocalizedRule for MyLocalizedRule {
///     fn localized_metadata(&self) -> Option<&LocalizedMetadata> {
///         Some(&self.localized_meta)
///     }
/// }
///
/// let rule = MyLocalizedRule {
///     localized_meta: LocalizedMetadata::new(
///         "麻将规则", "Mahjong Rules",
///         "中国麻将游戏规则", "Chinese Mahjong Game Rules"
///     ),
/// };
///
/// let chinese = rule.explain_in(Language::Chinese);
/// assert!(chinese.contains("麻将规则"));
/// ```
pub trait LocalizedRule {
    /// 获取双语元数据
    ///
    /// 返回规则的国际化元数据，如果规则不支持双语则返回 None。
    fn localized_metadata(&self) -> Option<&LocalizedMetadata> {
        None
    }

    /// 以指定语言生成规则说明
    ///
    /// # Arguments
    ///
    /// * `lang` - 目标语言（Chinese 或 English）
    ///
    /// # Returns
    ///
    /// 返回指定语言版本的规则说明文本。如果规则不支持双语，返回空字符串。
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

/// 示例双语元数据 - 足球规则
///
/// 用于演示双语元数据的使用方式。
pub const FOOTBALL_I18N: LocalizedMetadata = LocalizedMetadata::new(
    "足球规则",
    "Football Rules",
    "FIFA 足球比赛规则",
    "FIFA Football Match Rules",
);

/// 示例双语元数据 - 篮球规则
///
/// 用于演示双语元数据的使用方式。
pub const BASKETBALL_I18N: LocalizedMetadata = LocalizedMetadata::new(
    "篮球规则",
    "Basketball Rules",
    "NBA/FIBA 篮球比赛规则",
    "NBA/FIBA Basketball Match Rules",
);

/// 示例双语元数据 - 麻将规则
///
/// 用于演示双语元数据的使用方式。
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
