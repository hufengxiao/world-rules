//! 音乐理论定律
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: MusicTheoryRules, name: "音乐理论定律", desc: "音乐理论定律", origin: "国际", tags: ["科学", "艺术"] }
impl MusicTheoryRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["和弦进行", "调性体系"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["节拍体系", "节奏型"]
    }
}
impl Rule for MusicTheoryRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("music_theory")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "音乐理论定律",
            &[("和声", &self.section_0()), ("节奏", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = MusicTheoryRules::new();
        assert!(!r.explain().is_empty());
    }
}
