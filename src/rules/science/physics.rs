//! 物理定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 物理定律类型
#[derive(Debug, Clone)]
pub enum PhysicsLawType {
    /// 牛顿第一定律 (惯性定律)
    NewtonFirstLaw,
    /// 牛顿第二定律 (F=ma)
    NewtonSecondLaw,
    /// 牛顿第三定律 (作用与反作用)
    NewtonThirdLaw,
    /// 万有引力定律
    GravitationalLaw,
    /// 能量守恒定律
    EnergyConservation,
    /// 动量守恒定律
    MomentumConservation,
    /// 热力学第一定律
    ThermodynamicsFirstLaw,
    /// 热力学第二定律
    ThermodynamicsSecondLaw,
}

impl PhysicsLawType {
    pub fn name(&self) -> &'static str {
        match self {
            PhysicsLawType::NewtonFirstLaw => "牛顿第一定律 (惯性定律)",
            PhysicsLawType::NewtonSecondLaw => "牛顿第二定律",
            PhysicsLawType::NewtonThirdLaw => "牛顿第三定律",
            PhysicsLawType::GravitationalLaw => "万有引力定律",
            PhysicsLawType::EnergyConservation => "能量守恒定律",
            PhysicsLawType::MomentumConservation => "动量守恒定律",
            PhysicsLawType::ThermodynamicsFirstLaw => "热力学第一定律",
            PhysicsLawType::ThermodynamicsSecondLaw => "热力学第二定律",
        }
    }

    pub fn formula(&self) -> &'static str {
        match self {
            PhysicsLawType::NewtonFirstLaw => "物体在不受力或合力为零时保持静止或匀速直线运动",
            PhysicsLawType::NewtonSecondLaw => "F = ma",
            PhysicsLawType::NewtonThirdLaw => "F₁ = -F₂",
            PhysicsLawType::GravitationalLaw => "F = G(m₁m₂)/r²",
            PhysicsLawType::EnergyConservation => "E_total = E_kinetic + E_potential = constant",
            PhysicsLawType::MomentumConservation => "p₁ + p₂ = p₁' + p₂'",
            PhysicsLawType::ThermodynamicsFirstLaw => "ΔU = Q - W",
            PhysicsLawType::ThermodynamicsSecondLaw => "ΔS ≥ 0 (孤立系统)",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            PhysicsLawType::NewtonFirstLaw => {
                "一切物体在没有受到外力作用时，总保持匀速直线运动状态或静止状态"
            }
            PhysicsLawType::NewtonSecondLaw => "物体的加速度与所受合力成正比，与质量成反比",
            PhysicsLawType::NewtonThirdLaw => "两个物体之间的作用力和反作用力大小相等、方向相反",
            PhysicsLawType::GravitationalLaw => {
                "任意两个质点之间存在引力，大小与质量乘积成正比，与距离平方成反比"
            }
            PhysicsLawType::EnergyConservation => {
                "能量既不会凭空产生，也不会凭空消失，只能从一种形式转化为另一种形式"
            }
            PhysicsLawType::MomentumConservation => {
                "系统不受外力或所受外力之和为零时，系统总动量保持不变"
            }
            PhysicsLawType::ThermodynamicsFirstLaw => "系统内能的变化等于吸收的热量减去对外做的功",
            PhysicsLawType::ThermodynamicsSecondLaw => "热量不能自发地从低温物体传向高温物体",
        }
    }
}

/// 物理定律集合
pub struct PhysicsLaws {
    metadata: RuleMetadata,
}

impl PhysicsLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("物理定律", "经典物理学基本定律")
                .with_origin("物理学")
                .with_tags(vec!["科学".into(), "物理".into()]),
        }
    }

    pub fn all_laws() -> Vec<PhysicsLawType> {
        vec![
            PhysicsLawType::NewtonFirstLaw,
            PhysicsLawType::NewtonSecondLaw,
            PhysicsLawType::NewtonThirdLaw,
            PhysicsLawType::GravitationalLaw,
            PhysicsLawType::EnergyConservation,
            PhysicsLawType::MomentumConservation,
            PhysicsLawType::ThermodynamicsFirstLaw,
            PhysicsLawType::ThermodynamicsSecondLaw,
        ]
    }

    /// 计算牛顿第二定律
    pub fn calculate_force(mass: f64, acceleration: f64) -> f64 {
        mass * acceleration
    }

    /// 计算万有引力
    pub fn calculate_gravity(m1: f64, m2: f64, distance: f64) -> f64 {
        const G: f64 = 6.674e-11; // 万有引力常数
        G * m1 * m2 / (distance * distance)
    }

    /// 统计物理
    pub fn statistical_physics(&self) -> Vec<&'static str> {
        vec![
            "玻尔兹曼分布: 粒子在能级上的平衡分布P∝e^(-E/kT)",
            "费米狄拉克分布: 费米子在能级上的统计分布",
            "玻色爱因斯坦分布: 玻色子在能级上的统计分布",
            "麦克斯韦妖: 假想的违反热力学第二定律的微观分选者",
            "布朗运动: 悬浮微粒因分子碰撞产生的随机运动",
            "涨落耗散定理: 系统对微扰的响应与自发涨落相关",
        ]
    }

    /// 力学定律
    pub fn mechanics_laws(&self) -> Vec<&'static str> {
        vec![
            "胡克定律: 弹簧弹力与形变成正比F=-kx",
            "角动量守恒: 无外力矩时系统角动量保持不变",
            "转动惯量: 刚体转动惯性的度量I=Σmᵢrᵢ²",
            "平行轴定理: I=Icm+md²",
            "开普勒第三定律: T²∝a³轨道周期平方与半长轴立方成正比",
            "简谐运动: 回复力与位移成正比F=-kx",
            "单摆周期: T=2π√(L/g)小角度近似",
            "伯努利原理: 流体中流速大处压强小",
        ]
    }
}

impl Default for PhysicsLaws {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for PhysicsLaws {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("physics")
    }

    fn explain(&self) -> String {
        let laws = Self::all_laws();
        format!(
            "【物理定律】\n\n{}\n",
            laws.iter()
                .map(|law| format!(
                    "▶ {}\n   公式: {}\n   说明: {}\n",
                    law.name(),
                    law.formula(),
                    law.description()
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
    fn test_newton_second_law() {
        let force = PhysicsLaws::calculate_force(10.0, 2.0);
        assert_eq!(force, 20.0);
    }
}

#[cfg(test)]
mod extra_tests {
    use super::*;
    use crate::rules::core::ValidateContext;

    #[test]
    fn test_calculate_force_values() {
        // F = m * a
        assert_eq!(PhysicsLaws::calculate_force(0.0, 9.8), 0.0);
        assert_eq!(PhysicsLaws::calculate_force(1.0, 9.8), 9.8);
        assert_eq!(PhysicsLaws::calculate_force(100.0, 0.0), 0.0);
    }

    #[test]
    fn test_calculate_gravity() {
        // F = G * m1 * m2 / r^2 — 验证不 panic 且结果为正
        let force = PhysicsLaws::calculate_gravity(1e10, 1e10, 100.0);
        assert!(force > 0.0);
    }

    #[test]
    fn test_all_laws_not_empty() {
        let laws = PhysicsLaws::all_laws();
        assert!(!laws.is_empty());
    }

    #[test]
    fn test_mechanics_laws() {
        let rules = PhysicsLaws::new();
        assert!(!rules.mechanics_laws().is_empty());
    }

    #[test]
    fn test_rule_trait() {
        let rules = PhysicsLaws::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert!(!rules.explain().is_empty());
    }
}
