//! 机器学习理论

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: MlTheoryRules,
    name: "机器学习理论",
    desc: "机器学习理论定律",
    origin: "国际",
    tags: ["科学", "计算机"]
}

impl MlTheoryRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["VC维与PAC学习", "偏差方差权衡", "正则化理论"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["线性模型", "核方法", "集成学习"]
    }
}

impl Rule for MlTheoryRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("ml_theory")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "机器学习理论",
            &[("基础", &self.section_0()), ("模型", &self.section_1())],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ml_theory_rules() {
        let r = MlTheoryRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
