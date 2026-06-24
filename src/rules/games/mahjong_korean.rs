//! 韩国麻将规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: MahjongKoreanRules, name: "韩国麻将规则", desc: "韩国麻将规则", origin: "韩国", tags: ["游戏", "麻将"] }
impl MahjongKoreanRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec![
            "韩国麻将使用简化规则",
            "没有风牌和箭牌只有万条筒",
            "每人发13张牌",
            "不使用花牌",
        ]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec![
            "只有条和筒两种花色时可胡",
            "韩国麻将简化了番种计算",
            "注重速度和简洁",
        ]
    }
}
impl Rule for MahjongKoreanRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("mahjong_korean")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "韩国麻将规则",
            &[
                ("基本规则", &self.section_0()),
                ("特殊规则", &self.section_1()),
            ],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = MahjongKoreanRules::new();
        assert!(!r.explain().is_empty());
    }
}
