//! 越南社交礼仪
//!
//! 越南社交的问候、称呼与餐桌礼节

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: VietnameseEtiquetteRules,
    name: "越南社交礼仪",
    desc: "越南社交的问候、称呼与餐桌礼节",
    origin: "越南",
    tags: ["跨文化", "东南亚", "礼仪"]
}

impl VietnameseEtiquetteRules {
    /// 问候方式
    pub fn greeting(&self) -> Vec<&'static str> {
        vec![
            "见面微笑点头致意",
            "双手合十问候",
            "握手男女有别",
            "问候依年龄称呼",
        ]
    }

    /// 称呼讲究
    pub fn address(&self) -> Vec<&'static str> {
        vec![
            "按年龄辈分称呼",
            "用亲属称谓显亲近",
            "对长者加尊称",
            "名字顺序姓前名后",
        ]
    }

    /// 餐桌礼仪
    pub fn dining(&self) -> Vec<&'static str> {
        vec![
            "共用菜肴用公筷",
            "长辈先动筷",
            "双手递接物品",
            "用餐轻声细语",
        ]
    }

    /// 相处分寸
    pub fn interaction(&self) -> Vec<&'static str> {
        vec![
            "避免触碰他人头部",
            "公共场合注意言行",
            "尊重长辈与老师",
            "不随意拍摄寺庙",
        ]
    }
}

impl Rule for VietnameseEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("vietnamese_etiquette")
    }

    fn explain(&self) -> String {
        format!(
            "【越南社交礼仪】\n{}",
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
                    "称呼讲究：\\n{}",
                    self.address()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "餐桌礼仪：\\n{}",
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
    fn test_vietnameseetiquetterules_basic() {
        let rules = VietnameseEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "越南社交礼仪");
        assert!(!rules.greeting().is_empty());
        assert!(!rules.address().is_empty());
        assert!(!rules.dining().is_empty());
        assert!(!rules.interaction().is_empty());
    }

    #[test]
    fn test_vietnameseetiquetterules_validation() {
        let rules = VietnameseEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(
            rules.category(),
            RuleCategory::social("vietnamese_etiquette")
        );
    }

    #[test]
    fn test_vietnameseetiquetterules_explain() {
        let rules = VietnameseEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("问候方式"));
        assert!(e.contains("称呼讲究"));
        assert!(e.contains("餐桌礼仪"));
    }
}
