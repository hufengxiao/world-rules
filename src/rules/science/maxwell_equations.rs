//! 麦克斯韦方程组规则
//!
//! 麦克斯韦方程组是电磁学的核心方程，描述电磁场的产生和传播。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: MaxwellEquationsRules,
    name: "麦克斯韦方程组规则",
    desc: "电磁场基本方程与物理意义",
    origin: "电磁学",
    tags: ["科学", "物理", "电磁", "麦克斯韦"]
}

impl MaxwellEquationsRules {
    /// 高斯电场定律
    pub fn gauss_electric_law(&self) -> Vec<&'static str> {
        vec![
            "积分形式: ∮E·dA = Q/ε₀",
            "微分形式: ∇·E = ρ/ε₀",
            "物理意义: 电场来源于电荷",
            "电场散度: 正电荷为源，负电荷为汇",
            "电荷密度: ρ = dq/dV",
            "真空介电常数: ε₀ = 8.854×10⁻¹² F/m",
            "对称应用: 计算对称电荷分布的电场",
            "导体应用: 导体内部电场为零",
        ]
    }

    /// 高斯磁通定律
    pub fn gauss_magnetic_law(&self) -> Vec<&'static str> {
        vec![
            "积分形式: ∮B·dA = 0",
            "微分形式: ∇·B = 0",
            "物理意义: 磁场无源无汇",
            "磁单极子: 目前未发现磁单极子",
            "磁通守恒: 磁场线是闭合曲线",
            "磁场连续: 磁场线不中断",
            "磁力线性质: 从N极到S极再回到N极",
            "磁通量: 进入和离开闭合面的磁通相等",
        ]
    }

    /// 法拉第感应定律
    pub fn faraday_induction_law(&self) -> Vec<&'static str> {
        vec![
            "积分形式: ∮E·dl = -dΦ/dt",
            "微分形式: ∇×E = -∂B/∂t",
            "物理意义: 变化的磁场产生电场",
            "感应电场: 非保守场，有旋度",
            "涡旋电场: 垂直于磁场变化方向",
            "电动势: 沿闭合回路的电场积分",
            "磁通变化: 磁场、面积、角度变化",
            "电磁感应: 发电机、变压器基础",
        ]
    }

    /// 安培-麦克斯韦定律
    pub fn ampere_maxwell_law(&self) -> Vec<&'static str> {
        vec![
            "积分形式: ∮B·dl = μ₀I + μ₀ε₀(dΦₑ/dt)",
            "微分形式: ∇×B = μ₀J + μ₀ε₀(∂E/∂t)",
            "物理意义: 电流和变化电场产生磁场",
            "传导电流: 导体中的电荷流动",
            "位移电流: μ₀ε₀(∂E/∂t)",
            "电流连续性: ∇·J + ∂ρ/∂t = 0",
            "真空磁导率: μ₀ = 4π×10⁻⁷ T·m/A",
            "麦克斯韦贡献: 补充位移电流项",
        ]
    }

    /// 辅助方程
    pub fn auxiliary_equations(&self) -> Vec<&'static str> {
        vec![
            "电荷守恒: ∇·J + ∂ρ/∂t = 0",
            "洛伦兹力: F = q(E + v×B)",
            "本构关系: D = ε₀E + P",
            "本构关系: H = B/μ₀ - M",
            "介质方程: D = εE",
            "介质方程: B = μH",
            "能量密度: u = ½(E·D + B·H)",
            "能流密度: S = E×H（坡印廷矢量）",
        ]
    }

    /// 电磁波方程
    pub fn electromagnetic_wave_equation(&self) -> Vec<&'static str> {
        vec![
            "波动方程: ∇²E - (1/c²)(∂²E/∂t²) = 0",
            "波动方程: ∇²B - (1/c²)(∂²B/∂t²) = 0",
            "波速推导: c = 1/√(ε₀μ₀) = 3×10⁸ m/s",
            "横波特性: E⊥B⊥传播方向",
            "场关系: E = cB",
            "正弦波: E = E₀sin(kx - ωt)",
            "波长频率: λf = c",
            "能量传播: 坡印廷矢量 S = E×H",
        ]
    }

    /// 边界条件
    pub fn boundary_conditions(&self) -> Vec<&'static str> {
        vec![
            "电场边界: E₁ₜ = E₂ₜ（切向分量连续）",
            "电场边界: D₁ₙ - D₂ₙ = σ（法向分量）",
            "磁场边界: H₁ₜ - H₂ₜ = K（切向分量）",
            "磁场边界: B₁ₙ = B₂ₙ（法向分量连续）",
            "介质界面: 电场线折射",
            "介质界面: 磁场线折射",
            "导体表面: Eₜ = 0，Eₙ = σ/ε₀",
            "导体表面: Bₙ = 0",
        ]
    }

    /// 应用实例
    pub fn applications(&self) -> Vec<&'static str> {
        vec![
            "电磁波传播: 光、无线电波",
            "天线设计: 发射和接收电磁波",
            "光学器件: 镜子、透镜",
            "微波技术: 雷达、通信",
            "光纤通信: 光波传输信息",
            "等离子体: 电磁波在等离子体中传播",
            "量子电动力学: 微观电磁相互作用",
            "计算电磁学: 数值求解麦克斯韦方程",
        ]
    }
}

impl Rule for MaxwellEquationsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("maxwell_equations")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "麦克斯韦方程组规则",
            &[
                ("高斯电场定律", &self.gauss_electric_law()),
                ("高斯磁通定律", &self.gauss_magnetic_law()),
                ("法拉第感应定律", &self.faraday_induction_law()),
                ("安培-麦克斯韦定律", &self.ampere_maxwell_law()),
                ("辅助方程", &self.auxiliary_equations()),
                ("电磁波方程", &self.electromagnetic_wave_equation()),
                ("边界条件", &self.boundary_conditions()),
                ("应用实例", &self.applications()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maxwell_equations_rules() {
        let rules = MaxwellEquationsRules::new();
        assert_eq!(rules.metadata().name, "麦克斯韦方程组规则");
        assert!(!rules.explain().is_empty());
        assert!(!rules.gauss_electric_law().is_empty());
        assert!(!rules.gauss_magnetic_law().is_empty());
    }
}
