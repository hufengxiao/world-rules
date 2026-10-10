//! 极光的科学
//!
//! 极光现象的成因与特征，包括太阳风、地磁场引导、高层大气激发发光与极光颜色差异的科学解释。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: AuroraBorealisScienceRules,
    name: "极光的科学",
    desc: "极光现象的成因与特征，包括太阳风、地磁场引导、高层大气激发发光与极光颜色差异的科学解释。",
    origin: "空间物理与地磁学理论",
    tags: ["科学", "天文", "地磁", "自然现象"]
}

impl AuroraBorealisScienceRules {
    /// 成因机制
    pub fn cause(&self) -> Vec<&'static str> {
        vec![
            "太阳风携带带电粒子",
            "地磁场引导粒子向极区汇聚",
            "粒子撞击高层大气原子分子",
            "激发态回落发出光子",
        ]
    }

    /// 颜色差异
    pub fn colors(&self) -> Vec<&'static str> {
        vec![
            "氧原子激发发绿色最常见",
            "高层氧发红色极光",
            "氮分子产生蓝紫色",
            "颜色取决于高度与粒子种类",
        ]
    }

    /// 发生规律
    pub fn pattern(&self) -> Vec<&'static str> {
        vec![
            "多见于南北磁纬高纬地区",
            "太阳活动峰年更频繁",
            "夜间与特定季节更易观测",
            "常呈带状弧状或幕状",
        ]
    }

    /// 观测影响
    pub fn impact(&self) -> Vec<&'static str> {
        vec![
            "强磁暴可致电网与通信受扰",
            "影响卫星与导航信号",
            "高纬航空需关注辐射",
            "是极区旅游重要景观",
        ]
    }
}

impl Rule for AuroraBorealisScienceRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("aurora")
    }

    fn explain(&self) -> String {
        format!(
            "【极光的科学】\n{}",
            [
                format!(
                    "成因机制：\\n{}",
                    self.cause()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "颜色差异：\\n{}",
                    self.colors()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "发生规律：\\n{}",
                    self.pattern()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "观测影响：\\n{}",
                    self.impact()
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
    fn test_auroraborealissciencerules_basic() {
        let rules = AuroraBorealisScienceRules::new();
        assert_eq!(rules.metadata().name, "极光的科学");
        assert!(!rules.cause().is_empty());
        assert!(!rules.colors().is_empty());
        assert!(!rules.pattern().is_empty());
        assert!(!rules.impact().is_empty());
    }

    #[test]
    fn test_auroraborealissciencerules_validation() {
        let rules = AuroraBorealisScienceRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::science("aurora"));
    }

    #[test]
    fn test_auroraborealissciencerules_explain() {
        let rules = AuroraBorealisScienceRules::new();
        let e = rules.explain();
        assert!(e.contains("成因机制"));
        assert!(e.contains("颜色差异"));
        assert!(e.contains("发生规律"));
    }
}
