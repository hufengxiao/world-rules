//! 意大利社交礼仪
//!
//! 意大利日常社交的问候、用餐与着装礼节

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ItalianEtiquetteRules,
    name: "意大利社交礼仪",
    desc: "意大利日常社交的问候、用餐与着装礼节",
    origin: "意大利",
    tags: ["跨文化", "欧洲", "礼仪"]
}

impl ItalianEtiquetteRules {
    /// 问候方式
    pub fn greeting(&self) -> Vec<&'static str> {
        vec![
            "熟人贴面颊问候",
            "正式场合握手",
            "问候热情真诚",
            "称呼用头衔尊称",
        ]
    }

    /// 用餐礼仪
    pub fn dining(&self) -> Vec<&'static str> {
        vec![
            "意面用叉卷起吃",
            "不将面包蘸酱汁",
            "咖啡站着快饮",
            "餐后不点奶咖",
        ]
    }

    /// 着装讲究
    pub fn dress(&self) -> Vec<&'static str> {
        vec![
            "注重衣着得体",
            "教堂需着长衣",
            "正式场合穿正装",
            "配饰搭配讲究",
        ]
    }

    /// 相处分寸
    pub fn interaction(&self) -> Vec<&'static str> {
        vec![
            "交谈富有表现力",
            "重视家庭观念",
            "热情好客",
            "话题避免宗教政治",
        ]
    }
}

impl Rule for ItalianEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("italian_etiquette")
    }

    fn explain(&self) -> String {
        format!(
            "【意大利社交礼仪】\n{}",
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
                    "着装讲究：\\n{}",
                    self.dress()
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
    fn test_italianetiquetterules_basic() {
        let rules = ItalianEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "意大利社交礼仪");
        assert!(!rules.greeting().is_empty());
        assert!(!rules.dining().is_empty());
        assert!(!rules.dress().is_empty());
        assert!(!rules.interaction().is_empty());
    }

    #[test]
    fn test_italianetiquetterules_validation() {
        let rules = ItalianEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("italian_etiquette"));
    }

    #[test]
    fn test_italianetiquetterules_explain() {
        let rules = ItalianEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("问候方式"));
        assert!(e.contains("用餐礼仪"));
        assert!(e.contains("着装讲究"));
    }
}
