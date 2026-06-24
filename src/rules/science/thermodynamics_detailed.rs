//! 热力学详细定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ThermodynamicsDetailedRules, name: "热力学详细定律", desc: "热力学详细定律", origin: "国际", tags: ["科学", "物理"] }
impl ThermodynamicsDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec![
            "第零定律:若A与C热平衡B与C热平衡则A与B热平衡",
            "第一定律:内能变化=吸收热量-对外做功 dU=dQ-dW",
            "第二定律:热量不能自发从低温物体传到高温物体",
            "第三定律:绝对零度不可能达到",
        ]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec![
            "熵是系统无序程度的度量",
            "克劳修斯不等式:dQ/T <= 0 对于循环过程",
            "熵增原理:孤立系统的熵永不减少",
            "熵的统计解释:S=k_B * ln(W) 玻尔兹曼公式",
        ]
    }

    pub fn section_2(&self) -> Vec<&'static str> {
        vec![
            "卡诺热机:理想热机效率=1-T_cold/T_hot",
            "热力学第二定律的开尔文表述和克劳修斯表述等价",
            "自由能:Gibbs自由能G=H-TS判断反应方向",
        ]
    }
}
impl Rule for ThermodynamicsDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("thermodynamics_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "热力学详细定律",
            &[
                ("四大定律", &self.section_0()),
                ("熵", &self.section_1()),
                ("应用", &self.section_2()),
            ],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ThermodynamicsDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
