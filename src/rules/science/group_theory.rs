//! 群论规则
//!
//! 群论是研究群结构的代数学分支，是抽象代数的基础。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: GroupTheoryRules,
    name: "群论规则",
    desc: "群的定义、性质与重要定理",
    origin: "数学",
    tags: ["科学", "数学", "代数", "群论"]
}

impl GroupTheoryRules {
    /// 群的基本定义
    pub fn group_definition(&self) -> Vec<&'static str> {
        vec![
            "群定义: 集合 G 配合运算 *，满足封闭性、结合律、有单位元、有逆元",
            "封闭性: 对任意 a, b ∈ G，有 a * b ∈ G",
            "结合律: 对任意 a, b, c ∈ G，有 (a * b) * c = a * (b * c)",
            "单位元: 存在 e ∈ G，使任意 a ∈ G 有 e * a = a * e = a",
            "逆元: 对任意 a ∈ G，存在 a⁻¹ ∈ G，使 a * a⁻¹ = a⁻¹ * a = e",
            "群的阶: 群中元素个数 |G|，有限群或无限群",
            "阿贝尔群: 运算满足交换律的群，a * b = b * a",
            "非阿贝尔群: 运算不满足交换律的群，如矩阵群",
        ]
    }

    /// 子群与陪集
    pub fn subgroups_cosets(&self) -> Vec<&'static str> {
        vec![
            "子群定义: H 是 G 的子集，且 H 对 G 的运算也构成群",
            "子群判定: H ⊆ G，H 非空，a, b ∈ H ⇒ a * b⁻¹ ∈ H",
            "陪集定义: 对 a ∈ G，aH = {ah | h ∈ H} 为左陪集",
            "右陪集: Ha = {ha | h ∈ H}",
            "陪集性质: 两个陪集或相等或不相交",
            "拉格朗日定理: |H| 整除 |G|，子群阶整除群阶",
            "指数定义: |G|/|H| = [G:H] 为子群 H 在 G 中的指数",
            "正规子群: 对任意 g ∈ G，有 gH = Hg",
        ]
    }

    /// 群同态与同构
    pub fn homomorphisms(&self) -> Vec<&'static str> {
        vec![
            "同态定义: 映射 f: G → H，使 f(ab) = f(a)f(b)",
            "同态核: Ker(f) = {g ∈ G | f(g) = e_H}，是 G 的正规子群",
            "同态像: Im(f) = {f(g) | g ∈ G}，是 H 的子群",
            "同态基本定理: G/Ker(f) ≅ Im(f)",
            "同构定义: 既是同态又是双射",
            "同构意义: 两群结构完全相同，仅元素表示不同",
            "自同构: G 到自身的同构",
            "内自同构: 由群元素定义的自同构 φ_g(x) = gxg⁻¹",
        ]
    }

    /// 循环群
    pub fn cyclic_groups(&self) -> Vec<&'static str> {
        vec![
            "循环群定义: 由一个元素生成的群 G = ⟨a⟩",
            "生成元: a 是 G 的生成元，G = {aⁿ | n ∈ Z}",
            "无限循环群: ⟨a⟩ 无限时同构于 Z（整数加群）",
            "有限循环群: ⟨a⟩ 有限时同构于 Z_n",
            "循环群阶: 有限循环群 ⟨a⟩ 的阶为 a 的阶",
            "元素阶: aⁿ = e 的最小正整数 n",
            "循环群性质: 循环群必是阿贝尔群",
            "子群性质: 循环群的每个子群都是循环群",
        ]
    }

    /// 置换群
    pub fn permutation_groups(&self) -> Vec<&'static str> {
        vec![
            "置换定义: 有限集到自身的双射",
            "对称群 S_n: n 个元素的所有置换构成的群",
            "对称群阶: |S_n| = n!",
            "轮换表示: 置换写成轮换的乘积",
            "对换: 只交换两个元素的置换",
            "奇偶性: 置换可表示为偶数或奇数个对换的乘积",
            "交错群 A_n: 所有偶置换构成的群",
            "交错群阶: |A_n| = n!/2",
        ]
    }

    /// 群作用
    pub fn group_actions(&self) -> Vec<&'static str> {
        vec![
            "群作用定义: 群 G 在集合 X 上的作用满足恒等性和结合性",
            "轨道定义: 元素 x 的轨道 Orbit(x) = {g·x | g ∈ G}",
            "稳定子: Stab(x) = {g ∈ G | g·x = x}",
            "轨道-稳定子定理: |Orbit(x)| = |G|/|Stab(x)|",
            "轨道分解: 集合 X 分解为不相交轨道的并",
            "传递作用: 只有一个轨道的群作用",
            "忠实作用: 不同群元素有不同作用",
            "Burnside 引理: 不动点个数的平均值",
        ]
    }

    /// 商群
    pub fn quotient_groups(&self) -> Vec<&'static str> {
        vec![
            "商群定义: G/N，其中 N 是 G 的正规子群",
            "商群元素: N 的陪集 {gN | g ∈ G}",
            "商群运算: (aN)(bN) = (ab)N",
            "商群阶: |G/N| = |G|/|N|",
            "商群意义: 对群进行简化，保留部分结构",
            "单群定义: 只有平凡正规子群的群",
            "可解群: 商群列直到平凡群都是阿贝尔群",
            "群扩张: 从商群和子群重构原群",
        ]
    }

    /// 重要定理
    pub fn important_theorems(&self) -> Vec<&'static str> {
        vec![
            "拉格朗日定理: 子群阶整除群阶",
            "同态基本定理: G/Ker(f) ≅ Im(f)",
            "西罗定理: 有限群的 p-子群存在性",
            "Cayley 定理: 每个群同构于某个对称群的子群",
            "轨道-稳定子定理: |Orbit(x)|·|Stab(x)| = |G|",
            "第一同构定理: G/(N∩K) ≅ NK/N",
            "第二同构定理: G/K ≅ (G/N)/(K/N)",
            "第三同构定理: 商群的商群同构于更大的商群",
        ]
    }

    /// 应用实例
    pub fn applications(&self) -> Vec<&'static str> {
        vec![
            "对称性研究: 几何对称、物理对称",
            "晶体结构: 空间群描述晶体对称性",
            "密码学: RSA、椭圆曲线密码",
            "量子力学: 群表示论描述粒子对称性",
            "伽罗瓦理论: 群论与方程可解性",
            "组合数学: Burnside 引理计数",
            "编码理论: 纠错码的群结构",
            "分子化学: 分子对称性分类",
        ]
    }
}

impl Rule for GroupTheoryRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("group_theory")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "群论规则",
            &[
                ("群的基本定义", &self.group_definition()),
                ("子群与陪集", &self.subgroups_cosets()),
                ("群同态与同构", &self.homomorphisms()),
                ("循环群", &self.cyclic_groups()),
                ("置换群", &self.permutation_groups()),
                ("群作用", &self.group_actions()),
                ("商群", &self.quotient_groups()),
                ("重要定理", &self.important_theorems()),
                ("应用实例", &self.applications()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_theory_rules() {
        let rules = GroupTheoryRules::new();
        assert_eq!(rules.metadata().name, "群论规则");
        assert!(!rules.explain().is_empty());
        assert!(!rules.group_definition().is_empty());
        assert!(!rules.subgroups_cosets().is_empty());
        assert!(!rules.homomorphisms().is_empty());
    }
}
