//! 电磁学定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 电磁学定律集合
pub struct ElectromagnetismLaws {
    metadata: RuleMetadata,
}

impl ElectromagnetismLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "电磁学定律",
                "电磁学基本定律"
            )
            .with_origin("物理学")
            .with_tags(vec!["科学".into(), "物理".into(), "电磁".into()]),
        }
    }

    /// 麦克斯韦方程组
    pub fn maxwell_equations(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("高斯电场定律", "∇·E = ρ/ε₀", "电场散度等于电荷密度"),
            ("高斯磁通定律", "∇·B = 0", "磁场散度为零，不存在磁单极子"),
            ("法拉第感应定律", "∇×E = -∂B/∂t", "变化的磁场产生电场"),
            ("安培-麦克斯韦定律", "∇×B = μ₀J + μ₀ε₀∂E/∂t", "电流和变化的电场产生磁场"),
        ]
    }

    /// 其他电磁定律
    pub fn other_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("库仑定律", "F = k(q₁q₂)/r²", "电荷间的作用力"),
            ("欧姆定律", "V = IR", "电压、电流与电阻的关系"),
            ("焦耳定律", "P = I²R", "电流通过电阻产生的热量"),
            ("楞次定律", "感应电流阻碍变化", "感应电流方向阻碍磁通变化"),
            ("毕奥-萨伐尔定律", "B = μ₀I/(4πr)", "电流产生的磁场"),
            ("洛伦兹力定律", "F = q(E + v×B)", "电磁场对电荷的作用力"),
            ("法拉第电磁感应定律", "ε = -dΦ/dt", "感应电动势与磁通变化率"),
            ("楞次定律", "ε = -N·dΦ/dt", "感应电动势方向"),
        ]
    }

    /// 电磁常数
    pub fn constants(&self) -> Vec<(&'static str, f64, &'static str)> {
        vec![
            ("真空介电常数 ε₀", 8.854e-12, "F/m"),
            ("真空磁导率 μ₀", 1.257e-6, "H/m"),
            ("库仑常数 k", 8.988e9, "N·m²/C²"),
            ("光速 c", 2.998e8, "m/s"),
            ("电子电荷 e", 1.602e-19, "C"),
        ]
    }

    /// 电磁现象
    pub fn phenomena(&self) -> Vec<&'static str> {
        vec![
            "电磁波传播",
            "电磁感应",
            "电磁屏蔽",
            "电磁辐射",
            "电磁共振",
            "电磁波谱",
            "无线电波",
            "微波",
            "红外线",
            "可见光",
            "紫外线",
            "X射线",
            "伽马射线",
        ]
    }

    /// 静电学
    pub fn electrostatics(&self) -> Vec<&'static str> {
        vec![
            "库仑定律: F=kq₁q₂/r²两个点电荷之间的静电力",
            "高斯定律: 通过闭合曲面的电通量等于内部电荷除以ε₀",
            "电场强度: 单位正电荷在电场中受到的力E=F/q",
            "电势: 单位电荷在电场中某点的电势能",
            "电容: 导体储存电荷能力C=Q/V",
            "介电常数: 描述电介质对电场影响的物理量",
        ]
    }

    /// 静磁学
    pub fn magnetostatics(&self) -> Vec<&'static str> {
        vec![
            "安培定律: 磁场沿闭合回路积分等于μ₀乘以穿过电流",
            "毕奥萨伐尔定律: 电流元产生磁场dB=μ₀Idl×r/(4πr³)",
            "磁通量: 通过某面积的磁力线总量Φ=B·A",
            "法拉第定律: 变化的磁通量产生感应电动势",
            "楞次定律: 感应电流方向阻碍磁通量变化",
            "自感: 线圈中电流变化在自身产生感应电动势",
            "互感: 一个线圈电流变化在另一个线圈产生电动势",
        ]
    }

}

impl Default for ElectromagnetismLaws {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ElectromagnetismLaws {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("electromagnetism")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【电磁学定律】\n\n麦克斯韦方程组:\n{}\n\n其他定律:\n{}\n",
            self.maxwell_equations().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.other_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_electromagnetism_laws() {
        let laws = ElectromagnetismLaws::new();
        assert!(!laws.maxwell_equations().is_empty());
    }
}