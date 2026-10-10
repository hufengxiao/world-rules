//! 脚臭护理
//!
//! 脚部异味与多汗的清洁与护理方法

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: FootOdorCareRules,
    name: "脚臭护理",
    desc: "脚部异味与多汗的清洁与护理方法",
    origin: "中国",
    tags: ["足部", "护理", "生活"]
}

impl FootOdorCareRules {
    /// 清洁护理
    pub fn cleaning(&self) -> Vec<&'static str> {
        vec![
            "每日洗脚并擦干",
            "重点清洁趾缝",
            "定期修剪趾甲",
            "洗后彻底擦干脚部",
        ]
    }

    /// 鞋袜管理
    pub fn footwear(&self) -> Vec<&'static str> {
        vec![
            "选吸汗透气棉袜",
            "每日更换袜子",
            "鞋子轮换穿晾晒",
            "鞋内可放除臭剂",
        ]
    }

    /// 抑汗抑菌
    pub fn control(&self) -> Vec<&'static str> {
        vec![
            "用止汗剂减少出汗",
            "保持足部干燥",
            "可用抑菌喷雾",
            "避免长时间穿同一鞋",
        ]
    }

    /// 就医提示
    pub fn seeking_help(&self) -> Vec<&'static str> {
        vec![
            "伴瘙痒脱皮须查足癣",
            "异味顽固应就诊",
            "多汗严重可就医",
            "糖尿病患者足部尤须重视",
        ]
    }
}

impl Rule for FootOdorCareRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("foot_odor_care")
    }

    fn explain(&self) -> String {
        format!(
            "【脚臭护理】\n{}",
            [
                format!(
                    "清洁护理：\\n{}",
                    self.cleaning()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "鞋袜管理：\\n{}",
                    self.footwear()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "抑汗抑菌：\\n{}",
                    self.control()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "就医提示：\\n{}",
                    self.seeking_help()
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
    fn test_footodorcarerules_basic() {
        let rules = FootOdorCareRules::new();
        assert_eq!(rules.metadata().name, "脚臭护理");
        assert!(!rules.cleaning().is_empty());
        assert!(!rules.footwear().is_empty());
        assert!(!rules.control().is_empty());
        assert!(!rules.seeking_help().is_empty());
    }

    #[test]
    fn test_footodorcarerules_validation() {
        let rules = FootOdorCareRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("foot_odor_care"));
    }

    #[test]
    fn test_footodorcarerules_explain() {
        let rules = FootOdorCareRules::new();
        let e = rules.explain();
        assert!(e.contains("清洁护理"));
        assert!(e.contains("鞋袜管理"));
        assert!(e.contains("抑汗抑菌"));
    }
}
