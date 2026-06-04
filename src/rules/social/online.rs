//! 网络礼仪

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: OnlineRules,
    name: "网络礼仪",
    desc: "网络社交礼仪",
    origin: "国际",
    tags: ["社交", "网络"]
}

impl OnlineRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["使用礼貌用语", "不发垃圾信息", "尊重隐私"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["不刷屏", "不传播谣言", "尊重原创"]
    }
}

impl Rule for OnlineRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("online")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "网络礼仪",
            &[("沟通", &self.section_0()), ("社交媒体", &self.section_1())],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_online_rules() {
        let r = OnlineRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
