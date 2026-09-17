//! 重阳敬老礼仪
//!
//! 重阳节探望长辈、表达孝心与陪伴敬老的礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ChongyangFilialEtiquetteRules,
    name: "重阳敬老礼仪",
    desc: "重阳节探望长辈、表达孝心与陪伴敬老的礼仪",
    origin: "中国",
    tags: ["社交", "礼仪", "重阳", "敬老"]
}

impl ChongyangFilialEtiquetteRules {
    /// 探望陪伴
    pub fn visit(&self) -> Vec<&'static str> {
        vec![
            "节日主动探望长辈",
            "陪伴聊天多听心事",
            "关心身体健康生活",
            "经常联系送关怀",
        ]
    }

    /// 孝心表达
    pub fn filial(&self) -> Vec<&'static str> {
        vec![
            "表达感恩与敬意",
            "帮助解决实际困难",
            "尊重长辈意愿",
            "有耐心不嫌麻烦",
        ]
    }

    /// 敬老举止
    pub fn manner(&self) -> Vec<&'static str> {
        vec![
            "称谓礼貌敬语",
            "让长辈先行先食",
            "倾听不打断",
            "关照腿脚安全",
        ]
    }

    /// 传承美德
    pub fn virtue(&self) -> Vec<&'static str> {
        vec![
            "带动后辈敬老",
            "关怀孤寡老人",
            "发扬孝老传统",
            "让爱老成为日常",
        ]
    }
}

impl Rule for ChongyangFilialEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("chongyang")
    }

    fn explain(&self) -> String {
        format!(
            "【重阳敬老礼仪】\n{}",
            [
                format!(
                    "探望陪伴：\\n{}",
                    self.visit()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "孝心表达：\\n{}",
                    self.filial()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "敬老举止：\\n{}",
                    self.manner()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "传承美德：\\n{}",
                    self.virtue()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
            ]
            .join("\n\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::core::ValidateContext;

    #[test]
    fn test_chongyangfilialetiquetterules_basic() {
        let rules = ChongyangFilialEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "重阳敬老礼仪");
        assert!(!rules.visit().is_empty());
        assert!(!rules.filial().is_empty());
        assert!(!rules.manner().is_empty());
        assert!(!rules.virtue().is_empty());
    }

    #[test]
    fn test_chongyangfilialetiquetterules_validation() {
        let rules = ChongyangFilialEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("chongyang"));
    }

    #[test]
    fn test_chongyangfilialetiquetterules_explain() {
        let rules = ChongyangFilialEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("探望陪伴"));
        assert!(e.contains("孝心表达"));
        assert!(e.contains("敬老举止"));
    }
}
