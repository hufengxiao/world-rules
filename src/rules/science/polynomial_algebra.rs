//! 多项式代数规则
//!
//! 多项式代数研究多项式的运算、性质和因式分解，是代数学的重要内容。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: PolynomialAlgebraRules,
    name: "多项式代数规则",
    desc: "多项式运算、性质与因式分解",
    origin: "数学",
    tags: ["科学", "数学", "代数", "多项式"]
}

impl PolynomialAlgebraRules {
    /// 多项式定义
    pub fn polynomial_definition(&self) -> Vec<&'static str> {
        vec![
            "多项式定义: f(x) = aₙxⁿ + aₙ₋₁xⁿ₋₁ + ... + a₁x + a₀",
            "系数: aᵢ 来自系数环或域 R",
            "多项式环: R[x] 是系数在 R 上的多项式集合",
            "多项式度: deg(f) = 最高次项的指数 n",
            "零多项式: 所有系数为 0，度定义为 -∞",
            "常数多项式: 度为 0（或零多项式）",
            "首项系数: 最高次项的系数 aₙ",
            "首一多项式: 首项系数为 1 的多项式",
        ]
    }

    /// 多项式运算
    pub fn polynomial_operations(&self) -> Vec<&'static str> {
        vec![
            "加法: 对应系数相加",
            "减法: 对应系数相减",
            "乘法: (fg)(x) = Σᵢₖ aᵢbₖxⁱ⁺ᵏ",
            "度与运算: deg(f+g) ≤ max(deg(f), deg(g))",
            "度与乘法: deg(fg) = deg(f) + deg(g)（无零因子时）",
            "除法算法: f(x) = q(x)g(x) + r(x)，deg(r) < deg(g)",
            "商式: q(x) 是除法的商",
            "余式: r(x) 是除法的余",
        ]
    }

    /// 因式分解
    pub fn factorization(&self) -> Vec<&'static str> {
        vec![
            "因子定义: f = gh，则 g, h 是 f 的因子",
            "不可约多项式: 不能分解为更低次多项式的乘积",
            "可约多项式: 可以分解的多项式",
            "唯一分解: 域上多项式可唯一分解为不可约因子",
            "分解算法: 试除法、Berlekamp 算法",
            "因式定理: α 是根 iff (x-α) 是因子",
            "重因子: 因子出现多次",
            "因子度: 因子的次数",
        ]
    }

    /// 根与多项式
    pub fn roots_and_polynomials(&self) -> Vec<&'static str> {
        vec![
            "根定义: f(α) = 0，α 是多项式的根",
            "根与因子: α 是根 iff (x-α) 是因子",
            "根的个数: n 次多项式最多有 n 个根",
            "重根: 根出现多次",
            "单根: 只出现一次",
            "根的存在域: 根可能不在系数域中",
            "代数基本定理: C 上 n 次多项式恰有 n 个根",
            "Vieta 公式: 根与系数的关系",
        ]
    }

    /// 特殊多项式
    pub fn special_polynomials(&self) -> Vec<&'static str> {
        vec![
            "二次多项式: ax² + bx + c",
            "判别式: Δ = b² - 4ac",
            "三次多项式: ax³ + bx² + cx + d",
            "四次多项式: 可分解求解",
            "五次及以上: 一般无公式解",
            "对称多项式: 变量交换不变",
            "初等对称多项式: σ₁, σ₂, ..., σₙ",
            "齐次多项式: 各项次数相同",
        ]
    }

    /// 多项式环性质
    pub fn polynomial_ring_properties(&self) -> Vec<&'static str> {
        vec![
            "R[x] 是环: 多项式环是环",
            "交换环: R 是交换环时 R[x] 是交换环",
            "幺环: R 有幺元时 R[x] 有幺元 1",
            "整环: R 是整环时 R[x] 是整环",
            "UFD: R 是 UFD 时 R[x] 是 UFD",
            "PID: R 是域时 R[x] 不是 PID",
            "诺特环: R 是诺特环时 R[x] 是诺特环",
            "多项式环扩张: R[x,y] = R[x][y]",
        ]
    }

    /// 多项式算法
    pub fn polynomial_algorithms(&self) -> Vec<&'static str> {
        vec![
            "除法算法: 长除法计算商和余",
            "Euclid 算法: 计算最大公因子",
            "扩展 Euclid: 找到 gcd 的表示",
            "快速乘法: FFT 加速多项式乘法",
            "因式分解: 试除法、Berlekamp 算法",
            "根计算: Newton 法、迭代法",
            "多项式求值: Horner 方法",
            "插值: Lagrange 插值、Newton 插值",
        ]
    }

    /// 多元多项式
    pub fn multivariate_polynomials(&self) -> Vec<&'static str> {
        vec![
            "多元定义: f(x₁,...,xₙ) 是多元多项式",
            "多项式环: R[x₁,...,xₙ]",
            "单项式: c·x₁ᵃ₁··xₙᵃₙ",
            "多项式度: 各单项式的最大总度",
            "对称多项式: 变量任意交换不变",
            "初等对称多项式: σₖ = Σx₁··xₖ",
            "对称多项式定理: 任何对称多项式可用初等对称多项式表示",
            "理想: 多元多项式环的理想",
        ]
    }

    /// 应用实例
    pub fn applications(&self) -> Vec<&'static str> {
        vec![
            "方程求解: 多项式方程的根",
            "编码理论: 多项式码",
            "信号处理: 多项式滤波器",
            "计算代数: 多项式计算",
            "代数几何: 多项式与几何对象",
            "密码学: 多项式上的密码",
            "数值计算: 多项式逼近",
            "控制系统: 多项式传递函数",
        ]
    }
}

impl Rule for PolynomialAlgebraRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("polynomial_algebra")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "多项式代数规则",
            &[
                ("多项式定义", &self.polynomial_definition()),
                ("多项式运算", &self.polynomial_operations()),
                ("因式分解", &self.factorization()),
                ("根与多项式", &self.roots_and_polynomials()),
                ("特殊多项式", &self.special_polynomials()),
                ("多项式环性质", &self.polynomial_ring_properties()),
                ("多项式算法", &self.polynomial_algorithms()),
                ("多元多项式", &self.multivariate_polynomials()),
                ("应用实例", &self.applications()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polynomial_algebra_rules() {
        let rules = PolynomialAlgebraRules::new();
        assert_eq!(rules.metadata().name, "多项式代数规则");
        assert!(!rules.explain().is_empty());
        assert!(!rules.polynomial_definition().is_empty());
        assert!(!rules.factorization().is_empty());
        assert!(!rules.roots_and_polynomials().is_empty());
    }
}
