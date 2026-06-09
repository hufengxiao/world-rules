//! 量子力学定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 量子力学定律集合
pub struct QuantumMechanicsLaws {
    metadata: RuleMetadata,
}

impl QuantumMechanicsLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("量子力学定律", "量子物理学基本定律")
                .with_origin("物理学")
                .with_tags(vec!["科学".into(), "物理".into(), "量子".into()]),
        }
    }

    /// 量子力学定律列表
    pub fn all_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("薛定谔方程", "Ĥψ = Eψ", "描述量子系统状态的演化方程"),
            (
                "海森堡不确定性原理",
                "ΔxΔp ≥ ℏ/2",
                "不可能同时精确测量粒子的位置和动量",
            ),
            (
                "波函数坍缩",
                "测量导致量子态坍缩",
                "量子系统被测量时从叠加态坍缩到确定态",
            ),
            (
                "量子叠加原理",
                "ψ = Σcₙψₙ",
                "量子系统可以同时处于多个状态的叠加",
            ),
            ("量子纠缠", "非定域关联", "两个粒子可以产生超越空间的关联"),
            ("德布罗意波", "λ = h/p", "所有物质都具有波动性"),
            (
                "玻尔对应原理",
                "量子→经典",
                "量子力学在大尺度下趋近经典力学",
            ),
            (
                "泡利不相容原理",
                "不能相同量子态",
                "两个费米子不能同时占据相同量子态",
            ),
            ("玻色-爱因斯坦统计", "玻色子分布", "玻色子的量子统计规律"),
            ("费米-狄拉克统计", "费米子分布", "费米子的量子统计规律"),
            (
                "量子隧穿效应",
                "穿越势垒",
                "粒子可以穿越经典力学无法通过的势垒",
            ),
            ("量子化条件", "角动量量子化", "量子系统的角动量只能是离散值"),
        ]
    }

    /// 量子常数
    pub fn constants(&self) -> Vec<(&'static str, f64, &'static str)> {
        vec![
            ("普朗克常数 h", 6.626e-34, "J·s"),
            ("约化普朗克常数 ℏ", 1.055e-34, "J·s"),
            ("电子电荷 e", 1.602e-19, "C"),
            ("电子质量 mₑ", 9.109e-31, "kg"),
            ("玻尔半径 a₀", 5.292e-11, "m"),
            ("精细结构常数 α", 0.007297, "无量纲"),
        ]
    }

    /// 量子现象
    pub fn phenomena(&self) -> Vec<&'static str> {
        vec![
            "量子干涉",
            "量子衍射",
            "量子超导",
            "量子霍尔效应",
            "量子计算",
            "量子通信",
            "量子加密",
            "量子测量",
        ]
    }

    /// 量子态与算符
    pub fn quantum_states(&self) -> Vec<&'static str> {
        vec![
            "态叠加原理: 量子态可以是多个本征态的线性组合",
            "算符对易关系: 位置与动量满足[x,p]=iℏ",
            "本征值方程: 算符作用于态矢量等于本征值乘以态矢量",
            "密度矩阵: 描述混合态和统计系综的数学工具",
            "量子纠缠: 复合系统不能写成子系统态的直积",
            "贝尔不等式: 经典隐变量理论对关联的限制",
            "量子退相干: 量子态与环境相互作用失去相干性",
        ]
    }

    /// 量子效应
    pub fn quantum_effects(&self) -> Vec<&'static str> {
        vec![
            "量子隧穿: 粒子可以穿越经典禁区的势垒",
            "量子霍尔效应: 二维电子气在强磁场中电导量子化",
            "卡西米尔效应: 真空中两平行导体板之间的吸引力",
            "兰姆位移: 氢原子2S和2P能级的微小偏移",
            "AB效应: 矢量势对带电粒子波函数的可干涉影响",
            "超导BCS理论: 电子通过声子相互作用形成库珀对",
            "量子限域效应: 纳米尺度材料中能级离散化",
        ]
    }

    /// 量子信息
    pub fn quantum_information(&self) -> Vec<&'static str> {
        vec![
            "量子比特: 可处于0和1叠加态的信息单元",
            "量子门: 对量子比特进行操作的酉变换",
            "量子隐形传态: 利用纠缠传输量子态",
            "量子密钥分发: 利用量子原理安全分发密钥",
            "量子计算: 利用量子叠加和纠缠加速计算",
            "量子纠错: 保护量子信息免受退相干影响",
            "量子优势: 量子计算机超越经典计算机的能力",
        ]
    }
}

impl Default for QuantumMechanicsLaws {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for QuantumMechanicsLaws {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("quantum_mechanics")
    }

    fn explain(&self) -> String {
        format!(
            "【量子力学定律】\n\n{}\n",
            self.all_laws()
                .iter()
                .map(|(name, formula, desc)| format!(
                    "▶ {}\n   公式/原理: {}\n   说明: {}\n",
                    name, formula, desc
                ))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantum_mechanics_laws() {
        let laws = QuantumMechanicsLaws::new();
        assert!(!laws.all_laws().is_empty());
    }
}
