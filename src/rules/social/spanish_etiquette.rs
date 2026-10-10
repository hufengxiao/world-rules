//! 西班牙社交礼仪
//!
//! 西班牙日常社交的问候、用餐与作息礼节

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: SpanishEtiquetteRules,
    name: "西班牙社交礼仪",
    desc: "西班牙日常社交的问候、用餐与作息礼节",
    origin: "西班牙",
    tags: ["跨文化", "欧洲", "礼仪"]
}

impl SpanishEtiquetteRules {
    /// 问候方式
    pub fn greeting(&self) -> Vec<&'static str> {
        vec![
            "见面常贴面颊两次",
            "握手用于正式场合",
            "问候热情亲切",
            "熟人见面拥抱",
        ]
    }

    /// 用餐礼仪
    pub fn dining(&self) -> Vec<&'static str> {
        vec![
            "午餐为主餐较晚",
            "晚餐常在九点后",
            "用餐时间长爱聊天",
            "用餐时双手放桌面",
        ]
    }

    /// 作息习惯
    pub fn schedule(&self) -> Vec<&'static str> {
        vec![
            "午后有午休时段",
            "商店午后歇业",
            "社交聚会时间晚",
            "守时相对宽松",
        ]
    }

    /// 相处分寸
    pub fn interaction(&self) -> Vec<&'static str> {
        vec![
            "交谈热情声音洪亮",
            "喜欢肢体接触",
            "话题轻松随意",
            "重视人情往来",
        ]
    }
}

impl Rule for SpanishEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("spanish_etiquette")
    }

    fn explain(&self) -> String {
        format!(
            "【西班牙社交礼仪】\n{}",
            [
                format!(
                    "问候方式：\\n{}",
                    self.greeting()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "用餐礼仪：\\n{}",
                    self.dining()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "作息习惯：\\n{}",
                    self.schedule()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "相处分寸：\\n{}",
                    self.interaction()
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
    fn test_spanishetiquetterules_basic() {
        let rules = SpanishEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "西班牙社交礼仪");
        assert!(!rules.greeting().is_empty());
        assert!(!rules.dining().is_empty());
        assert!(!rules.schedule().is_empty());
        assert!(!rules.interaction().is_empty());
    }

    #[test]
    fn test_spanishetiquetterules_validation() {
        let rules = SpanishEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("spanish_etiquette"));
    }

    #[test]
    fn test_spanishetiquetterules_explain() {
        let rules = SpanishEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("问候方式"));
        assert!(e.contains("用餐礼仪"));
        assert!(e.contains("作息习惯"));
    }
}
