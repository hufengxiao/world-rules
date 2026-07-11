//! 数论规则
//!
//! 数论研究整数及其性质，是数学中最古老的分支之一。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: NumberTheoryRules,
    name: "数论规则",
    desc: "整数性质、素数理论与应用",
    origin: "数学",
    tags: ["科学", "数学", "数论"]
}

impl NumberTheoryRules {
    /// 整数基础
    pub fn integer_basics(&self) -> Vec<&'static str> {
        vec![
            "整数集合: Z = {...,-2,-1,0,1,2,...}",
            "自然数: N = {0,1,2,...} 或 {1,2,3,...}",
            "整除性: a | b 表示 a 能整除 b",
            "除法定理: a = bq + r，0 ≤ r < |b|",
            "最大公约数: gcd(a,b) 是最大正公约数",
            "最小公倍数: lcm(a,b) 是最小正公倍数",
            "互质: gcd(a,b) = 1",
            "欧几里得算法: 计算 gcd 的方法",
        ]
    }

    /// 素数理论
    pub fn prime_number_theory(&self) -> Vec<&'static str> {
        vec![
            "素数定义: 只有 1 和自身两个正因子",
            "合数: 有多于两个正因子的数",
            "素数判定: 检查是否有其他因子",
            "埃拉托斯特尼筛法: 找素数的方法",
            "素数无穷性: 素数有无限多个",
            "素数定理: π(x) ~ x/ln(x)",
            "梅森素数: 2^p - 1 形式的素数",
            "费马素数: 2^(2^n) + 1 形式的素数",
        ]
    }

    /// 整数的唯一分解
    pub fn unique_factorization(&self) -> Vec<&'static str> {
        vec![
            "算术基本定理: 每个整数可唯一分解为素数乘积",
            "标准分解: n = p₁^a₁·p₂^a₂·...·p_k^a_k",
            "因子个数: τ(n) = (a₁+1)(a₂+1)...(a_k+1)",
            "因子之和: σ(n) = σ(p₁^a₁)·σ(p₂^a₂)·...",
            "完美数: σ(n) = 2n（因子之和等于两倍本身）",
            "欧拉公式: σ(p^a) = (p^(a+1)-1)/(p-1)",
            "gcd 与 lcm 关系: gcd(a,b)·lcm(a,b) = a·b",
            "费马小定理: a^(p-1) ≡ 1 (mod p)（p 为素数）",
        ]
    }

    /// 同余理论
    pub fn congruence_theory(&self) -> Vec<&'static str> {
        vec![
            "同余定义: a ≡ b (mod n) 表示 n | (a-b)",
            "同余性质: 加、减、乘保持同余",
            "同余类: [a] = {a + kn | k ∈ Z}",
            "完全剩余系: {0,1,...,n-1}",
            "简化剩余系: 与 n 互质的剩余系",
            "欧拉函数 φ(n): 与 n 互质且小于 n 的正整数个数",
            "欧拉定理: a^φ(n) ≡ 1 (mod n)（gcd(a,n)=1）",
            "威尔逊定理: (p-1)! ≡ -1 (mod p)（p 为素数）",
        ]
    }

    /// 线性同余方程
    pub fn linear_congruence(&self) -> Vec<&'static str> {
        vec![
            "方程形式: ax ≡ b (mod n)",
            "可解性: 有解 iff gcd(a,n) | b",
            "求解方法: 用欧几里得算法",
            "解的个数: gcd(a,n) 个解",
            "中国剩余定理: 多个同余方程的解",
            "CRT 形式: x ≡ a_i (mod n_i)，n_i 互质",
            "应用: 大数计算、密码学",
            "孙子算经: 中国剩余定理的最早记载",
        ]
    }

    /// 二次剩余
    pub fn quadratic_residues(&self) -> Vec<&'static str> {
        vec![
            "二次剩余: x² ≡ a (mod p) 有解",
            "二次非剩余: 无解",
            "勒让德符号: (a/p) = ±1",
            "欧拉判别法: (a/p) ≡ a^(p-1)/2 (mod p)",
            "二次互反律: (p/q)(q/p) = (-1)^[(p-1)(q-1)/4]",
            "高斯引理: 计算勒让德符号",
            "雅可比符号: 推广到合数",
            "应用: 素数判定、密码学",
        ]
    }

    /// 特殊数列
    pub fn special_sequences(&self) -> Vec<&'static str> {
        vec![
            "斐波那契数列: F_n = F_{n-1} + F_{n-2}",
            "斐波那契性质: F_n/F_{n-1} → 黄金比例",
            "卢卡斯数列: 类似斐波那契，不同初值",
            "三角形数: T_n = n(n+1)/2",
            "平方数: n²",
            "立方数: n³",
            "六边形数: H_n = 2n² - n",
            "五角数定理: 欧拉发现的数列关系",
        ]
    }

    /// 代数数论
    pub fn algebraic_number_theory(&self) -> Vec<&'static str> {
        vec![
            "代数整数: 多项式方程的整数根",
            "代数数域: Q 的有限扩张",
            "素理想分解: 素数在数域中的分解",
            "类数: 理想类群的阶数",
            "唯一分解域: 类数为 1 的数域",
            "单位群: 数域中的可逆元素",
            "理想理论: Dedekind 的理想概念",
            "应用: 椭圆曲线、密码学",
        ]
    }

    /// 解析数论
    pub fn analytic_number_theory(&self) -> Vec<&'static str> {
        vec![
            "黎曼 ζ 函数: ζ(s) = ∑1/n^s",
            "欧拉乘积: ζ(s) = ∏(1 - 1/p^s)^{-1}",
            "黎曼猜想: ζ(s) 的非平凡零点在临界线",
            "素数定理证明: 用 ζ 函数",
            "筛法: Brun 篮、Selberg 篮",
            "哥德巴赫猜想: 每个偶数 > 2 是两素数之和",
            "孪生素数猜想: 孪生素数有无限多",
            "应用: 素数分布、密码学",
        ]
    }

    /// 应用领域
    pub fn applications(&self) -> Vec<&'static str> {
        vec![
            "RSA 加密: 基于大素数分解难题",
            "椭圆曲线密码: 基于离散对数问题",
            "随机数生成: 用数论方法",
            "编码理论: 纠错码设计",
            "算法设计: 快速计算方法",
            "组合数学: 计数和证明",
            "数学竞赛: 基础训练内容",
            "计算机科学: 离散数学基础",
        ]
    }
}

