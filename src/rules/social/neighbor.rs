//! 邻里礼仪

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: NeighborRules,
    name: "邻里礼仪",
    desc: "邻里相处礼仪",
    origin: "中国",
    tags: ["社交", "邻里"]
}

impl NeighborRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["晚间保持安静", "装修注意时间", "控制音乐音量"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["不占用楼道", "保持公共区域整洁", "合理使用电梯"]
    }
}

impl Rule for NeighborRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("neighbor")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "邻里礼仪",
            &[
                ("噪音控制", &self.section_0()),
                ("公共空间", &self.section_1()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_neighbor_rules() {
        let r = NeighborRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
