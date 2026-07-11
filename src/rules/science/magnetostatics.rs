//! 静磁学规则
//!
//! 静磁学研究恒定电流产生的磁场及其相互作用。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: MagnetostaticsRules,
    name: "静磁学规则",
    desc: "静磁场、电流与磁相互作用分析方法",
    origin: "电磁学",
    tags: ["科学", "物理", "电磁", "静磁"]
}

impl MagnetostaticsRules {
    /// 磁场基础
    pub fn magnetic_field_basics(&self) -> Vec<&'static str> {
        vec![
            "磁场定义: B = F/(qv)，运动电荷受到的磁场力",
            "磁感应强度: B，描述磁场强弱和方向的物理量",
            "磁场方向: 小磁针N极指向",
            "磁通量: Φ = ∮B·dA，磁场穿过面的总量",
            "磁场线: 从N极出发、终止于S极的闭合曲线",
            "磁场线性质: 不相交、密度表示场强、切线表示方向",
            "磁单极子: 目前未发现磁单极子",
            "磁场叠加: 多个电流源磁场的矢量和",
        ]
    }

    /// 毕奥-萨伐尔定律
    pub fn biot_savart_law(&self) -> Vec<&'static str> {
        vec![
            "毕奥-萨伐尔定律: dB = μ₀Idl×r/(4πr³)",
            "真空磁导率: μ₀ = 4π×10⁻⁷ T·m/A",
            "电流元磁场: Idl 产生的磁场垂直于电流和距离",
            "直线电流磁场: B = μ₀I/(2πr)",
            "圆环电流中心磁场: B = μ₀I/(2R)",
            "无限长螺线管磁场: B = μ₀nI",
            "有限长螺线管磁场: B = μ₀nI(cosθ₁-cosθ₂)/2",
            "叠加原理: 总磁场等于各电流元磁场的叠加",
        ]
    }

    /// 安培定律
    pub fn ampere_law(&self) -> Vec<&'static str> {
        vec![
            "安培定律: ∮B·dl = μ₀I，磁场沿回路积分",
            "安培定律微分形式: ∇×B = μ₀J",
            "电流密度: J = I/A，单位面积通过的电流",
            "对称性应用: 利用对称性简化磁场计算",
            "无限长直导线: B = μ₀I/(2πr)",
            "同轴电缆: 两导线间磁场",
            "螺绕环磁场: B = μ₀NI/(2πR)",
            "安培环路: 选择合适的积分路径",
        ]
    }

    /// 磁偶极子
    pub fn magnetic_dipole(&self) -> Vec<&'static str> {
        vec![
            "磁偶极子: 闭合电流环产生的磁场",
            "磁矩定义: μ = IA，电流环的磁矩",
            "磁矩方向: 垂直于电流平面，右手定则",
            "远处磁场: B ≈ μ₀μ/(4πr³)，类似电偶极子",
            "磁偶极子受力: F = μ×B，磁场对磁矩的力",
            "磁偶极子力矩: τ = μ×B，磁场对磁矩的力矩",
            "磁势能: U = -μ·B，磁矩在磁场中的势能",
            "地球磁场: 地球可视为大磁偶极子",
        ]
    }

    /// 磁力与洛伦兹力
    pub fn magnetic_force(&self) -> Vec<&'static str> {
        vec![
            "洛伦兹力: F = q(E + v×B)，电磁场对电荷的力",
            "磁场力: F = qv×B = Bqv sinθ",
            "磁场力方向: 垂直于速度和磁场",
            "磁场力不做功: 磁场力只改变速度方向",
            "带电粒子圆周运动: qvB = mv²/r",
            "回旋半径: r = mv/(qB)",
            "回旋频率: f = qB/(2πm)",
            "霍尔效应: 磁场中导体产生横向电压",
        ]
    }

    /// 磁介质
    pub fn magnetic_materials(&self) -> Vec<&'static str> {
        vec![
            "磁化强度: M，材料的磁化程度",
            "磁场强度: H = B/μ₀ - M",
            "磁导率: μ = μ₀μᵣ，材料的磁导性质",
            "相对磁导率: μᵣ = μ/μ₀",
            "顺磁质: μᵣ > 1，弱磁性材料",
            "抗磁质: μᵣ < 1，弱磁性材料",
            "铁磁质: μᵣ >> 1，强磁性材料",
            "磁滞回线: 铁磁质磁化过程的B-H曲线",
        ]
    }

    /// 磁路定律
    pub fn magnetic_circuit(&self) -> Vec<&'static str> {
        vec![
            "磁阻: Rₘ = l/(μA)，磁路阻力",
            "磁动势: Fₘ = NI，产生磁场的源",
            "磁路定律: Φ = Fₘ/Rₘ，类比欧姆定律",
            "磁路串联: Rₘ = Rₘ₁ + Rₘ₂ + ...",
            "磁路并联: 1/Rₘ = 1/Rₘ₁ + 1/Rₘ₂ + ...",
            "气隙磁阻: 气隙磁阻远大于铁芯磁阻",
            "漏磁通: 未通过主磁路的磁通",
            "磁饱和: 铁磁质磁化达到饱和",
        ]
    }

    /// 应用实例
    pub fn applications(&self) -> Vec<&'static str> {
        vec![
            "电磁铁: 利用电流产生强磁场",
            "永久磁铁: 铁磁材料保持磁性",
            "磁存储: 磁带、磁盘存储信息",
            "磁传感器: 霍尔传感器、磁阻传感器",
            "核磁共振: 利用磁场进行成像",
            "粒子加速器: 磁场引导粒子轨迹",
            "磁悬浮: 利用磁场实现悬浮",
            "地球磁场导航: 利用地球磁场定位",
        ]
    }
}

impl Rule for MagnetostaticsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("magnetostatics")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "静磁学规则",
            &[
                ("磁场基础", &self.magnetic_field_basics()),
                ("毕奥-萨伐尔定律", &self.biot_savart_law()),
                ("安培定律", &self.ampere_law()),
                ("磁偶极子", &self.magnetic_dipole()),
                ("磁力与洛伦兹力", &self.magnetic_force()),
                ("磁介质", &self.magnetic_materials()),
                ("磁路定律", &self.magnetic_circuit()),
                ("应用实例", &self.applications()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_magnetostatics_rules() {
        let rules = MagnetostaticsRules::new();
        assert_eq!(rules.metadata().name, "静磁学规则");
        assert!(!rules.explain().is_empty());
        assert!(!rules.magnetic_field_basics().is_empty());
        assert!(!rules.biot_savart_law().is_empty());
        assert!(!rules.ampere_law().is_empty());
    }
}
