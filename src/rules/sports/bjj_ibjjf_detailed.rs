//! IBJJF详细规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: BjjIbjjfDetailedRules, name: "IBJJF详细规则", desc: "IBJJF巴西柔术详细", origin: "巴西", tags: ["体育", "格斗"] }
impl BjjIbjjfDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec![
            "白带:初学者",
            "蓝带:2年训练",
            "紫带:4年训练",
            "棕带:6年训练",
            "黑带:8年以上",
        ]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec![
            "比赛时间根据带位:白蓝5分钟紫棕6分钟黑10分钟",
            "得分:扫技2分摔倒3分过腿3分骑乘4分拿背4分",
            "优势:近似得分动作",
        ]
    }

    pub fn section_2(&self) -> Vec<&'static str> {
        vec![
            "绞技:裸绞三角绞领绞",
            "关节技:十字固肩锁膝十字固",
            "拍垫认输:被降服时拍对手或垫子",
        ]
    }
}
impl Rule for BjjIbjjfDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::sports("bjj_ibjjf_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "IBJJF详细规则",
            &[
                ("带位制度", &self.section_0()),
                ("比赛规则", &self.section_1()),
                ("降服", &self.section_2()),
            ],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = BjjIbjjfDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
