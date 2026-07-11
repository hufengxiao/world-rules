//! 双线性代数规则
//!
//! 双线性代数研究双线性映射、二次型和内积，是线性代数的深化内容。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: BilinearAlgebraRules,
    name: "双线性代数规则",
    desc: "双线性形式、二次型与内积空间",
    origin: "数学",
    tags: ["科学", "数学", "代数", "线性代数"]
}

impl BilinearAlgebraRules {
    /// 双线性形式定义
    pub fn bilinear_forms(&self) -> Vec<&'static str> {
        vec![
            "双线性形式定义: B: V×V → F，对每个变量线性",
            "线性性: B(αu+βv, w) = αB(u,w) + βB(v,w)",
            "线性性2: B(u, αv+βw) = αB(u,v) + βB(u,w)",
            "对称双线性: B(u,v) = B(v,u)",
            "反对称双线性: B(u,v) = -B(v,u)",
            "非退化: B(u,v) = 0 ∀v ⇒ u = 0",
            "矩阵表示: B(u,v) = uᵀAv",
            "矩阵对称: A = Aᵀ",
        ]
    }

    /// 二次型
    pub fn quadratic_forms(&self) -> Vec<&'static str> {
        vec![
            "二次型定义: Q(v) = B(v,v)",
            "二次型性质: Q(αv) = α²Q(v)",
            "极化恒等式: Q(u+v) - Q(u) - Q(v) = 2B(u,v)",
            "二次型矩阵: Q(v) = vᵀAv",
            "二次型标准化: 通过坐标变换",
            "正二次型: Q(v) > 0 ∀v ≠ 0",
            "负二次型: Q(v) < 0 ∀v ≠ 0",
            "不定二次型: 可正可负",
        ]
    }

    /// 内积空间
    pub fn inner_product_spaces(&self) -> Vec<&'static str> {
        vec![
            "内积定义: ⟨u,v⟩ 满足正性、对称性、线性性",
            "正性: ⟨v,v⟩ ≥ 0，⟨v,v⟩ = 0 iff v = 0",
            "对称性: ⟨u,v⟩ = ⟨v,u⟩（实）或 ⟨v,u⟩（复）",
            "线性性: ⟨αu+βv,w⟩ = α⟨u,w⟩ + β⟨v,w⟩",
            "范数: ||v|| = √⟨v,v⟩",
            "正交: ⟨u,v⟩ = 0",
            "正交补: W⊥",
            "Hilbert 空间: 完备的内积空间",
        ]
    }

    /// 正交性
    pub fn orthogonality(&self) -> Vec<&'static str> {
        vec![
            "正交定义: ⟨u,v⟩ = 0",
            "正交向量: 两个向量正交",
            "正交集合: 集合中向量相互正交",
            "正交基: 基向量相互正交",
            "标准正交基: 正交且范数为 1",
            "Gram-Schmidt: 构造正交基的方法",
            "正交补: W⊥ = {v | ⟨v,w⟩ = 0 ∀w ∈ W}",
            "直和分解: V = W ⊕ W⊥",
        ]
    }

    /// 正交变换
    pub fn orthogonal_transformations(&self) -> Vec<&'static str> {
        vec![
            "正交变换定义: ⟨T(u),T(v)⟩ = ⟨u,v⟩",
            "保范数: ||T(v)|| = ||v||",
            "保正交: T 保持正交关系",
            "正交矩阵: T 的矩阵 Q 满足 QᵀQ = I",
            "行列式: |det(Q)| = 1",
            "旋转: det(Q) = 1 的正交变换",
            "反射: det(Q) = -1 的正交变换",
            "应用: 图形变换、坐标系变换",
        ]
    }

    /// 二次型分类
    pub fn quadratic_form_classification(&self) -> Vec<&'static str> {
        vec![
            "惯性定理: 二次型的标准形有固定形式",
            "正惯性指数: 正系数个数",
            "负惯性指数: 负系数个数",
            "符号差: 正惯性指数 - 负惯性指数",
            "秩: 非零系数个数",
            "合同: A 和 B 合同 iff 有相同惯性指数",
            "正定判定: 所有特征值 > 0",
            "半正定: 所有特征值 ≥ 0",
        ]
    }

    /// Gram 矩阵
    pub fn gram_matrix(&self) -> Vec<&'static str> {
        vec![
            "Gram 矩阵定义: G = (⟨vᵢ,vⱼ⟩)",
            "Gram 矩阵性质: 对称、正半定",
            "Gram 行列式: 向量无关判定",
            "Gram 行列式为零: 向量相关",
            "Gram 行列式非零: 向量无关",
            "体积: Gram 行列式的平方根",
            "Gram-Schmidt: Gram 矩阵在正交化中的应用",
            "Gram 矩阵应用: 最小二乘",
        ]
    }

    /// 双线性形式分类
    pub fn bilinear_form_classification(&self) -> Vec<&'static str> {
        vec![
            "对称形式: B(u,v) = B(v,u)",
            "反对称形式: B(u,v) = -B(v,u)",
            "交替形式: B(v,v) = 0",
            "退化形式: 存在 u ≠ 0 使 B(u,v) = 0 ∀v",
            "非退化形式: 无退化",
            "合同分类: 通过坐标变换分类",
            "秩: 非退化部分的维数",
            "标准形: 分类的标准形",
        ]
    }

    /// 应用实例
    pub fn applications(&self) -> Vec<&'static str> {
        vec![
            "优化: 二次型优化",
            "物理学: 能量的二次形式",
            "统计学: 二次型统计量",
            "信号处理: 内积相关",
            "机器学习: 内积核方法",
            "几何: 二次曲面",
            "数值分析: Gram-Schmidt 正交化",
            "量子力学: Hilbert 空间内积",
        ]
    }
}

impl Rule for BilinearAlgebraRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("bilinear_algebra")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "双线性代数规则",
            &[
                ("双线性形式", &self.bilinear_forms()),
                ("二次型", &self.quadratic_forms()),
                ("内积空间", &self.inner_product_spaces()),
                ("正交性", &self.orthogonality()),
                ("正交变换", &self.orthogonal_transformations()),
                ("二次型分类", &self.quadratic_form_classification()),
                ("Gram 矩阵", &self.gram_matrix()),
                ("双线性形式分类", &self.bilinear_form_classification()),
                ("应用实例", &self.applications()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bilinear_algebra_rules() {
        let rules = BilinearAlgebraRules::new();
        assert_eq!(rules.metadata().name, "双线性代数规则");
        assert!(!rules.explain().is_empty());
        assert!(!rules.bilinear_forms().is_empty());
        assert!(!rules.quadratic_forms().is_empty());
        assert!(!rules.orthogonality().is_empty());
    }
}