//! 静电学规则
//!
//! 静电学研究静止电荷产生的电场及其相互作用。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: ElectrostaticsRules,
    name: "静电学规则",
    desc: "静电场、电荷分布与电势分析方法",
    origin: "电磁学",
    tags: ["科学", "物理", "电磁", "静电"]
}

impl ElectrostaticsRules {
    /// 库仑定律与电荷
    pub fn coulomb_law(&self) -> Vec<&'static str> {
        vec![
            "库仑定律: F = kq₁q₂/r²，两个点电荷之间的静电力",
            "库仑常数: k = 1/(4πε₀) ≈ 8.99×10⁹ N·m²/C²",
            "电荷守恒: 电荷既不能创造也不能消灭，只能转移",
            "电荷量子化: 所有电荷都是基本电荷 e 的整数倍",
            "基本电荷: e = 1.602×10⁻¹⁹ C",
            "电荷分布: 点电荷、线电荷、面电荷、体电荷",
            "电荷密度: λ = dq/dl（线）、σ = dq/dA（面）、ρ = dq/dV（体）",
            "叠加原理: 多个电荷的总电场等于各电荷电场的矢量和",
        ]
    }

    /// 电场强度
    pub fn electric_field(&self) -> Vec<&'static str> {
        vec![
            "电场定义: E = F/q，单位正电荷受到的电场力",
            "点电荷电场: E = kq/r²，方向沿径向",
            "电场叠加: E = ΣEᵢ，各电荷电场的矢量和",
            "均匀电场: 各点电场强度大小和方向相同",
            "电场线: 从正电荷出发、终止于负电荷的曲线",
            "电场线性质: 不相交、密度表示场强、切线表示方向",
            "电偶极子: 两个等量异号电荷组成的系统",
            "电偶极子电场: 远处电场 E ≈ kp/r³",
        ]
    }

    /// 高斯定律
    pub fn gauss_law(&self) -> Vec<&'static str> {
        vec![
            "高斯定律: ∮E·dA = Q/ε₀，闭合面电通量",
            "电通量: Φ = ∮E·dA，电场穿过面的总量",
            "真空介电常数: ε₀ = 8.854×10⁻¹² F/m",
            "对称性应用: 利用对称性简化电场计算",
            "球对称: 均匀带电球体外部电场等效于点电荷",
            "柱对称: 无限长均匀带电圆柱的电场",
            "面对称: 无限大均匀带电平面的电场",
            "导体内部: 导体内部电场为零",
        ]
    }

    /// 电势与电势能
    pub fn electric_potential(&self) -> Vec<&'static str> {
        vec![
            "电势能: W = qV，电荷在电场中的势能",
            "电势定义: V = W/q = -∫E·dl，单位电荷的势能",
            "电势差: ΔV = V₂ - V₁ = -∫E·dl",
            "点电荷电势: V = kq/r",
            "电势叠加: V = ΣVᵢ，各电荷电势的代数和",
            "等势面: 电势相等的点构成的面",
            "等势面性质: 电场线垂直于等势面",
            "电势与电场: E = -∇V，电场是电势的负梯度",
        ]
    }

    /// 电容与电容器
    pub fn capacitance(&self) -> Vec<&'static str> {
        vec![
            "电容定义: C = Q/V，储存电荷的能力",
            "平行板电容: C = ε₀A/d，面积A、间距d",
            "电容并联: C = C₁ + C₂ + ...",
            "电容串联: 1/C = 1/C₁ + 1/C₂ + ...",
            "电容能量: W = ½CV² = ½QV = ½Q²/C",
            "电介质: 插入电介质使电容增大",
            "相对介电常数: K = ε/ε₀，电介质的介电性质",
            "电介质电容: C = KC₀，比真空电容大K倍",
        ]
    }

    /// 导体与静电平衡
    pub fn conductors(&self) -> Vec<&'static str> {
        vec![
            "静电平衡: 导体内部电场为零",
            "电荷分布: 导体电荷分布在表面",
            "表面电场: E = σ/ε₀，垂直于表面",
            "等势体: 导体各点电势相等",
            "尖端效应: 尖端电荷密度大，电场强",
            "静电屏蔽: 导体壳可屏蔽外电场",
            "法拉第笼: 金属笼可保护内部免受电场影响",
            "接地: 导体接地后电势为零",
        ]
    }

    /// 电介质
    pub fn dielectrics(&self) -> Vec<&'static str> {
        vec![
            "电介质极化: 电介质在外电场中产生极化电荷",
            "极化强度: P = χₑε₀E，极化程度",
            "电极化率: χₑ，材料极化能力",
            "电位移矢量: D = ε₀E + P = εE",
            "束缚电荷: 极化产生的电荷，不能自由移动",
            "介电常数: ε = Kε₀，材料的介电性质",
            "击穿电压: 电介质能承受的最大电压",
            "能量密度: u = ½εE²，电介质中的电场能量",
        ]
    }

    /// 应用实例
    pub fn applications(&self) -> Vec<&'static str> {
        vec![
            "静电除尘: 利用静电吸附粉尘",
            "静电喷涂: 利用静电使涂料均匀附着",
            "静电复印: 静电成像和显影技术",
            "静电纺丝: 利用静电制备纳米纤维",
            "电容器储能: 电容器储存电能用于闪光灯等",
            "静电屏蔽: 保护电子设备免受静电干扰",
            "静电测量: 测量电荷、电场、电势",
            "静电安全: 防止静电放电危害",
        ]
    }
}

impl Rule for ElectrostaticsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("electrostatics")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "静电学规则",
            &[
                ("库仑定律", &self.coulomb_law()),
                ("电场强度", &self.electric_field()),
                ("高斯定律", &self.gauss_law()),
                ("电势与电势能", &self.electric_potential()),
                ("电容与电容器", &self.capacitance()),
                ("导体与静电平衡", &self.conductors()),
                ("电介质", &self.dielectrics()),
                ("应用实例", &self.applications()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_electrostatics_rules() {
        let rules = ElectrostaticsRules::new();
        assert_eq!(rules.metadata().name, "静电学规则");
        assert!(!rules.explain().is_empty());
        assert!(!rules.coulomb_law().is_empty());
        assert!(!rules.electric_field().is_empty());
        assert!(!rules.gauss_law().is_empty());
    }
}
