//! 心脏健康规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: HeartHealthRules, name: "心脏健康规则", desc: "心脏健康护理规则", origin: "国际", tags: ["健康", "心脏"] }
impl HeartHealthRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec![
            "高血压:收缩压>=140或舒张压>=90mmHg",
            "高血脂:LDL-C>3.4mmol/L",
            "糖尿病:空腹血糖>=7.0mmol/L",
            "吸烟:心血管疾病的重要危险因素",
            "肥胖:BMI>=28",
            "家族史:直系亲属有心血管疾病",
        ]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec![
            "控制血压:目标<140/90mmHg",
            "控制血脂:LDL-C目标<2.6mmol/L(高危<1.8)",
            "控制血糖:HbA1c<7%",
            "戒烟:戒烟1年后心血管风险降低50%",
            "运动:每周150分钟中等强度有氧运动",
            "饮食:低盐(<6g/天)/低脂/多蔬果",
        ]
    }

    pub fn section_2(&self) -> Vec<&'static str> {
        vec![
            "胸痛:胸骨后压榨性疼痛持续>15分钟",
            "呼吸困难:活动后气短",
            "心悸:心跳不规则或过快",
            "晕厥:突然意识丧失",
            "发现症状立即拨打120急救电话",
        ]
    }
}
impl Rule for HeartHealthRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::health("heart_health")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "心脏健康规则",
            &[
                ("风险因素", &self.section_0()),
                ("预防措施", &self.section_1()),
                ("警示症状", &self.section_2()),
            ],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = HeartHealthRules::new();
        assert!(!r.explain().is_empty());
    }
}
