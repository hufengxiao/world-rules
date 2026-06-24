//! 围棋详细规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: GoDetailedRules, name: "围棋详细规则", desc: "围棋详细规则", origin: "中国", tags: ["游戏", "棋类"] }
impl GoDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec![
            "19x19棋盘也有9x9和13x13",
            "黑白双方轮流在交叉点落子",
            "棋子落下后不能移动除非被提",
            "气:棋子相邻的空交叉点",
            "无气的棋子被提走",
            "禁止自杀不能下无气的点除非能提对方",
        ]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec![
            "眼:被己方棋子包围的空交叉点",
            "两个真眼的棋群是活棋不会被提",
            "假眼:可被对方破坏的眼",
        ]
    }

    pub fn section_2(&self) -> Vec<&'static str> {
        vec![
            "中国规则:数子法活子+围空",
            "日本规则:数目法围空-提子",
            "贴目:黑方先行补偿白方中国7.5目日本6.5目",
            "终局:双方pass后计算领地",
            "劫争:禁止立即回提同一子",
        ]
    }
}
impl Rule for GoDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("go_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "围棋详细规则",
            &[
                ("基本规则", &self.section_0()),
                ("眼与活棋", &self.section_1()),
                ("规则体系", &self.section_2()),
            ],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = GoDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
