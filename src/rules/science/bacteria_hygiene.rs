//! 微生物卫生常识
//!
//! 细菌传播途径、清洁消毒与食品安全科学常识

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: BacteriaHygieneRules,
    name: "微生物卫生常识",
    desc: "细菌传播途径、清洁消毒与食品安全科学常识",
    origin: "国际",
    tags: ["科学", "细菌", "卫生", "微生物"]
}

impl BacteriaHygieneRules {
    /// 传播途径
    pub fn transmission(&self) -> Vec<&'static str> {
        vec![
            "细菌经接触与飞沫传播",
            "双手常成传播媒介",
            "食物污染引发疾病",
            "共用餐具增加风险",
        ]
    }

    /// 清洁消毒
    pub fn cleaning(&self) -> Vec<&'static str> {
        vec![
            "勤洗手是关键措施",
            "正确清洁食物表面",
            "餐具餐具高温消毒",
            "定期清洁环境",
        ]
    }

    /// 食品卫生
    pub fn food(&self) -> Vec<&'static str> {
        vec![
            "生熟分离防交叉",
            "食物彻底加工熟透",
            "冷藏保存及时",
            "过期食品不食用",
        ]
    }

    /// 科学认知
    pub fn science(&self) -> Vec<&'static str> {
        vec![
            "不必谈菌色变",
            "多数细菌无害或有利",
            "抗生素不治病毒感染",
            "遵医嘱规范使用抗菌药",
        ]
    }
}

impl Rule for BacteriaHygieneRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("bacteria_hygiene")
    }

    fn explain(&self) -> String {
        format!(
            "【微生物卫生常识】\n{}",
            [
                format!(
                    "传播途径：\\n{}",
                    self.transmission()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "清洁消毒：\\n{}",
                    self.cleaning()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "食品卫生：\\n{}",
                    self.food()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "科学认知：\\n{}",
                    self.science()
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
    fn test_bacteriahygienerules_basic() {
        let rules = BacteriaHygieneRules::new();
        assert_eq!(rules.metadata().name, "微生物卫生常识");
        assert!(!rules.transmission().is_empty());
        assert!(!rules.cleaning().is_empty());
        assert!(!rules.food().is_empty());
        assert!(!rules.science().is_empty());
    }

    #[test]
    fn test_bacteriahygienerules_validation() {
        let rules = BacteriaHygieneRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::science("bacteria_hygiene"));
    }

    #[test]
    fn test_bacteriahygienerules_explain() {
        let rules = BacteriaHygieneRules::new();
        let e = rules.explain();
        assert!(e.contains("传播途径"));
        assert!(e.contains("清洁消毒"));
        assert!(e.contains("食品卫生"));
    }
}
