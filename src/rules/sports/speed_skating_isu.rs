//! 速滑ISU规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: SpeedSkatingIsuRules, name: "速滑ISU规则", desc: "ISU速度滑冰规则", origin: "国际", tags: ["体育", "冬季"] }
impl SpeedSkatingIsuRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec![
            "标准跑道:400米椭圆形冰道",
            "两人一组比赛分别在内外道",
            "每圈交换内外道确保公平",
            "以时间排名不是以对手为参照",
        ]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec![
            "500米:爆发力项目",
            "1000米:速度耐力项目",
            "1500米:中距离项目",
            "5000米/10000米:长距离项目",
            "团体追逐赛:3人一队",
            "集体出发:多人同时比赛",
        ]
    }

    pub fn section_2(&self) -> Vec<&'static str> {
        vec![
            "克莱普冰鞋:冰刀可活动提高蹬冰效率",
            "连体服:减少空气阻力",
            "头盔:安全保护",
            "护目镜:防风防冰屑",
        ]
    }
}
impl Rule for SpeedSkatingIsuRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("speed_skating_isu")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "速滑ISU规则",
            &[
                ("比赛规则", &self.section_0()),
                ("项目", &self.section_1()),
                ("装备", &self.section_2()),
            ],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = SpeedSkatingIsuRules::new();
        assert!(!r.explain().is_empty());
    }
}
