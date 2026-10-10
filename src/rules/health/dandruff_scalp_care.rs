//! 头皮屑护理
//!
//! 头皮屑与头皮瘙痒的清洁与日常养护

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: DandruffScalpCareRules,
    name: "头皮屑护理",
    desc: "头皮屑与头皮瘙痒的清洁与日常养护",
    origin: "中国",
    tags: ["皮肤", "头皮", "护理"]
}

impl DandruffScalpCareRules {
    /// 清洁方法
    pub fn cleaning(&self) -> Vec<&'static str> {
        vec![
            "温水洗发勿用过热",
            "洗发水充分起泡再上头",
            "指腹按摩头皮勿用指甲抓",
            "冲洗彻底不留残留",
        ]
    }

    /// 洗发频率
    pub fn frequency(&self) -> Vec<&'static str> {
        vec![
            "油性头皮可隔日一洗",
            "干性头皮不宜过勤",
            "出汗后及时清洁",
            "依头皮状况调整",
        ]
    }

    /// 去屑护理
    pub fn treatment(&self) -> Vec<&'static str> {
        vec![
            "可选含去屑成分洗发水",
            "去屑产品需停留数分钟",
            "交替使用防耐受",
            "坚持一段时间见效",
        ]
    }

    /// 生活调整
    pub fn lifestyle(&self) -> Vec<&'static str> {
        vec![
            "规律作息少熬夜",
            "清淡饮食少油腻辛辣",
            "减少烫染刺激头皮",
            "枕巾毛巾勤换洗",
        ]
    }
}

impl Rule for DandruffScalpCareRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("dandruff_scalp_care")
    }

    fn explain(&self) -> String {
        format!(
            "【头皮屑护理】\n{}",
            [
                format!(
                    "清洁方法：\\n{}",
                    self.cleaning()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "洗发频率：\\n{}",
                    self.frequency()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "去屑护理：\\n{}",
                    self.treatment()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "生活调整：\\n{}",
                    self.lifestyle()
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
    fn test_dandruffscalpcarerules_basic() {
        let rules = DandruffScalpCareRules::new();
        assert_eq!(rules.metadata().name, "头皮屑护理");
        assert!(!rules.cleaning().is_empty());
        assert!(!rules.frequency().is_empty());
        assert!(!rules.treatment().is_empty());
        assert!(!rules.lifestyle().is_empty());
    }

    #[test]
    fn test_dandruffscalpcarerules_validation() {
        let rules = DandruffScalpCareRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(
            rules.category(),
            RuleCategory::health("dandruff_scalp_care")
        );
    }

    #[test]
    fn test_dandruffscalpcarerules_explain() {
        let rules = DandruffScalpCareRules::new();
        let e = rules.explain();
        assert!(e.contains("清洁方法"));
        assert!(e.contains("洗发频率"));
        assert!(e.contains("去屑护理"));
    }
}
