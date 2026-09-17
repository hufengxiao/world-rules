//! 太阳系八大行星
//!
//! 太阳系八大行星的顺序、特点与常识

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: SolarSystemPlanetsRules,
    name: "太阳系八大行星",
    desc: "太阳系八大行星的顺序、特点与常识",
    origin: "国际",
    tags: ["科学", "行星", "太阳系", "天文"]
}

impl SolarSystemPlanetsRules {
    /// 行星排序
    pub fn order(&self) -> Vec<&'static str> {
        vec![
            "水金地火木土天王海王",
            "由近到远排列",
            "地球是第三颗",
            "围绕太阳公转",
        ]
    }

    /// 内外行星
    pub fn group(&self) -> Vec<&'static str> {
        vec!["类地行星较近小", "类木远在大", "土星有光环", "木星体积大"]
    }

    /// 特色科普
    pub fn feature(&self) -> Vec<&'static str> {
        vec!["水星离日最近", "金星温度高", "火星呈红色", "木星云雾大"]
    }

    /// 观星认知
    pub fn observe(&self) -> Vec<&'static str> {
        vec!["望远镜观行星", "夜空找亮点", "光污染少处看", "增科学乐趣"]
    }
}

impl Rule for SolarSystemPlanetsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("solar_system")
    }

    fn explain(&self) -> String {
        format!(
            "【太阳系八大行星】\n{}",
            [
                format!(
                    "行星排序：\\n{}",
                    self.order()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "内外行星：\\n{}",
                    self.group()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "特色科普：\\n{}",
                    self.feature()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "观星认知：\\n{}",
                    self.observe()
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
    fn test_solarsystemplanetsrules_basic() {
        let rules = SolarSystemPlanetsRules::new();
        assert_eq!(rules.metadata().name, "太阳系八大行星");
        assert!(!rules.order().is_empty());
        assert!(!rules.group().is_empty());
        assert!(!rules.feature().is_empty());
        assert!(!rules.observe().is_empty());
    }

    #[test]
    fn test_solarsystemplanetsrules_validation() {
        let rules = SolarSystemPlanetsRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::science("solar_system"));
    }

    #[test]
    fn test_solarsystemplanetsrules_explain() {
        let rules = SolarSystemPlanetsRules::new();
        let e = rules.explain();
        assert!(e.contains("行星排序"));
        assert!(e.contains("内外行星"));
        assert!(e.contains("特色科普"));
    }
}
