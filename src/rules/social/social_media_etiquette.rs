//! 社交媒体礼仪

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: SocialMediaEtiquetteRules,
    name: "社交媒体礼仪",
    desc: "社交媒体使用礼仪",
    origin: "国际",
    tags: ["社交", "网络"]
}

impl SocialMediaEtiquetteRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["不发不当内容", "尊重他人肖像权", "注明出处"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["文明评论", "不网暴", "理性讨论"]
    }
}

impl Rule for SocialMediaEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::social("social_media_etiquette")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "社交媒体礼仪",
            &[("发布", &self.section_0()), ("互动", &self.section_1())],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_social_media_etiquette_rules() {
        let r = SocialMediaEtiquetteRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
