//! 分析力学规则
//!
//! 分析力学用能量和广义坐标描述力学系统。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: AnalyticalMechanicsRules,
    name: "分析力学规则",
    desc: "分析力学基本概念与拉格朗日、哈密顿力学",
    origin: "力学",
    tags: ["科学", "物理", "力学", "分析力学"]
}

impl AnalyticalMechanicsRules {
    /// 广义坐标
    pub fn generalized_coordinates(&self) -> Vec<&'static str> {
        vec![
            "广义坐标: 独立描述系统位形的变量",
            "自由度: 广义坐标数目",
            "广义位移: δq（虚位移）",
            "广义速度: dq/dt",
            "广义力: Q（与广义坐标对应）",
            "约束条件: 限制系统运动的条件",
            "完整约束: 可用方程表示的约束",
            "非完整约束: 不可用方程表示的约束",
        ]
    }

    /// 虚功原理
    pub fn virtual_work(&self) -> Vec<&'static str> {
        vec![
            "虚位移: 满足约束条件的假想位移",
            "虚功: δW = ΣF·δr",
            "虚功原理: 系统平衡时虚功为零",
            "理想约束: 约束反力不做功",
            "虚功方程: ΣQ·δq = 0",
            "平衡条件: 所有广义力为零",
            "虚功应用: 求解静力学问题",
            "虚功意义: 分析力学的基础",
        ]
    }

    /// 拉格朗日力学
    pub fn lagrangian_mechanics(&self) -> Vec<&'static str> {
        vec![
            "拉格朗日量: L = T - V（动能减势能）",
            "广义动能: T = Σ½mv²",
            "广义势能: V(q, t)",
            "拉格朗日方程: d(dL/dq)/dt - dL/dq = Q",
            "第一类拉格朗日方程: 含约束条件",
            "第二类拉格朗日方程: 无约束条件",
            "保守系统: Q = 0，拉格朗日方程简化",
            "拉格朗日优势: 约束处理方便",
        ]
    }

    /// 拉格朗日方程应用
    pub fn lagrangian_applications(&self) -> Vec<&'static str> {
        vec![
            "单摆: L = mL²θ'²/2 - mgL cosθ",
            "双摆: 两个自由度系统",
            "弹簧振子: L = mx'²/2 - kx²/2",
            "有心力运动: 角动量守恒",
            "耦合振子: 多自由度振动",
            "相对运动: 在转动参考系中",
            "电磁场中带电粒子: L含电磁势",
            "拉格朗日方程求解: 系统运动方程",
        ]
    }

    /// 哈密顿力学
    pub fn hamiltonian_mechanics(&self) -> Vec<&'static str> {
        vec![
            "哈密顿量: H = T + V（总能量）",
            "广义动量: p = dL/dq'",
            "哈密顿方程: q' = dH/dp, p' = -dH/dq",
            "正则方程: 哈密顿方程",
            "相空间: q-p 空间",
            "相轨迹: 系统在相空间中的轨迹",
            "守恒量: 与哈密顿量对称性有关",
            "哈密顿优势: 量子力学基础",
        ]
    }

    /// 哈密顿方程应用
    pub fn hamiltonian_applications(&self) -> Vec<&'static str> {
        vec![
            "谐振子: H = p²/2m + kx²/2",
            "自由粒子: H = p²/2m",
            "有心力场: 角动量守恒",
            "刚体转动: H = L²/2I",
            "周期运动: 相空间闭合轨迹",
            "散射问题: 轨迹不闭合",
            "量子对应: 哈密顿量子化",
            "统计力学: 能量分布计算",
        ]
    }

    /// 守恒定律
    pub fn conservation_laws(&self) -> Vec<&'static str> {
        vec![
            "能量守恒: H 不含时间",
            "动量守恒: 系统平移不变",
            "角动量守恒: 系统转动不变",
            "诺特定理: 对称性与守恒量对应",
            "时间对称: 能量守恒",
            "空间对称: 动量守恒",
            "转动对称: 角动量守恒",
            "规范对称: 电荷守恒",
        ]
    }

    /// 变分原理
    pub fn variational_principle(&self) -> Vec<&'static str> {
        vec![
            "最小作用原理: δS = 0",
            "作用量: S = ∫L dt",
            "变分: 虚改变量",
            "极值条件: 作用量取极值",
            "真实路径: 满足最小作用原理",
            "欧拉-拉格朗日方程: 由变分导出",
            "变分优势: 统一处理力学问题",
            "广义应用: 场论、量子力学",
        ]
    }
}

impl Rule for AnalyticalMechanicsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("analytical_mechanics")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "分析力学规则",
            &[
                ("广义坐标", &self.generalized_coordinates()),
                ("虚功原理", &self.virtual_work()),
                ("拉格朗日力学", &self.lagrangian_mechanics()),
                ("拉格朗日方程应用", &self.lagrangian_applications()),
                ("哈密顿力学", &self.hamiltonian_mechanics()),
                ("哈密顿方程应用", &self.hamiltonian_applications()),
                ("守恒定律", &self.conservation_laws()),
                ("变分原理", &self.variational_principle()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analytical_mechanics_rules() {
        let rules = AnalyticalMechanicsRules::new();
        assert_eq!(rules.metadata().name, "分析力学规则");
        assert!(!rules.explain().is_empty());
        assert!(!rules.generalized_coordinates().is_empty());
        assert!(!rules.lagrangian_mechanics().is_empty());
        assert!(!rules.hamiltonian_mechanics().is_empty());
    }
}
