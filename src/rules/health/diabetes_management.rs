//! 糖尿病管理规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: DiabetesManagementRules, name: "糖尿病管理规则", desc: "糖尿病管理规则", origin: "国际", tags: ["健康", "慢性病"] }
impl DiabetesManagementRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec![
            "空腹血糖>=7.0mmol/L",
            "餐后2小时血糖>=11.1mmol/L",
            "糖化血红蛋白HbA1c>=6.5%",
            "随机血糖>=11.1mmol/L伴典型症状",
        ]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec![
            "控制总热量:根据体重和活动量计算",
            "碳水化合物占总热量45-60%",
            "选择低GI食物:全谷物/豆类/蔬菜",
            "定时定量:每天3餐规律进食",
            "限制含糖饮料和精制糖",
        ]
    }

    pub fn section_2(&self) -> Vec<&'static str> {
        vec![
            "每周至少150分钟中等强度有氧运动",
            "运动时间:餐后1小时最佳",
            "运动前后监测血糖",
            "避免空腹运动防止低血糖",
            "血糖>16.7mmol/L或有酮症时不宜运动",
        ]
    }
}
impl Rule for DiabetesManagementRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::health("diabetes_management")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "糖尿病管理规则",
            &[
                ("诊断标准", &self.section_0()),
                ("饮食管理", &self.section_1()),
                ("运动管理", &self.section_2()),
            ],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = DiabetesManagementRules::new();
        assert!(!r.explain().is_empty());
    }
}
