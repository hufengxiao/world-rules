//! 实分析规则
//!
//! 实分析研究实数、函数、极限和积分的严格理论。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: RealAnalysisRules,
    name: "实分析规则",
    desc: "实数、函数与积分的严格理论",
    origin: "数学",
    tags: ["科学", "数学", "分析"]
}

impl RealAnalysisRules {
    /// 实数系统
    pub fn real_number_system(&self) -> Vec<&'static str> {
        vec![
            "实数定义: 有理数和无理数的集合",
            "有理数: 可表示为 p/q 的数",
            "无理数: 不能表示为分数的数",
            "实数的完备性: 有序域且完备",
            "Archimedean 性质: 无最大元素",
            "有界集: 存在上下界",
            "确界存在定理: 有界集必有确界",
            "实数的稠密性: 任意两实数间有无理数",
        ]
    }

    /// 序列与极限
    pub fn sequences_and_limits(&self) -> Vec<&'static str> {
        vec![
            "序列定义: {a_n}，无穷数列",
            "极限定义: lim[n→∞]a_n = a",
            "收敛序列: 有极限的序列",
            "发散序列: 无极限的序列",
            "唯一性: 极限唯一",
            "有界性: 收敛序列必有界",
            "保序性: 极限保持不等式关系",
            "子序列: 原序列中选取的部分",
        ]
    }

    /// 连续性
    pub fn continuity(&self) -> Vec<&'static str> {
        vec![
            "连续定义: lim[x→c]f(x) = f(c)",
            "点连续: 函数在某点连续",
            "区间连续: 在区间每点连续",
            "间断点: 函数不连续的点",
            "第一类间断: 左右极限存在但不相等",
            "第二类间断: 左右极限至少一个不存在",
            "连续函数性质: 介值定理、极值定理",
            "一致连续: 连续性在区间上一致",
        ]
    }

    /// 可微性
    pub fn differentiability(&self) -> Vec<&'static str> {
        vec![
            "导数定义: f'(x) = lim[h→0](f(x+h)-f(x))/h",
            "可导点: 导数存在",
            "左导数、右导数: 单侧导数",
            "可导与连续: 可导必连续",
            "不可导点: 导数不存在",
            "微分中值定理: Rolle、Lagrange、Cauchy",
            "泰勒定理: 用多项式逼近函数",
            "高阶导数: f^(n)(x)",
        ]
    }

    /// Riemann 积分
    pub fn riemann_integration(&self) -> Vec<&'static str> {
        vec![
            "Riemann 积分定义: 分割求和的极限",
            "分割: 区间 [a,b] 的划分",
            "积分和: Σf(x_i*)Δx_i",
            "积分上限、下限: 上下积分",
            "可积条件: Darboux 条件",
            "可积函数: 连续函数必可积",
            "积分性质: 线性性、单调性",
            "积分不等式: 积分的比较",
        ]
    }

    /// Lebesgue 积分
    pub fn lebesgue_integration(&self) -> Vec<&'static str> {
        vec![
            "测度理论: 集合的大小",
            "Lebesgue 测度: 实数集的标准测度",
            "可测函数: 测度理论中的函数",
            "Lebesgue 积分定义: 基于测度",
            "积分收敛定理: 单调收敛、控制收敛",
            "Fatou 引理: 序列积分的下界",
            "可积空间: L^p 空间",
            "Lebesgue vs Riemann: 更一般化",
        ]
    }

    /// 函数空间
    pub fn function_spaces(&self) -> Vec<&'static str> {
        vec![
            "连续函数空间 C[a,b]: 区间上的连续函数",
            "可积函数空间 L^1: Lebesgue 可积",
            "平方可积 L^2: 能量有限函数",
            "范数: ||f||_p = (∫|f|^p dx)^(1/p)",
            "完备性: 函数空间的完备性",
            "内积空间: L^2 有内积",
            "函数序列收敛: 点收敛、一致收敛",
            "弱收敛: 函数空间中的收敛",
        ]
    }

    /// Fourier 分析
    pub fn fourier_analysis(&self) -> Vec<&'static str> {
        vec![
            "Fourier 级数: f(x) = Σa_n cos(nx) + b_n sin(nx)",
            "系数计算: a_n = (1/π)∫f(x)cos(nx)dx",
            "收敛性: 点收敛、一致收敛",
            "Parseval 定理: 能量等式",
            "Fourier 变换: f̂(ω) = ∫f(x)e^(-iωx)dx",
            "逆变换: f(x) = (1/2π)∫f̂(ω)e^(iωx)dω",
            "卷积: (f*g)(x) = ∫f(t)g(x-t)dt",
            "应用: 信号处理、物理",
        ]
    }

    /// 测度理论
    pub fn measure_theory(&self) -> Vec<&'static str> {
        vec![
            "测度定义: 集合函数 μ",
            "测度性质: 非负性、可加性",
            "可测集: 测度定义的集合",
            "零测集: 测度为 0",
            "几乎处处: 除了零测集",
            "σ-代数: 测度的域",
            "Borel 集: 开集生成的 σ-代数",
            "完备测度: 包含所有子集",
        ]
    }

    /// 应用领域
    pub fn applications(&self) -> Vec<&'static str> {
        vec![
            "物理学: 量子力学、统计物理",
            "信号处理: Fourier 分析",
            "概率论: Lebesgue 积分",
            "泛函分析: 函数空间",
            "微分方程: 解的存在性",
            "数值分析: 函数逼近",
            "经济学: 优化理论",
            "控制理论: 系统分析",
        ]
    }
}

impl Rule for RealAnalysisRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("real_analysis")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "实分析规则",
            &[
                ("实数系统", &self.real_number_system()),
                ("序列与极限", &self.sequences_and_limits()),
                ("连续性", &self.continuity()),
                ("可微性", &self.differentiability()),
                ("Riemann 积分", &self.riemann_integration()),
                ("Lebesgue 积分", &self.lebesgue_integration()),
                ("函数空间", &self.function_spaces()),
                ("Fourier 分析", &self.fourier_analysis()),
                ("测度理论", &self.measure_theory()),
                ("应用领域", &self.applications()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_real_analysis_rules() {
        let rules = RealAnalysisRules::new();
        assert_eq!(rules.metadata().name, "实分析规则");
        assert!(!rules.explain().is_empty());
        assert!(!rules.real_number_system().is_empty());
        assert!(!rules.sequences_and_limits().is_empty());
        assert!(!rules.continuity().is_empty());
        assert!(!rules.differentiability().is_empty());
        assert!(!rules.riemann_integration().is_empty());
        assert!(!rules.lebesgue_integration().is_empty());
        assert!(!rules.function_spaces().is_empty());
        assert!(!rules.fourier_analysis().is_empty());
        assert!(!rules.measure_theory().is_empty());
        assert!(!rules.applications().is_empty());
    }
}
