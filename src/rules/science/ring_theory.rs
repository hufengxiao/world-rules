//! 环论规则
//!
//! 环论是研究环结构的代数学分支，环是比群更复杂的代数结构。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: RingTheoryRules,
    name: "环论规则",
    desc: "环的定义、理想与环同态",
    origin: "数学",
    tags: ["科学", "数学", "代数", "环论"]
}

impl RingTheoryRules {
    /// 环的基本定义
    pub fn ring_definition(&self) -> Vec<&'static str> {
        vec![
            "环定义: 集合 R 配合两种运算（加法 + 和乘法 ·）",
            "加法群: R 对加法构成阿贝尔群",
            "乘法半群: R 对乘法满足结合律",
            "分配律: a·(b+c) = a·b + a·c 和 (a+b)·c = a·c + b·c",
            "零元: 加法单位元 0，a + 0 = a",
            "负元: 加法逆元 -a，a + (-a) = 0",
            "幺环: 有乘法单位元 1 的环",
            "交换环: 乘法满足交换律的环",
        ]
    }

    /// 特殊环
    pub fn special_rings(&self) -> Vec<&'static str> {
        vec![
            "整数环 Z: 最基本的交换幺环",
            "多项式环 R[x]: 系数在环 R 上的多项式",
            "矩阵环 M_n(R): 环 R 上的 n×n 矩阵",
            "域: 每个非零元素有乘法逆元的环",
            "整环: 无零因子、非零元素有逆元的交换幺环",
            "零因子: a ≠ 0, b ≠ 0 但 a·b = 0",
            "除环: 非零元素都有逆元的环（可能非交换）",
            "布尔环: 每个元素满足 x² = x 的环",
        ]
    }

    /// 理想
    pub fn ideals(&self) -> Vec<&'static str> {
        vec![
            "理想定义: I 是 R 的子集，对加法和乘法封闭",
            "左理想: r ∈ R, a ∈ I ⇒ ra ∈ I",
            "右理想: r ∈ R, a ∈ I ⇒ ar ∈ I",
            "双边理想: 同时是左理想和右理想",
            "主理想: 由一个元素生成的理想 ⟨a⟩",
            "理想运算: I + J、IJ、I ∩ J 都是理想",
            "极大理想: 不被其他真理想包含的理想",
            "素理想: ab ∈ P ⇒ a ∈ P 或 b ∈ P",
        ]
    }

    /// 商环
    pub fn quotient_rings(&self) -> Vec<&'static str> {
        vec![
            "商环定义: R/I，I 是 R 的双边理想",
            "商环元素: I 的陪集 {r + I | r ∈ R}",
            "商环运算: (a+I)(b+I) = ab + I",
            "商环意义: 对环进行简化",
            "同构定理: R/I ≅ R'/J",
            "第一同构定理: R/Ker(f) ≅ Im(f)",
            "第二同构定理: (R/I)/(J/I) ≅ R/J",
            "第三同构定理: R/(I+J) ≅ (R/I)×(R/J)",
        ]
    }

    /// 理想与商环关系
    pub fn ideal_quotient_relations(&self) -> Vec<&'static str> {
        vec![
            "理想对应: 商环的理想对应原环的理想",
            "理想包含: I ⊆ J 对应 R/J 的理想",
            "素理想判定: I 是素理想 iff R/I 是整环",
            "极大理想判定: I 是极大理想 iff R/I 是域",
            "零理想: {0} 对应商环 R 本身",
            "整个环: R 本身是理想，但 R/R 是零环",
            "根理想: √I = {r | rⁿ ∈ I 对某 n}",
            "Jacobson 根: 所有极大理想的交",
        ]
    }

    /// 环同态
    pub fn ring_homomorphisms(&self) -> Vec<&'static str> {
        vec![
            "同态定义: f: R → S，满足 f(a+b) = f(a)+f(b) 和 f(ab) = f(a)f(b)",
            "核: Ker(f) = {r ∈ R | f(r) = 0}，是 R 的理想",
            "像: Im(f) 是 S 的子环",
            "单射同态: Ker(f) = {0}",
            "满射同态: Im(f) = S",
            "同构: 既是单射又是满射的同态",
            "嵌入: 单射同态将 R 嵌入 S",
            "自然投影: R → R/I 的同态",
        ]
    }

    /// 环的性质
    pub fn ring_properties(&self) -> Vec<&'static str> {
        vec![
            "特征: 使 n·1 = 0 的最小正整数 n",
            "零特征: 特征为 0 的环",
            "特征 p: p·1 = 0，p 是素数",
            "唯一因子分解环 (UFD): 每个元素可唯一分解",
            "主理想环 (PID): 所有理想都是主理想",
            "诺特环: 每个理想都可有限生成",
            "阿廷环: 降链条件成立",
            "正规环: 整闭的整环",
        ]
    }

    /// 中国剩余定理
    pub fn chinese_remainder_theorem(&self) -> Vec<&'static str> {
        vec![
            "CRT 定理: I ∩ J = I·J 时，R/(I·J) ≅ R/I × R/J",
            "一般形式: 多个互素理想 I₁, ..., Iₙ",
            "互素定义: I + J = R",
            "解方程: 在每个商环中独立求解",
            "整数环: Z/(mn) ≅ Z/m × Z/n，m, n 互素",
            "多项式环: 不同多项式的 CRT",
            "应用: 模运算简化",
            "推广: 非交换环的 CRT",
        ]
    }

    /// 应用实例
    pub fn applications(&self) -> Vec<&'static str> {
        vec![
            "整数理论: 整数的因子分解",
            "多项式理论: 多项式环的研究",
            "代数几何: 琯理想与几何对象",
            "编码理论: 环上的纠错码",
            "密码学: 环上的密码系统",
            "数论: 环论在数论中的应用",
            "量子计算: 量子环代数",
            "信号处理: 环上的滤波器设计",
        ]
    }
}

impl Rule for RingTheoryRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("ring_theory")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "环论规则",
            &[
                ("环的基本定义", &self.ring_definition()),
                ("特殊环", &self.special_rings()),
                ("理想", &self.ideals()),
                ("商环", &self.quotient_rings()),
                ("理想与商环关系", &self.ideal_quotient_relations()),
                ("环同态", &self.ring_homomorphisms()),
                ("环的性质", &self.ring_properties()),
                ("中国剩余定理", &self.chinese_remainder_theorem()),
                ("应用实例", &self.applications()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ring_theory_rules() {
        let rules = RingTheoryRules::new();
        assert_eq!(rules.metadata().name, "环论规则");
        assert!(!rules.explain().is_empty());
        assert!(!rules.ring_definition().is_empty());
        assert!(!rules.ideals().is_empty());
        assert!(!rules.quotient_rings().is_empty());
    }
}