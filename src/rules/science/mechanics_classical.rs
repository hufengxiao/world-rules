//! 经典力学定律
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: MechanicsClassicalRules, name: "经典力学定律", desc: "经典力学定律", origin: "国际", tags: ["科学", "物理"] }
impl MechanicsClassicalRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec![
            "第一定律惯性定律:物体不受力时保持静止或匀速直线运动",
            "第二定律:F=ma 力等于质量乘以加速度",
            "第三定律:作用力与反作用力大小相等方向相反",
        ]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec![
            "F=GMm/r^2 两物体间引力与质量乘积成正比与距离平方成反比",
            "G=6.674x10^-11 N·m^2/kg^2 万有引力常数",
            "适用于质点或均匀球体",
        ]
    }

    pub fn section_2(&self) -> Vec<&'static str> {
        vec![
            "功:W=Fs cos theta 力乘以位移乘以夹角余弦",
            "动能:Ek=1/2 mv^2",
            "势能:Ep=mgh 重力势能",
            "能量守恒定律:能量不能被创造或消灭只能转化",
        ]
    }
}
impl Rule for MechanicsClassicalRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::science("mechanics_classical")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "经典力学定律",
            &[
                ("牛顿三大定律", &self.section_0()),
                ("万有引力定律", &self.section_1()),
                ("功和能", &self.section_2()),
            ],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = MechanicsClassicalRules::new();
        assert!(!r.explain().is_empty());
    }
}
