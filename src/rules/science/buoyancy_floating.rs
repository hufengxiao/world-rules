//! 浮力与沉浮
//!
//! 物体在水中上浮下沉所受的浮力知识

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: BuoyancyFloatingRules,
    name: "浮力与沉浮",
    desc: "物体在水中上浮下沉所受的浮力知识",
    origin: "中国",
    tags: ["科学", "浮力", "物理", "水"]
}

impl BuoyancyFloatingRules {
    /// 浮力原理
    pub fn principle(&self) -> Vec<&'static str> {
        vec![
            "物体受向上浮力",
            "等于排开液体重",
            "水给浮托力",
            "决定上浮下沉",
        ]
    }

    /// 沉浮规律
    pub fn rule(&self) -> Vec<&'static str> {
        vec![
            "密度大于水下沉",
            "密度小于水上浮",
            "空心可增大体积",
            "轮船铁船能浮",
        ]
    }

    /// 生活应用
    pub fn apply(&self) -> Vec<&'static str> {
        vec![
            "游泳仰漂浮起",
            "救生衣增大浮力",
            "潜水艇排水潜浮",
            "鱼鳔调节浮沉",
        ]
    }

    /// 简单验证
    pub fn verify(&self) -> Vec<&'static str> {
        vec!["木块浮水面", "石头沉水底", "盐水量浮蛋", "观察浮沉应变"]
    }
}

impl Rule for BuoyancyFloatingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("buoyancy")
    }

    fn explain(&self) -> String {
        format!(
            "【浮力与沉浮】\n{}",
            [
                format!(
                    "浮力原理：\\n{}",
                    self.principle()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "沉浮规律：\\n{}",
                    self.rule()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "生活应用：\\n{}",
                    self.apply()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "简单验证：\\n{}",
                    self.verify()
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
    fn test_buoyancyfloatingrules_basic() {
        let rules = BuoyancyFloatingRules::new();
        assert_eq!(rules.metadata().name, "浮力与沉浮");
        assert!(!rules.principle().is_empty());
        assert!(!rules.rule().is_empty());
        assert!(!rules.apply().is_empty());
        assert!(!rules.verify().is_empty());
    }

    #[test]
    fn test_buoyancyfloatingrules_validation() {
        let rules = BuoyancyFloatingRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::science("buoyancy"));
    }

    #[test]
    fn test_buoyancyfloatingrules_explain() {
        let rules = BuoyancyFloatingRules::new();
        let e = rules.explain();
        assert!(e.contains("浮力原理"));
        assert!(e.contains("沉浮规律"));
        assert!(e.contains("生活应用"));
    }
}
