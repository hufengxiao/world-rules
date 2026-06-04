//! 天体物理定律

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: AstrophysicsRules,
    name: "天体物理定律",
    desc: "天体物理定律",
    origin: "国际",
    tags: ["科学", "天文"]
}

impl AstrophysicsRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["赫罗图", "恒星演化", "白矮星中子星黑洞"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["哈勃定律", "宇宙微波背景", "暗物质暗能量"]
    }
}

impl Rule for AstrophysicsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("astrophysics")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "天体物理定律",
            &[("恒星", &self.section_0()), ("宇宙", &self.section_1())],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_astrophysics_rules() {
        let r = AstrophysicsRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
