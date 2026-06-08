//! 费舍尔随机棋规则
use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;
simple_rule! { struct: Chess960Rules, name: "费舍尔随机棋规则", desc: "Chess960规则", origin: "国际", tags: ["游戏", "棋类"] }
impl Chess960Rules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["随机排列首排棋子", "王车易位规则不变"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["消除开局理论依赖", "增加创造力"]
    }
}
impl Rule for Chess960Rules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("chess960")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "费舍尔随机棋规则",
            &[("开局", &self.section_0()), ("目的", &self.section_1())],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = Chess960Rules::new();
        assert!(!r.explain().is_empty());
    }
}
