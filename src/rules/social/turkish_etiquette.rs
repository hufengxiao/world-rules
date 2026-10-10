//! 土耳其社交礼仪
//!
//! 土耳其社交的问候、待客与用餐礼节

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: TurkishEtiquetteRules,
    name: "土耳其社交礼仪",
    desc: "土耳其社交的问候、待客与用餐礼节",
    origin: "土耳其",
    tags: ["跨文化", "中东", "礼仪"]
}

impl TurkishEtiquetteRules {
    /// 问候方式
    pub fn greeting(&self) -> Vec<&'static str> {
        vec![
            "见面握手或拥抱",
            "长辈可行贴手礼",
            "问候询问近况",
            "尊重长者",
        ]
    }

    /// 待客之道
    pub fn hospitality(&self) -> Vec<&'static str> {
        vec![
            "热情招待来客",
            "奉茶咖啡不可拒",
            "客人进门先寒暄",
            "送客至门口",
        ]
    }

    /// 用餐礼仪
    pub fn dining(&self) -> Vec<&'static str> {
        vec![
            "多用右手取食",
            "面包撕开蘸食",
            "主人劝菜宜品尝",
            "餐后饮茶收尾",
        ]
    }

    /// 相处分寸
    pub fn interaction(&self) -> Vec<&'static str> {
        vec![
            "尊重宗教信仰",
            "避免敏感政治话题",
            "进门脱鞋",
            "礼让长者与客人",
        ]
    }
}

impl Rule for TurkishEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("turkish_etiquette")
    }

    fn explain(&self) -> String {
        format!(
            "【土耳其社交礼仪】\n{}",
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
                    "待客之道：\\n{}",
                    self.hospitality()
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
    fn test_turkishetiquetterules_basic() {
        let rules = TurkishEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "土耳其社交礼仪");
        assert!(!rules.greeting().is_empty());
        assert!(!rules.hospitality().is_empty());
        assert!(!rules.dining().is_empty());
        assert!(!rules.interaction().is_empty());
    }

    #[test]
    fn test_turkishetiquetterules_validation() {
        let rules = TurkishEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("turkish_etiquette"));
    }

    #[test]
    fn test_turkishetiquetterules_explain() {
        let rules = TurkishEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("问候方式"));
        assert!(e.contains("待客之道"));
        assert!(e.contains("用餐礼仪"));
    }
}
