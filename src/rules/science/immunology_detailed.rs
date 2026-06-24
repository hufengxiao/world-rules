//! 免疫学详细定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ImmunologyDetailedRules, name: "免疫学详细定律", desc: "免疫学定律", origin: "国际", tags: ["科学", "生物"] }
impl ImmunologyDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec![
            "物理屏障:皮肤/黏膜/纤毛",
            "化学屏障:胃酸/溶菌酶/抗菌肽",
            "细胞:巨噬细胞/中性粒细胞/NK细胞",
            "炎症反应:红肿热痛",
        ]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec![
            "T细胞:细胞免疫(杀伤性T细胞/辅助性T细胞)",
            "B细胞:体液免疫(产生抗体)",
            "抗体:IgG/IgM/IgA/IgE/IgD五类",
            "免疫记忆:疫苗原理",
        ]
    }

    pub fn section_2(&self) -> Vec<&'static str> {
        vec![
            "过敏:免疫系统对无害物质过度反应",
            "自身免疫病:免疫系统攻击自身组织",
            "免疫缺陷:免疫系统功能不足",
            "免疫疗法:利用免疫系统治疗疾病",
        ]
    }
}
impl Rule for ImmunologyDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("immunology_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "免疫学详细定律",
            &[
                ("先天免疫", &self.section_0()),
                ("适应性免疫", &self.section_1()),
                ("免疫相关疾病", &self.section_2()),
            ],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ImmunologyDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
