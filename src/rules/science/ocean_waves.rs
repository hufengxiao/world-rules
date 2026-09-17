//! 海浪与海洋常识
//!
//! 海浪成因、潮汐与海滨安全的科学常识

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: OceanWavesRules,
    name: "海浪与海洋常识",
    desc: "海浪成因、潮汐与海滨安全的科学常识",
    origin: "国际",
    tags: ["科学", "海浪", "海洋", "潮汐"]
}

impl OceanWavesRules {
    /// 波浪成因
    pub fn origin(&self) -> Vec<&'static str> {
        vec![
            "风推动水面成浪",
            "海浪向岸传播",
            "浪高受风力影响",
            "波峰波谷交替",
        ]
    }

    /// 潮汐规律
    pub fn tide(&self) -> Vec<&'static str> {
        vec![
            "潮汐受天体引力",
            "涨落有周期",
            "大潮小潮有别",
            "沿海生活循潮",
        ]
    }

    /// 海洋环境
    pub fn ocean(&self) -> Vec<&'static str> {
        vec![
            "海洋广阔多样",
            "洋流环流影响气候",
            "海洋生物丰富",
            "保护海洋生态",
        ]
    }

    /// 安全滨海
    pub fn safety(&self) -> Vec<&'static str> {
        vec![
            "游泳注意离岸流",
            "不低估海浪力量",
            "留意潮讯与警示",
            "安全滨海游乐",
        ]
    }
}

impl Rule for OceanWavesRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("ocean_waves")
    }

    fn explain(&self) -> String {
        format!(
            "【海浪与海洋常识】\n{}",
            [
                format!(
                    "波浪成因：\\n{}",
                    self.origin()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "潮汐规律：\\n{}",
                    self.tide()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "海洋环境：\\n{}",
                    self.ocean()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "安全滨海：\\n{}",
                    self.safety()
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
    fn test_oceanwavesrules_basic() {
        let rules = OceanWavesRules::new();
        assert_eq!(rules.metadata().name, "海浪与海洋常识");
        assert!(!rules.origin().is_empty());
        assert!(!rules.tide().is_empty());
        assert!(!rules.ocean().is_empty());
        assert!(!rules.safety().is_empty());
    }

    #[test]
    fn test_oceanwavesrules_validation() {
        let rules = OceanWavesRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::science("ocean_waves"));
    }

    #[test]
    fn test_oceanwavesrules_explain() {
        let rules = OceanWavesRules::new();
        let e = rules.explain();
        assert!(e.contains("波浪成因"));
        assert!(e.contains("潮汐规律"));
        assert!(e.contains("海洋环境"));
    }
}
