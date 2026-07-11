//! 刚体动力学规则
//!
//! 刚体动力学研究刚体（不变形物体）的运动规律。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: RigidBodyDynamicsRules,
    name: "刚体动力学规则",
    desc: "刚体平动、转动和平衡的基本定律",
    origin: "力学",
    tags: ["科学", "物理", "力学", "刚体"]
}

impl RigidBodyDynamicsRules {
    /// 刚体平动
    pub fn translation(&self) -> Vec<&'static str> {
        vec![
            "平动定义: 刚体各点运动轨迹相同",
            "平动特点: 各点速度和加速度相同",
            "平动简化: 可视为质点运动",
            "平动运动方程: 同质点运动方程",
            "平动牛顿方程: F = ma",
            "平动动量: p = mv",
            "平动动能: Ek = ½mv²",
            "平动应用: 汽车、船舶的运动",
        ]
    }

    /// 刚体转动
    pub fn rotation(&self) -> Vec<&'static str> {
        vec![
            "转动定义: 绕固定轴的旋转运动",
            "角位移 θ: 转过的角度",
            "角速度 ω: ω = dθ/dt",
            "角加速度 α: α = dω/dt",
            "转动方程: θ = θ₀ + ω₀t + ½αt²",
            "角速度变化: ω = ω₀ + αt",
            "匀速转动: α = 0，ω = const",
            "匀加速转动: α = const",
        ]
    }

    /// 转动惯量
    pub fn moment_of_inertia(&self) -> Vec<&'static str> {
        vec![
            "转动惯量定义: I = Σmr²",
            "转动惯量意义: 转动惯性大小的度量",
            "细杆（中心）: I = mL²/12",
            "细杆（端点）: I = mL²/3",
            "圆盘: I = mR²/2",
            "圆柱: I = mR²/2（绕轴线）",
            "球体: I = 2mR²/5（绕中心）",
            "平行轴定理: I = I₀ + md²",
        ]
    }

    /// 转动动力学
    pub fn rotational_dynamics(&self) -> Vec<&'static str> {
        vec![
            "转动定律: M = Iα（类似 F = ma）",
            "力矩: M = Fr = Fr sinθ",
            "角动量: L = Iω（类似 p = mv）",
            "角动量定理: M = dL/dt",
            "角动量守恒: 外力矩为零时角动量不变",
            "转动动能: Ek = ½Iω²",
            "转动功率: P = Mω",
            "转动动能定理: W = ΔEk = ½(Iω₂² - Iω₁²)",
        ]
    }

    /// 转动与平动的联系
    pub fn translation_rotation_relation(&self) -> Vec<&'static str> {
        vec![
            "滚动运动: 平动与转动的组合",
            "滚动条件: v = ωR（纯滚动）",
            "滚动动能: Ek = ½mv² + ½Iω²",
            "滚动动能简化: Ek = ½(m + I/R²)v²",
            "静摩擦力: 使滚动不滑动",
            "滚动摩擦力: 比滑动摩擦小",
            "滚动应用: 车轮、滚筒",
            "瞬心: 瞬时转动中心",
        ]
    }

    /// 刚体平衡
    pub fn rigid_body_equilibrium(&self) -> Vec<&'static str> {
        vec![
            "平衡条件: ΣF = 0 和 ΣM = 0",
            "力平衡: 合力为零",
            "力矩平衡: 合力矩为零（对任意轴）",
            "二力平衡: 两个力大小相等、方向相反",
            "三力平衡: 三力汇交于一点",
            "平衡类型: 稳定、不稳定、随遇",
            "重心位置: 重力作用点",
            "稳定条件: 重心在支撑面内",
        ]
    }

    /// 刚体碰撞
    pub fn rigid_body_collision(&self) -> Vec<&'static str> {
        vec![
            "碰撞类型: 弹性、非弹性",
            "动量守恒: 系统总动量不变",
            "角动量守恒: 系统总角动量不变",
            "能量守恒: 弹性碰撞动能守恒",
            "碰撞冲量: Δp = FΔt",
            "碰撞力矩冲量: ΔL = MΔt",
            "碰撞恢复系数: e = v₂ - v₁ / v₁₀ - v₂₀",
            "碰撞应用: 体育运动、机械传动",
        ]
    }

    /// 刚体应用
    pub fn applications(&self) -> Vec<&'static str> {
        vec![
            "陀螺仪: 角动量守恒的应用",
            "飞轮: 储存转动能量",
            "齿轮传动: 力矩和转速变换",
            "连杆机构: 传递力和运动",
            "机械臂: 刚体运动控制",
            "车辆动力学: 平动与转动组合",
            "航天器姿态: 角动量控制",
            "机器人运动: 刚体动力学分析",
        ]
    }
}

impl Rule for RigidBodyDynamicsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("rigid_body_dynamics")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "刚体动力学规则",
            &[
                ("刚体平动", &self.translation()),
                ("刚体转动", &self.rotation()),
                ("转动惯量", &self.moment_of_inertia()),
                ("转动动力学", &self.rotational_dynamics()),
                ("转动与平动的联系", &self.translation_rotation_relation()),
                ("刚体平衡", &self.rigid_body_equilibrium()),
                ("刚体碰撞", &self.rigid_body_collision()),
                ("刚体应用", &self.applications()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rigid_body_dynamics_rules() {
        let rules = RigidBodyDynamicsRules::new();
        assert_eq!(rules.metadata().name, "刚体动力学规则");
        assert!(!rules.explain().is_empty());
        assert!(!rules.rotation().is_empty());
        assert!(!rules.moment_of_inertia().is_empty());
        assert!(!rules.rotational_dynamics().is_empty());
    }
}
