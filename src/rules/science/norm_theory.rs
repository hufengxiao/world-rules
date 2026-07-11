//! 范数理论规则
//!
//! 范数理论研究向量和矩阵的范数，是函数分析和数值代数的重要工具。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: NormTheoryRules,
    name: "范数理论规则",
    desc: "范数定义、性质与应用",
    origin: "数学",
    tags: ["科学", "数学", "代数", "分析"]
}

impl NormTheoryRules {
    /// 范数定义
    pub fn norm_definition(&self) -> Vec<&'static str> {
        vec![
            "范数定义: ||v|| 满足正性、齐次性、三角不等式",
            "正性: ||v|| ≥ 0，且 ||v|| = 0 iff v = 0",
            "齐次性: ||αv|| = |α|||v||",
            "三角不等式: ||u + v|| ≤ ||u|| + ||v||",
            "赋范空间: 配备范数的向量空间",
            "范数意义: 度量向量的大小",
            "距离: d(u, v) = ||u - v||",
            "范数与内积: 内积定义范数 ||v|| = √⟨v, v⟩",
        ]
    }

    /// 向量范数
    pub fn vector_norms(&self) -> Vec<&'static str> {
        vec![
            "p-范数: ||v||ₚ = (Σ|vᵢ|ᵖ)^(1/p)",
            "1-范数: ||v||₁ = Σ|vᵢ|",
            "2-范数: ||v||₂ = √(Σ|vᵢ|²)，欧几里得范数",
            "∞-范数: ||v||∞ = max|vᵢ|",
            "0-范数: 非零元素的个数",
            "范数等价: 不同范数的等价关系",
            "范数选择: 根据问题选择合适的范数",
            "范数几何: 不同范数的单位球形状不同",
        ]
    }

    /// 矩阵范数
    pub fn matrix_norms(&self) -> Vec<&'static str> {
        vec![
            "Frobenius 范数: ||A||F = √(Σ|aᵢⱼ|²)",
            "谱范数: ||A||₂ = σ₁，最大奇异值",
            "诱导范数: ||A||ₚ = sup||Ax||ₚ/||x||ₚ",
            "算子范数: 矩阵作为算子的范数",
            "1-范数: ||A||₁ = maxΣ|aᵢⱼ|（列和）",
            "∞-范数: ||A||∞ = maxΣ|aᵢⱼ|（行和）",
            "范数关系: ||Ax|| ≤ ||A||||x||",
            "范数应用: 误差估计、稳定性分析",
        ]
    }

    /// 范数性质
    pub fn norm_properties(&self) -> Vec<&'static str> {
        vec![
            "范数等价: 有限维空间所有范数等价",
            "等价关系: c₁||v||ₐ ≤ ||v||ₑ ≤ c₂||v||ₐ",
            "范数诱导拓扑: 范数定义拓扑结构",
            "完备性: 完备赋范空间是 Banach 空间",
            "收敛性: 范数收敛 ⇔ 每个分量收敛",
            "范数连续: v → ||v|| 连续",
            "范数凸性: 单位球是凸集",
            "范数严格凸: 单位球边界无线段",
        ]
    }

    /// 不等式
    pub fn norm_inequalities(&self) -> Vec<&'static str> {
        vec![
            "三角不等式: ||u + v|| ≤ ||u|| + ||v||",
            "反向三角不等式: |||u|| - ||v||| ≤ ||u - v||",
            "Hölder 不等式: |⟨u, v⟩| ≤ ||u||ₚ||v||q",
            "Cauchy-Schwarz: |⟨u, v⟩| ≤ ||u||₂||v||₂",
            "Minkowski 不等式: ||u + v||ₚ ≤ ||u||ₚ + ||v||ₚ",
            "矩阵范数不等式: ||AB|| ≤ ||A||||B||",
            "向量矩阵不等式: ||Av|| ≤ ||A||||v||",
            "范数稳定性: 误差传播估计",
        ]
    }

    /// 范数与收敛
    pub fn norm_convergence(&self) -> Vec<&'static str> {
        vec![
            "范数收敛: ||vₙ - v|| → 0",
            "收敛定义: 向量序列收敛到向量",
            "范数拓扑: 范数定义的拓扑",
            "强收敛: 范数收敛",
            "弱收敛: 每个泛函收敛",
            "Banach 空间: 完备赋范空间",
            "完备化: 不完备空间可完备化",
            "级数收敛: Σ||vₙ|| < ∞ ⇒ Σvₙ 收敛",
        ]
    }

    /// 范数与距离
    pub fn norm_distance(&self) -> Vec<&'static str> {
        vec![
            "距离定义: d(u, v) = ||u - v||",
            "距离性质: 正性、对称性、三角不等式",
            "度量空间: 配备距离的集合",
            "范数距离: 由范数诱导的距离",
            "距离几何: 距离空间的几何性质",
            "距离拓扑: 距离诱导拓扑",
            "距离收敛: d(vₙ, v) → 0",
            "距离完备: 距离空间的完备性",
        ]
    }

    /// 范数应用
    pub fn norm_applications(&self) -> Vec<&'static str> {
        vec![
            "误差分析: 用范数度量误差",
            "稳定性: 系统的范数稳定性",
            "优化: 范数优化问题",
            "逼近: 最佳逼近的范数条件",
            "机器学习: 范数作为损失函数",
            "信号处理: 信号范数分析",
            "数值方法: 方法的范数分析",
            "控制理论: 系统范数控制",
        ]
    }

    /// 特殊范数
    pub fn special_norms(&self) -> Vec<&'static str> {
        vec![
            "加权范数: ||v||w = √(Σwᵢ|vᵢ|²)",
            "核范数: ||A||* = Σσᵢ（奇异值和）",
            "迹范数: 矩阵的核范数",
            "Mahalanobis 范数: ||v||M = √(vᵀMv)",
            "范数球: {v | ||v|| ≤ 1}",
            "范数单位球: 范数为 1 的向量集合",
            "对偶范数: ||v||* = sup|⟨u, v⟩|/||u||",
            "范数族: p-范数族",
        ]
    }

    /// 应用实例
    pub fn applications(&self) -> Vec<&'static str> {
        vec![
            "数值分析: 算法误差估计",
            "机器学习: L1、L2 正则化",
            "信号处理: 信号误差度量",
            "图像处理: 图像距离",
            "优化理论: 约束范数优化",
            "控制理论: 系统稳定性",
            "量子力学: 状态范数",
            "经济学: 投资组合范数",
        ]
    }
}

impl Rule for NormTheoryRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("norm_theory")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "范数理论规则",
            &[
                ("范数定义", &self.norm_definition()),
                ("向量范数", &self.vector_norms()),
                ("矩阵范数", &self.matrix_norms()),
                ("范数性质", &self.norm_properties()),
                ("不等式", &self.norm_inequalities()),
                ("范数与收敛", &self.norm_convergence()),
                ("范数与距离", &self.norm_distance()),
                ("范数应用", &self.norm_applications()),
                ("特殊范数", &self.special_norms()),
                ("应用实例", &self.applications()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_norm_theory_rules() {
        let rules = NormTheoryRules::new();
        assert_eq!(rules.metadata().name, "范数理论规则");
        assert!(!rules.explain().is_empty());
        assert!(!rules.norm_definition().is_empty());
        assert!(!rules.vector_norms().is_empty());
        assert!(!rules.matrix_norms().is_empty());
    }
}
