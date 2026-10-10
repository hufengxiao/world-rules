//! 足癣（脚气）护理
//!
//! 足癣的日常护理与防复发规则，包括保持足部干燥、鞋袜处理、规范用药与防止交叉感染，避免反复发作。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: TineaPedisCareRules,
    name: "足癣（脚气）护理",
    desc: "足癣的日常护理与防复发规则，包括保持足部干燥、鞋袜处理、规范用药与防止交叉感染，避免反复发作。",
    origin: "皮肤科真菌感染防治建议",
    tags: ["健康", "皮肤", "真菌", "护理"]
}

impl TineaPedisCareRules {
    /// 保持干燥
    pub fn dryness(&self) -> Vec<&'static str> {
        vec![
            "洗脚后彻底擦干趾缝",
            "选择透气鞋袜",
            "出汗多时及时更换袜子",
            "避免长时间穿闷热鞋",
        ]
    }

    /// 鞋袜处理
    pub fn footwear(&self) -> Vec<&'static str> {
        vec![
            "鞋袜勤换勤洗",
            "阳光下晾晒杀菌",
            "不与他人共用拖鞋毛巾",
            "可定期消毒鞋内",
        ]
    }

    /// 规范用药
    pub fn medication(&self) -> Vec<&'static str> {
        vec![
            "遵医嘱坚持外用抗真菌药",
            "症状消失后继续巩固疗程",
            "不随意停药防复发",
            "不滥用激素类药膏",
        ]
    }

    /// 防交叉感染
    pub fn prevention(&self) -> Vec<&'static str> {
        vec![
            "公共场所不光脚行走",
            "指甲剪短避免抓挠传播",
            "手部接触患处后洗手",
            "家人同患需一同治疗",
        ]
    }
}

impl Rule for TineaPedisCareRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("tinea_pedis")
    }

    fn explain(&self) -> String {
        format!(
            "【足癣（脚气）护理】\n{}",
            [
                format!(
                    "保持干燥：\\n{}",
                    self.dryness()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "鞋袜处理：\\n{}",
                    self.footwear()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "规范用药：\\n{}",
                    self.medication()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "防交叉感染：\\n{}",
                    self.prevention()
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
    fn test_tineapediscarerules_basic() {
        let rules = TineaPedisCareRules::new();
        assert_eq!(rules.metadata().name, "足癣（脚气）护理");
        assert!(!rules.dryness().is_empty());
        assert!(!rules.footwear().is_empty());
        assert!(!rules.medication().is_empty());
        assert!(!rules.prevention().is_empty());
    }

    #[test]
    fn test_tineapediscarerules_validation() {
        let rules = TineaPedisCareRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("tinea_pedis"));
    }

    #[test]
    fn test_tineapediscarerules_explain() {
        let rules = TineaPedisCareRules::new();
        let e = rules.explain();
        assert!(e.contains("保持干燥"));
        assert!(e.contains("鞋袜处理"));
        assert!(e.contains("规范用药"));
    }
}
