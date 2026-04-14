//! 流体力学定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 流体力学定律集合
pub struct FluidMechanicsLaws {
    metadata: RuleMetadata,
}

impl FluidMechanicsLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "流体力学定律",
                "流体力学基本定律"
            )
            .with_origin("物理学")
            .with_tags(vec!["科学".into(), "物理".into(), "流体".into()]),
        }
    }

    /// 流体静力学定律
    pub fn hydrostatic_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("帕斯卡定律", "压强传递", "外加压强等值传递"),
            ("流体压强定律", "P = ρgh", "液体压强与深度关系"),
            ("阿基米德定律", "F = ρgV", "浮力等于排开液体重量"),
            ("连通器定律", "同液面", "连通器同液面等高"),
            ("液压定律", "帕斯卡原理", "液压系统原理"),
            ("静水压定律", "压力分布", "静止液体压力分布"),
        ]
    }

    /// 流体动力学定律
    pub fn hydrodynamic_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("伯努利定律", "P + ρv²/2 + ρgh = 常数", "流体能量守恒"),
            ("连续性定律", "A₁v₁ = A₂v₂", "流量连续方程"),
            ("欧拉方程", "流体运动方程", "理想流体运动方程"),
            ("纳维-斯托克斯方程", "粘性流体方程", "粘性流体运动方程"),
            ("雷诺定律", "Re = ρvd/μ", "雷诺数判别流动状态"),
            ("层流定律", "Re < 2300", "层流流动条件"),
            ("湍流定律", "Re > 4000", "湍流流动条件"),
            ("边界层定律", "粘性边界", "流体边界层"),
        ]
    }

    /// 空气动力学定律
    pub fn aerodynamic_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("升力定律", "L = ½ρv²SCₗ", "机翼升力公式"),
            ("阻力定律", "D = ½ρv²SCₓ", "气动阻力公式"),
            ("马格努斯效应", "旋转升力", "旋转物体升力"),
            ("音速定律", "a = √(γRT)", "声音传播速度"),
            ("马赫数定律", "M = v/a", "速度与音速比"),
            ("激波定律", "超声速激波", "超声速飞行激波"),
            ("压缩性定律", "密度变化", "高速流体压缩"),
            ("气动加热定律", "摩擦生热", "高速飞行加热"),
        ]
    }

    /// 水动力学定律
    pub fn hydrodynamic_water_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("波浪定律", "波浪传播", "水面波浪传播规律"),
            ("潮汐定律", "引力潮汐", "天体引力产生潮汐"),
            ("洋流定律", "海水流动", "大洋环流规律"),
            ("水击定律", "压力波动", "管道水击现象"),
            ("空化定律", "气泡形成", "液体空化现象"),
            ("渗透定律", "达西定律", "液体渗透规律"),
        ]
    }

    /// 流体性质
    pub fn fluid_properties(&self) -> Vec<&'static str> {
        vec![
            "密度",
            "粘度",
            "压缩性",
            "表面张力",
            "毛细现象",
            "润湿性",
            "流变性质",
            "牛顿流体",
            "非牛顿流体",
        ]
    }

    /// 流体现象
    pub fn phenomena(&self) -> Vec<&'static str> {
        vec![
            "涡流",
            "湍流",
            "边界层分离",
            "卡门涡街",
            "激波",
            "膨胀波",
            "激波边界层干扰",
            "流动不稳定性",
            "旋涡脱落",
        ]
    }
}

impl Default for FluidMechanicsLaws {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for FluidMechanicsLaws {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("fluid_mechanics")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【流体力学定律】\n\n静力学定律:\n{}\n\n动力学定律:\n{}\n\n空气动力学定律:\n{}\n",
            self.hydrostatic_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.hydrodynamic_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.aerodynamic_laws().iter()
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
    fn test_fluid_mechanics_laws() {
        let laws = FluidMechanicsLaws::new();
        assert!(!laws.hydrostatic_laws().is_empty());
        assert!(!laws.hydrodynamic_laws().is_empty());
    }
}