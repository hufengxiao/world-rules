//! 英超规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: FootballLeagueRules, name: "英超规则", desc: "英格兰足球超级联赛规则", origin: "英国", tags: ["体育", "球类"] }
impl FootballLeagueRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec![
            "20支球队主客场双循环共38轮",
            "胜3分平1分负0分",
            "积分相同依次比较净胜球进球数相互战绩",
            "欧冠资格前4名直接进入小组赛",
            "降级最后3名降入英冠",
        ]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec![
            "视频助理裁判审查进球点球红牌认错人",
            "主裁判可查看场边监视器",
            "仅明显错误时介入",
        ]
    }

    pub fn section_2(&self) -> Vec<&'static str> {
        vec![
            "每场可换5人3次换人窗口",
            "加时赛淘汰赛平局后30分钟",
            "点球大战加时赛后仍平局时进行",
        ]
    }
}
impl Rule for FootballLeagueRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("football_league")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "英超规则",
            &[
                ("联赛结构", &self.section_0()),
                ("VAR规则", &self.section_1()),
                ("比赛规则", &self.section_2()),
            ],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = FootballLeagueRules::new();
        assert!(!r.explain().is_empty());
    }
}
