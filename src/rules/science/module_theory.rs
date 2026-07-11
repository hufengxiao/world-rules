//! 模论规则
//!
//! 模论研究模结构，是环上向量空间的推广，是抽象代数的重要内容。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: ModuleTheoryRules,
    name: "模论规则",
    desc: "模的定义、性质与分类",
    origin: "数学",
    tags: ["科学", "数学", "代数", "抽象代数"]
}

impl ModuleTheoryRules {
    /// 模的定义
    pub fn module_definition(&self) -> Vec<&'static str> {
        vec![
            "模定义: 集合 M 配合环 R 的运算",
            "模加法: M 对加法构成阿贝尔群",
            "标量乘法: r·m ∈ M，r ∈ R，m ∈ M",
            "加法封闭: m₁ + m₂ ∈ M",
            "标量封闭: r·m ∈ M",
            "模性质: r(m₁ + m₂) = rm₁ + rm₂",
            "模性质: (r₁ + r₂)m = r₁m + r₂m",
            "模性质: (r₁r₂)m = r₁(r₂m)",
        ]
    }

    /// 向量空间与模
    pub fn modules_vs_vector_spaces(&self) -> Vec<&'static str> {
        vec![
            "向量空间: 域上的模",
            "模推广: 环上的向量空间推广",
            "域特点: 每个非零元素可逆",
            "环特点: 可能无逆元",
            "模复杂性: 模比向量空间复杂",
            "基底: 模可能无基底",
            "子模: 模的子结构",
            "自由模: 有基底的模",
        ]
    }

    /// 子模
    pub fn submodules(&self) -> Vec<&'static str> {
        vec![
            "子模定义: M 的子集 N，自身构成模",
            "子模判定: 非空、加法封闭、标量乘法封闭",
            "零模: {0} 是最小子模",
            "全模: M 是最大子模",
            "子模和: N₁ + N₂",
            "子模交: N₁ ∩ N₂",
            "直和: N₁ ⊕ N₂，交集为零",
            "子模生成: 由元素集合生成的子模",
        ]
    }

    /// 商模
    pub fn quotient_modules(&self) -> Vec<&'static str> {
        vec![
            "商模定义: M/N，N 是子模",
            "商模元素: N 的陪集 {m + N | m ∈ M}",
            "商模运算: (m₁ + N) + (m₂ + N) = (m₁ + m₂) + N",
            "商模标量: r(m + N) = rm + N",
            "商模意义: 对模进行简化",
            "同构定理: M/K ≅ N",
            "第一同构定理: M/Ker(f) ≅ Im(f)",
            "第二同构定理: (M/N)/(K/N) ≅ M/K",
        ]
    }

    /// 模同态
    pub fn module_homomorphisms(&self) -> Vec<&'static str> {
        vec![
            "同态定义: f: M → N，f(m₁ + m₂) = f(m₁) + f(m₂) 且 f(rm) = rf(m)",
            "核: Ker(f) = {m | f(m) = 0}",
            "像: Im(f) = {f(m) | m ∈ M}",
            "单射同态: Ker(f) = {0}",
            "满射同态: Im(f) = N",
            "同构: 既是单射又是满射",
            "同态核定理: Ker(f) 是子模",
            "同态像定理: Im(f) 是子模",
        ]
    }

    /// 自由模
    pub fn free_modules(&self) -> Vec<&'static str> {
        vec![
            "自由模定义: 有基底的模",
            "基底: 生成模且线性无关的元素集合",
            "自由模结构: Rⁿ = R ⊕ R ⊕ ... ⊕ R",
            "自由模同构: Rⁿ ≅ Rᵐ iff n = m",
            "自由模性质: 有基底便于计算",
            "自由模个数: 基底的个数固定",
            "自由模与向量空间: 类似于向量空间",
            "非自由模: 没有基底的模",
        ]
    }

    /// 模分解
    pub fn module_decomposition(&self) -> Vec<&'static str> {
        vec![
            "分解: 将模分解为简单模块的直和",
            "直和: M = N₁ ⊕ N₂ ⊕ ... ⊕ Nₖ",
            "不可分解模: 不能再分解的模",
            "简单模: 只有零子模和自身",
            "Jordan-Hölder: 简单模块的唯一性",
            "分解存在: 模的分解可能不存在",
            "有限生成模: 可以有限生成的模",
            "有限生成性: 生成模的元素个数有限",
        ]
    }

    /// 诺特模
    pub fn noetherian_modules(&self) -> Vec<&'static str> {
        vec![
            "诺特模定义: 每个子模都可有限生成",
            "升链条件: 子模升链稳定",
            "诺特环: 诺特模上的环",
            "诺特性质: 诺特模的良好性质",
            "有限生成: 诺特模的子模有限生成",
            "分解定理: 诺特模有分解",
            "诺特判定: 模是诺特的判定条件",
            "诺特应用: 诺特模的应用",
        ]
    }

    /// 模的应用
    pub fn module_applications(&self) -> Vec<&'static str> {
        vec![
            "代数几何: 模与几何对象",
            "代数拓扑: 模与拓扑结构",
            "群表示: 群在模上的表示",
            "线性代数: 向量空间是域上的模",
            "多项式环: 多项式环上的模",
            "模论重要性: 抽象代数的核心",
            "同调代数: 模的同调性质",
            "代数结构: 模作为代数结构",
        ]
    }
}

impl Rule for ModuleTheoryRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("module_theory")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "模论规则",
            &[
                ("模的定义", &self.module_definition()),
                ("向量空间与模", &self.modules_vs_vector_spaces()),
                ("子模", &self.submodules()),
                ("商模", &self.quotient_modules()),
                ("模同态", &self.module_homomorphisms()),
                ("自由模", &self.free_modules()),
                ("模分解", &self.module_decomposition()),
                ("诺特模", &self.noetherian_modules()),
                ("模的应用", &self.module_applications()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_theory_rules() {
        let rules = ModuleTheoryRules::new();
        assert_eq!(rules.metadata().name, "模论规则");
        assert!(!rules.explain().is_empty());
        assert!(!rules.module_definition().is_empty());
        assert!(!rules.submodules().is_empty());
        assert!(!rules.quotient_modules().is_empty());
    }
}