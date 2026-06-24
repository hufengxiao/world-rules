//! 分子生物学定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: MolecularBiologyRules, name: "分子生物学定律", desc: "分子生物学定律", origin: "国际", tags: ["科学", "生物"] }
impl MolecularBiologyRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec![
            "DNA复制:DNA->DNA 以DNA为模板合成新DNA",
            "转录:DNA->RNA 以DNA为模板合成mRNA",
            "翻译:RNA->蛋白质 mRNA在核糖体上翻译为蛋白质",
            "逆转录:RNA->DNA 逆转录酶催化(病毒)",
        ]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec![
            "转录水平:启动子/增强子/转录因子",
            "转录后水平:mRNA剪接/修饰/稳定性",
            "翻译水平:核糖体结合/miRNA调控",
            "表观遗传:DNA甲基化/组蛋白修饰",
        ]
    }

    pub fn section_2(&self) -> Vec<&'static str> {
        vec![
            "限制性内切酶:识别特定DNA序列并切割",
            "DNA连接酶:连接DNA片段",
            "PCR:聚合酶链式反应扩增DNA",
            "CRISPR-Cas9:基因编辑技术",
        ]
    }
}
impl Rule for MolecularBiologyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("molecular_biology")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "分子生物学定律",
            &[
                ("中心法则", &self.section_0()),
                ("基因表达调控", &self.section_1()),
                ("基因工程", &self.section_2()),
            ],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = MolecularBiologyRules::new();
        assert!(!r.explain().is_empty());
    }
}
