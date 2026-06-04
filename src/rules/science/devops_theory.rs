//! DevOps理论

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: DevopsTheoryRules,
    name: "DevOps理论",
    desc: "DevOps工程理论定律",
    origin: "国际",
    tags: ["科学", "计算机"]
}

impl DevopsTheoryRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["持续集成持续部署", "基础设施即代码", "监控与可观测性"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["容器化与编排", "微服务架构", "自动化测试"]
    }
}

impl Rule for DevopsTheoryRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("devops_theory")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "DevOps理论",
            &[("原则", &self.section_0()), ("实践", &self.section_1())],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_devops_theory_rules() {
        let r = DevopsTheoryRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
