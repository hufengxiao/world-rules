//! 印尼社交礼仪
//!
//! 印尼社交的问候、手势与做客礼节

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: IndonesianEtiquetteRules,
    name: "印尼社交礼仪",
    desc: "印尼社交的问候、手势与做客礼节",
    origin: "印度尼西亚",
    tags: ["跨文化", "东南亚", "礼仪"]
}

impl IndonesianEtiquetteRules {
    /// 问候方式
    pub fn greeting(&self) -> Vec<&'static str> {
        vec![
            "见面微笑点头",
            "握手轻柔不使劲",
            "双手合十问候",
            "对异性问候较含蓄",
        ]
    }

    /// 手势禁忌
    pub fn gestures(&self) -> Vec<&'static str> {
        vec![
            "不用左手递物",
            "不触摸他人头部",
            "不指人用食指",
            "不叉腰站立说话",
        ]
    }

    /// 做客礼节
    pub fn visiting(&self) -> Vec<&'static str> {
        vec!["进门脱鞋", "接受主人招待", "告辞礼貌致谢", "带小礼物得体"]
    }

    /// 相处分寸
    pub fn interaction(&self) -> Vec<&'static str> {
        vec![
            "尊重宗教信仰",
            "衣着得体保守",
            "说话温和委婉",
            "避免公开争执",
        ]
    }
}

impl Rule for IndonesianEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("indonesian_etiquette")
    }

    fn explain(&self) -> String {
        format!(
            "【印尼社交礼仪】\n{}",
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
                    "手势禁忌：\\n{}",
                    self.gestures()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "做客礼节：\\n{}",
                    self.visiting()
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
    fn test_indonesianetiquetterules_basic() {
        let rules = IndonesianEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "印尼社交礼仪");
        assert!(!rules.greeting().is_empty());
        assert!(!rules.gestures().is_empty());
        assert!(!rules.visiting().is_empty());
        assert!(!rules.interaction().is_empty());
    }

    #[test]
    fn test_indonesianetiquetterules_validation() {
        let rules = IndonesianEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(
            rules.category(),
            RuleCategory::social("indonesian_etiquette")
        );
    }

    #[test]
    fn test_indonesianetiquetterules_explain() {
        let rules = IndonesianEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("问候方式"));
        assert!(e.contains("手势禁忌"));
        assert!(e.contains("做客礼节"));
    }
}
