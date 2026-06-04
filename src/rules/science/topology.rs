//! 拓扑学定律

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: TopologyRules,
    name: "拓扑学定律",
    desc: "拓扑学定律",
    origin: "国际",
    tags: ["科学", "数学"]
}

impl TopologyRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["开集与闭集", "连续映射", "同胚"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["布劳威尔不动点定理", "欧拉示性数", "若尔当曲线定理"]
    }
}

impl Rule for TopologyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("topology")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "拓扑学定律",
            &[("基本概念", &self.section_0()), ("定理", &self.section_1())],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_topology_rules() {
        let r = TopologyRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
