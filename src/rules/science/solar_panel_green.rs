//! 太阳能发电
//!
//! 太阳能利用、光伏组件与绿色能源常识

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: SolarPanelGreenRules,
    name: "太阳能发电",
    desc: "太阳能利用、光伏组件与绿色能源常识",
    origin: "国际",
    tags: ["科学", "太阳能", "光伏", "能源"]
}

impl SolarPanelGreenRules {
    /// 太阳能原理
    pub fn principle(&self) -> Vec<&'static str> {
        vec!["阳光能转电能", "光伏组件发电", "光热也可利用", "可再生无限"]
    }

    /// 光伏运用
    pub fn solar_use(&self) -> Vec<&'static str> {
        vec![
            "屋顶安装光伏板",
            "分布式并网发电",
            "储能配合使用",
            "可家用供电",
        ]
    }

    /// 环保价值
    pub fn green(&self) -> Vec<&'static str> {
        vec!["清洁无排放", "减碳环保", "可持续能源", "利国利家"]
    }

    /// 效益考量
    pub fn benefit(&self) -> Vec<&'static str> {
        vec!["初期投入成本", "日照地区效益好", "维护需合理", "按情况选择"]
    }
}

impl Rule for SolarPanelGreenRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("solar")
    }

    fn explain(&self) -> String {
        format!(
            "【太阳能发电】\n{}",
            [
                format!(
                    "太阳能原理：\\n{}",
                    self.principle()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "光伏运用：\\n{}",
                    self.solar_use()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "环保价值：\\n{}",
                    self.green()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "效益考量：\\n{}",
                    self.benefit()
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
    fn test_solarpanelgreenrules_basic() {
        let rules = SolarPanelGreenRules::new();
        assert_eq!(rules.metadata().name, "太阳能发电");
        assert!(!rules.principle().is_empty());
        assert!(!rules.solar_use().is_empty());
        assert!(!rules.green().is_empty());
        assert!(!rules.benefit().is_empty());
    }

    #[test]
    fn test_solarpanelgreenrules_validation() {
        let rules = SolarPanelGreenRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::science("solar"));
    }

    #[test]
    fn test_solarpanelgreenrules_explain() {
        let rules = SolarPanelGreenRules::new();
        let e = rules.explain();
        assert!(e.contains("太阳能原理"));
        assert!(e.contains("光伏运用"));
        assert!(e.contains("环保价值"));
    }
}
