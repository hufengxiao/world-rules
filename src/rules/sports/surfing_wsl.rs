//! WSL冲浪规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: SurfingWslRules, name: "WSL冲浪规则", desc: "世界冲浪联盟规则", origin: "美国", tags: ["体育", "水上"] }
impl SurfingWslRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec![
            "比赛时间:20-30分钟(根据浪况)",
            "每位选手最多冲25道浪",
            "取最好的两道浪得分相加",
            "满分10分每道浪(总分20分)",
        ]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec![
            "承诺:浪的难度和选择",
            "创新:创新性动作",
            "组合:动作的组合和流畅性",
            "速度力量和流畅性",
            "浪的大小和质量影响基础分",
        ]
    }

    pub fn section_2(&self) -> Vec<&'static str> {
        vec![
            "最内侧选手有优先权(最靠近浪的破碎点)",
            "阻挡对手冲浪会被扣分",
            "选手必须在浪的正面冲浪",
            "两人同时冲一道浪时优先权选手得分",
        ]
    }
}
impl Rule for SurfingWslRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("surfing_wsl")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "WSL冲浪规则",
            &[
                ("比赛规则", &self.section_0()),
                ("评分标准", &self.section_1()),
                ("优先权", &self.section_2()),
            ],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = SurfingWslRules::new();
        assert!(!r.explain().is_empty());
    }
}