impl Rule for NumberTheoryRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("number_theory")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "数论规则",
            &[
                ("整数基础", &self.integer_basics()),
                ("素数理论", &self.prime_number_theory()),
                ("整数的唯一分解", &self.unique_factorization()),
                ("同余理论", &self.congruence_theory()),
                ("线性同余方程", &self.linear_congruence()),
                ("二次剩余", &self.quadratic_residues()),
                ("特殊数列", &self.special_sequences()),
                ("代数数论", &self.algebraic_number_theory()),
                ("解析数论", &self.analytic_number_theory()),
                ("应用领域", &self.applications()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_theory_rules() {
        let rules = NumberTheoryRules::new();
        assert_eq!(rules.metadata().name, "数论规则");
        assert!(!rules.explain().is_empty());
        assert!(!rules.integer_basics().is_empty());
        assert!(!rules.prime_number_theory().is_empty());
        assert!(!rules.unique_factorization().is_empty());
        assert!(!rules.congruence_theory().is_empty());
        assert!(!rules.linear_congruence().is_empty());
        assert!(!rules.quadratic_residues().is_empty());
        assert!(!rules.special_sequences().is_empty());
        assert!(!rules.algebraic_number_theory().is_empty());
        assert!(!rules.analytic_number_theory().is_empty());
        assert!(!rules.applications().is_empty());
    }
}
