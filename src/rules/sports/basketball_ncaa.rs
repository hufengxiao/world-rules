//! NCAA 大学篮球规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: BasketballNcaaRules, name: "NCAA大学篮球规则", desc: "美国大学体育协会篮球规则", origin: "美国", tags: ["体育", "篮球", "大学"] }
impl BasketballNcaaRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec![
            "每队5名球员上场",
            "上下半场各20分钟比赛时长",
            "三分线距离6.75米(男子) 6.32米(女子)",
            "30秒进攻时限",
        ]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["男子64强淘汰赛", "女子64强淘汰赛", "四强赛决出冠军"]
    }

    pub fn section_2(&self) -> Vec<&'static str> {
        vec![
            "学生运动员必须是业余身份",
            "允许提供体育奖学金",
            "不得接受任何形式的报酬",
        ]
    }

    pub fn section_3(&self) -> Vec<&'static str> {
        vec![
            "大一新生可参加比赛",
            "红衫球员可保留一年资格",
            "五年内最多四年参赛资格",
        ]
    }
}
impl Rule for BasketballNcaaRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("basketball_ncaa")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "NCAA大学篮球规则",
            &[
                ("基本规则", &self.section_0()),
                ("锦标赛", &self.section_1()),
                ("业余原则", &self.section_2()),
                ("参赛资格", &self.section_3()),
            ],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = BasketballNcaaRules::new();
        assert!(!r.explain().is_empty());
    }
}
