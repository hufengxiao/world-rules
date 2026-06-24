//! 量子力学详细定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: QuantumMechanicsDetailedRules, name: "量子力学详细定律", desc: "量子力学详细定律", origin: "国际", tags: ["科学", "物理"] }
impl QuantumMechanicsDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec![
            "波粒二象性:微观粒子同时具有波动和粒子特性",
            "薛定谔方程:i*h_bar*dPsi/dt=H*Psi 描述量子态演化",
            "波函数Psi:描述粒子量子态的数学函数",
            "波函数模的平方给出粒子出现概率",
        ]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec![
            "海森堡不确定性原理:位置和动量不能同时精确测量",
            "Delta x * Delta p >= h_bar/2",
            "能量和时间的不确定性:Delta E * Delta t >= h_bar/2",
        ]
    }

    pub fn section_2(&self) -> Vec<&'static str> {
        vec![
            "量子隧穿:粒子可以穿越经典力学不允许的势垒",
            "量子纠缠:两个粒子状态关联测量一个立即影响另一个",
            "量子叠加:粒子可以同时处于多个状态",
            "观测导致波函数坍缩到确定状态",
        ]
    }
}
impl Rule for QuantumMechanicsDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("quantum_mechanics_detailed")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "量子力学详细定律",
            &[
                ("基本原理", &self.section_0()),
                ("不确定性原理", &self.section_1()),
                ("量子效应", &self.section_2()),
            ],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = QuantumMechanicsDetailedRules::new();
        assert!(!r.explain().is_empty());
    }
}
