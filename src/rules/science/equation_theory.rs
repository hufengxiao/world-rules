//! 方程理论规则
//!
//! 方程理论研究方程的性质、解法和可解性，是代数学的核心内容。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: EquationTheoryRules,
    name: "方程理论规则",
    desc: "方程求解方法与可解性理论",
    origin: "数学",
    tags: ["科学", "数学", "代数", "方程"]
}

impl EquationTheoryRules {
    /// 方程基本概念
    pub fn equation_basics(&self) -> Vec<&'static str> {
        vec![
            "方程定义: 含未知量的等式",
            "未知量: 用符号表示的待求量",
            "方程类型: 线性方程、二次方程、高次方程",
            "方程组: 多个方程同时求解",
            "解的定义: 使方程成立的未知量值",
            "解集: 所有解的集合",
            "无解方程: 没有解的方程",
            "恒等方程: 任意值都是解",
        ]
    }

    /// 线性方程
    pub fn linear_equations(&self) -> Vec<&'static str> {
        vec![
            "一元线性: ax + b = 0，解为 x = -b/a",
            "二元线性: ax + by = c",
            "方程组: 多个线性方程",
            "消元法: 通过消元求解",
            "代入法: 用代入求解",
            "矩阵法: 用矩阵表示和求解",
            "行列式: Cramer 法则",
            "唯一解条件: 行列式不为零",
        ]
    }

    /// 二次方程
    pub fn quadratic_equations(&self) -> Vec<&'static str> {
        vec![
            "标准形式: ax² + bx + c = 0",
            "求根公式: x = (-b ± √(b²-4ac))/(2a)",
            "判别式: Δ = b² - 4ac",
            "两实根: Δ > 0",
            "一实根: Δ = 0",
            "两复根: Δ < 0",
            "Vieta 公式: x₁ + x₂ = -b/a, x₁x₂ = c/a",
            "因式分解: a(x-x₁)(x-x₂) = 0",
        ]
    }

    /// 高次方程
    pub fn higher_degree_equations(&self) -> Vec<&'static str> {
        vec![
            "三次方程: ax³ + bx² + cx + d = 0",
            "Cardano 公式: 三次方程的根式解",
            "四次方程: Ferrari 方法",
            "五次及以上: 一般无根式解",
            "Abel 定理: 五次及以上一般方程不可解",
            "特殊方程: 特殊形式可能可解",
            "数值解: Newton 法等数值方法",
            "近似解: 逼近法求解",
        ]
    }

    /// 方程组
    pub fn equation_systems(&self) -> Vec<&'static str> {
        vec![
            "线性方程组: 矩阵法求解",
            "非线性方程组: 更复杂的求解",
            "独立方程: 每个方程提供新信息",
            "方程个数: 决定解的个数",
            "自由变量: 决定解的维度",
            "唯一解: 方程足够且独立",
            "多解: 方程不足",
            "无解: 方程矛盾",
        ]
    }

    /// 特殊方程
    pub fn special_equations(&self) -> Vec<&'static str> {
        vec![
            "齐次方程: 常数项为零",
            "对称方程: 具有对称性",
            "递归方程: 与递推关系",
            "差分方程: 差分形式",
            "函数方程: 含函数的方程",
            "参数方程: 含参数的方程",
            "同余方程: 模运算方程",
            "超越方程: 含超越函数",
        ]
    }

    /// 可解性理论
    pub fn solvability_theory(&self) -> Vec<&'static str> {
        vec![
            "可解定义: 能用根式表示根",
            "伽罗瓦群: 描述方程的对称性",
            "可解群: 群是可解的",
            "可解方程: 伽罗瓦群可解的方程",
            "不可解方程: 伽罗瓦群不可解",
            "Abel-Ruffini 定理: 一般五次方程不可解",
            "特殊可解: 某些特殊五次方程可解",
            "根式解: 解用根式表示",
        ]
    }

    /// 数值方法
    pub fn numerical_methods(&self) -> Vec<&'static str> {
        vec![
            "Newton 法: xₙ₊₁ = xₙ - f(xₙ)/f'(xₙ)",
            "二分法: 逐步缩小区间",
            "迭代法: 反复迭代逼近",
            "不动点法: 求函数的不动点",
            "割线法: 不用导数的 Newton 法",
            "Muller 法: 二次逼近",
            "迭代加速: 加速收敛",
            "收敛条件: 方法收敛的条件",
        ]
    }

    /// 应用实例
    pub fn applications(&self) -> Vec<&'static str> {
        vec![
            "物理问题: 物理方程求解",
            "工程计算: 工程中的方程",
            "经济模型: 经济方程",
            "优化问题: 方程优化",
            "信号处理: 滤波方程",
            "控制理论: 系统方程",
            "计算几何: 几何方程",
            "机器学习: 学习方程",
        ]
    }
}

impl Rule for EquationTheoryRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("equation_theory")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "方程理论规则",
            &[
                ("方程基本概念", &self.equation_basics()),
                ("线性方程", &self.linear_equations()),
                ("二次方程", &self.quadratic_equations()),
                ("高次方程", &self.higher_degree_equations()),
                ("方程组", &self.equation_systems()),
                ("特殊方程", &self.special_equations()),
                ("可解性理论", &self.solvability_theory()),
                ("数值方法", &self.numerical_methods()),
                ("应用实例", &self.applications()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equation_theory_rules() {
        let rules = EquationTheoryRules::new();
        assert_eq!(rules.metadata().name, "方程理论规则");
        assert!(!rules.explain().is_empty());
        assert!(!rules.equation_basics().is_empty());
        assert!(!rules.quadratic_equations().is_empty());
        assert!(!rules.solvability_theory().is_empty());
    }
}