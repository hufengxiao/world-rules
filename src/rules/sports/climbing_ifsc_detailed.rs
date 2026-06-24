//! 攀岩IFSC详细
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ClimbingIfscDetailedRules, name: "攀岩IFSC详细", desc: "攀岩国际联合会规则", origin: "国际", tags: ["体育", "极限"] }
impl ClimbingIfscDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec![
            "标准赛道:15米高45度倾斜",
            "两人同时攀登相同赛道",
            "最快到达顶部者胜",
            "世界纪录:男子约5秒女子约6秒",
        ]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec![
            "先锋赛:选手在规定时间内攀登尽可能高的位置",
            "先锋赛时间:男子6分钟女子6分钟",
            "选手不能事先查看赛道",
            "高度越高排名越前",
        ]
    }

    pub fn section_2(&self) -> Vec<&'static str> {
        vec![
            "选手在规定时间内尝试多条路线",
            "每条路线有多个得分点(Zone和Top)",
            "Top到达路线顶部得分最高",
            "Zone到达中间得分点得分次之",
            "以完成路线数和尝试次数排名",
        ]
    }
}
impl Rule for ClimbingIfscDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("climbing_ifsc_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "攀岩IFSC详细",
            &[
                ("速度赛", &self.section_0()),
                ("难度赛", &self.section_1()),
                ("攀石赛", &self.section_2()),
            ],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ClimbingIfscDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
