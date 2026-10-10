//! 钟乳石与溶洞形成
//!
//! 喀斯特溶洞中钟乳石与石笋的形成原理，包括碳酸钙溶解与沉积、生长速度与溶洞地貌的科学解释。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: StalactiteCaveFormationRules,
    name: "钟乳石与溶洞形成",
    desc: "喀斯特溶洞中钟乳石与石笋的形成原理，包括碳酸钙溶解与沉积、生长速度与溶洞地貌的科学解释。",
    origin: "地质学与喀斯特地貌研究",
    tags: ["科学", "地质", "溶洞", "自然现象"]
}

impl StalactiteCaveFormationRules {
    /// 化学原理
    pub fn chemistry(&self) -> Vec<&'static str> {
        vec![
            "雨水溶入二氧化碳成碳酸",
            "碳酸溶解石灰岩生成碳酸氢钙",
            "水滴逸出二氧化碳析出碳酸钙",
            "沉积形成碳酸钙结晶",
        ]
    }

    /// 形态类型
    pub fn forms(&self) -> Vec<&'static str> {
        vec![
            "自上而下生长为钟乳石",
            "自下而上生长为石笋",
            "上下相接形成石柱",
            "薄层漫流形成石幔",
        ]
    }

    /// 生长速度
    pub fn growth(&self) -> Vec<&'static str> {
        vec![
            "生长极为缓慢常百年数毫米",
            "受水量与二氧化碳浓度影响",
            "气候湿润地区发育更快",
            "形成大型景观需数万年",
        ]
    }

    /// 溶洞地貌
    pub fn landform(&self) -> Vec<&'static str> {
        vec![
            "地下水长期溶蚀形成洞穴",
            "溶洞内恒温潮湿",
            "常伴地下河与天坑",
            "属脆弱不可再生景观",
        ]
    }
}

impl Rule for StalactiteCaveFormationRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("stalactite")
    }

    fn explain(&self) -> String {
        format!(
            "【钟乳石与溶洞形成】\n{}",
            [
                format!(
                    "化学原理：\\n{}",
                    self.chemistry()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "形态类型：\\n{}",
                    self.forms()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "生长速度：\\n{}",
                    self.growth()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "溶洞地貌：\\n{}",
                    self.landform()
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
    fn test_stalactitecaveformationrules_basic() {
        let rules = StalactiteCaveFormationRules::new();
        assert_eq!(rules.metadata().name, "钟乳石与溶洞形成");
        assert!(!rules.chemistry().is_empty());
        assert!(!rules.forms().is_empty());
        assert!(!rules.growth().is_empty());
        assert!(!rules.landform().is_empty());
    }

    #[test]
    fn test_stalactitecaveformationrules_validation() {
        let rules = StalactiteCaveFormationRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::science("stalactite"));
    }

    #[test]
    fn test_stalactitecaveformationrules_explain() {
        let rules = StalactiteCaveFormationRules::new();
        let e = rules.explain();
        assert!(e.contains("化学原理"));
        assert!(e.contains("形态类型"));
        assert!(e.contains("生长速度"));
    }
}
