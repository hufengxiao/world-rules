//! 流体动力学规则
//!
//! 流体动力学研究流体（液体和气体）的运动规律。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: FluidDynamicsRules,
    name: "流体动力学规则",
    desc: "流体动力学基本定律与分析方法",
    origin: "力学",
    tags: ["科学", "物理", "力学", "流体"]
}

impl FluidDynamicsRules {
    /// 流体性质
    pub fn fluid_properties(&self) -> Vec<&'static str> {
        vec![
            "密度: ρ = m/V，单位体积的质量",
            "粘性: 流体抵抗剪切变形的能力",
            "粘度 μ: 动力粘度，Pa·s",
            "运动粘度 ν: ν = μ/ρ，m²/s",
            "表面张力: 液体表面收缩的趋势",
            "压缩性: 流体体积随压力变化的性质",
            "不可压缩流体: 密度近似不变（液体）",
            "可压缩流体: 密度随压力变化（气体）",
        ]
    }

    /// 流体静力学
    pub fn fluid_statics(&self) -> Vec<&'static str> {
        vec![
            "静压强: p = ρgh",
            "帕斯卡原理: 外加压强传递到各点",
            "连通器原理: 同一水平面压强相等",
            "浮力: F = ρgV排（阿基米德原理）",
            "浮力条件: 物体密度小于流体密度则上浮",
            "压力中心: 合压力作用点",
            "液面压强: p₀ + ρgh",
            "大气压强: p₀ ≈ 101.3 kPa",
        ]
    }

    /// 流体运动学
    pub fn fluid_kinematics(&self) -> Vec<&'static str> {
        vec![
            "流线: 流体质点运动轨迹的切线",
            "流管: 一组流线围成的管道",
            "恒定流: 流场参数不随时间变化",
            "非恒定流: 流场参数随时间变化",
            "均匀流: 流场参数不随位置变化",
            "非均匀流: 流场参数随位置变化",
            "层流: 流体分层流动，互不掺混",
            "湍流: 流体紊乱流动，有漩涡",
        ]
    }

    /// 连续性方程
    pub fn continuity_equation(&self) -> Vec<&'static str> {
        vec![
            "质量守恒: 流入质量等于流出质量",
            "连续性方程: A₁v₁ = A₂v₂",
            "体积流量: Q = Av",
            "质量流量: m = ρAv",
            "可压缩流体连续性: ρ₁A₁v₁ = ρ₂A₂v₂",
            "三维连续性方程: ∂ρ/∂t + ∇·(ρv) = 0",
            "不可压缩流体: ∇·v = 0",
            "管道流动: 流速与截面积成反比",
        ]
    }

    /// 伯努利方程
    pub fn bernoulli_equation(&self) -> Vec<&'static str> {
        vec![
            "伯努利方程: p + ½ρv² + ρgh = const",
            "能量守恒: 机械能沿流线守恒",
            "静压能: p（压力势能）",
            "动能: ½ρv²（动能密度）",
            "势能: ρgh（重力势能密度）",
            "应用: 流量测量、流速计算",
            "文丘里管: 通过压差测量流速",
            "皮托管: 测量流体速度",
        ]
    }

    /// 雷诺数
    pub fn reynolds_number(&self) -> Vec<&'static str> {
        vec![
            "雷诺数: Re = ρvd/μ = vd/ν",
            "临界雷诺数: Rec ≈ 2300（圆管）",
            "层流: Re < Rec",
            "湍流: Re > Rec",
            "过渡流: Rec附近",
            "雷诺数意义: 惯性力与粘性力之比",
            "无量纲参数: 用于流态判别",
            "雷诺数应用: 管流、绕流分析",
        ]
    }

    /// 流体阻力
    pub fn fluid_resistance(&self) -> Vec<&'static str> {
        vec![
            "阻力公式: F = CdρAv²/2",
            "阻力系数 Cd: 与形状和流态有关",
            "摩擦阻力: 流体粘性引起的阻力",
            "压差阻力: 前后压力差引起的阻力",
            "形状阻力: 与物体形状有关",
            "边界层: 紧贴物体表面的流体层",
            "边界层分离: 形成漩涡区",
            "减阻措施: 优化形状、减小粗糙度",
        ]
    }

    /// 管道流动
    pub fn pipe_flow(&self) -> Vec<&'static str> {
        vec![
            "管流阻力: hf = λLv²/(2gd)",
            "摩擦系数 λ: 与Re和相对粗糙度有关",
            "层流摩擦系数: λ = 64/Re",
            "湍流摩擦系数: 查莫迪图",
            "局部阻力: 弯头、阀门、变径等",
            "局部阻力系数: ζ",
            "总阻力损失: h总 = hf + h局部",
            "管道设计: 确定管径、泵功率",
        ]
    }
}

impl Rule for FluidDynamicsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("fluid_dynamics")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "流体动力学规则",
            &[
                ("流体性质", &self.fluid_properties()),
                ("流体静力学", &self.fluid_statics()),
                ("流体运动学", &self.fluid_kinematics()),
                ("连续性方程", &self.continuity_equation()),
                ("伯努利方程", &self.bernoulli_equation()),
                ("雷诺数", &self.reynolds_number()),
                ("流体阻力", &self.fluid_resistance()),
                ("管道流动", &self.pipe_flow()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fluid_dynamics_rules() {
        let rules = FluidDynamicsRules::new();
        assert_eq!(rules.metadata().name, "流体动力学规则");
        assert!(!rules.explain().is_empty());
        assert!(!rules.fluid_properties().is_empty());
        assert!(!rules.bernoulli_equation().is_empty());
        assert!(!rules.pipe_flow().is_empty());
    }
}
