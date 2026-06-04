//! 黑白棋规则

use crate::rules::core::{format_rule_sections, Rule, RuleCategory, RuleMetadata, RuleResult};
use crate::simple_rule;

simple_rule! {
    struct: ReversiRules,
    name: "黑白棋规则",
    desc: "黑白棋(奥赛罗)规则",
    origin: "英国",
    tags: ["游戏", "棋类"]
}

impl ReversiRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["8x8棋盘", "开局中心4子交叉放置"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["在能夹住对方棋子的位置落子", "被夹住的棋子翻转为己方"]
    }

    pub fn section_2(&self) -> Vec<&'static str> {
        vec!["棋盘满时棋子多者胜", "双方pass则提前结束"]
    }
}

impl Rule for ReversiRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("reversi")
    }
    fn validate(&self, ctx: &str) -> RuleResult<bool> {
        Ok(!ctx.is_empty())
    }
    fn explain(&self) -> String {
        format_rule_sections(
            "黑白棋规则",
            &[
                ("棋盘", &self.section_0()),
                ("走法", &self.section_1()),
                ("胜负", &self.section_2()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_reversi_rules() {
        let r = ReversiRules::new();
        assert!(!r.metadata().name.is_empty());
        assert!(!r.explain().is_empty());
    }
}
