//! 药理学详细定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: PharmacologyDetailedRules, name: "药理学详细定律", desc: "药理学定律", origin: "国际", tags: ["科学", "医学"] }
impl PharmacologyDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec![
            "吸收:药物从给药部位进入血液循环",
            "分布:药物从血液分布到各组织",
            "代谢:药物在肝脏被代谢(主要CYP450酶)",
            "排泄:药物从体内排出(主要肾脏)",
            "半衰期:药物浓度降低一半的时间",
        ]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec![
            "受体理论:药物与受体结合产生效应",
            "激动剂:激活受体产生效应",
            "拮抗剂:阻断受体不产生效应",
            "量效关系:剂量与效应的关系",
            "治疗窗口:有效剂量与中毒剂量之间的范围",
        ]
    }

    pub fn section_2(&self) -> Vec<&'static str> {
        vec![
            "药酶诱导:加速其他药物代谢",
            "药酶抑制:减慢其他药物代谢",
            "协同作用:两药合用效应增强",
            "拮抗作用:两药合用效应减弱",
        ]
    }
}
impl Rule for PharmacologyDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("pharmacology_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "药理学详细定律",
            &[
                ("药代动力学", &self.section_0()),
                ("药效动力学", &self.section_1()),
                ("药物相互作用", &self.section_2()),
            ],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = PharmacologyDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
