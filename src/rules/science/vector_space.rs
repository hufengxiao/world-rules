//! 向量空间规则
//!
//! 向量空间是线性代数的基础结构，广泛应用于数学和工程领域。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: VectorSpaceRules,
    name: "向量空间规则",
    desc: "向量空间的定义、性质与运算",
    origin: "数学",
    tags: ["科学", "数学", "代数", "线性代数"]
}

impl VectorSpaceRules {
    /// 向量空间定义
    pub fn vector_space_definition(&self) -> Vec<&'static str> {
        vec![
            "向量空间定义: 集合 V 配合向量加法和标量乘法",
            "向量加法: V 中两个向量的运算，结果仍在 V 中",
            "标量乘法: 标量与向量的运算，结果在 V 中",
            "零向量: 加法的单位元 0",
            "加法逆元: 每个向量有相反向量",
            "标量域: 通常为实数域 R 或复数域 C",
            "维度: 向量空间的维数",
            "向量: 向量空间的元素",
        ]
    }

    /// 向量空间性质
    pub fn vector_space_properties(&self) -> Vec<&'static str> {
        vec![
            "加法封闭性: u + v ∈ V",
            "标量封闭性: αv ∈ V",
            "加法交换律: u + v = v + u",
            "加法结合律: (u + v) + w = u + (v + w)",
            "标量分配律: α(u + v) = αu + αv",
            "向量分配律: (α + β)u = αu + βu",
            "标量结合律: α(βu) = (αβ)u",
            "幺元性质: 1u = u",
        ]
    }

    /// 子空间
    pub fn subspaces(&self) -> Vec<&'static str> {
        vec![
            "子空间定义: V 的子集 W，自身构成向量空间",
            "子空间判定: 非空、加法封闭、标量乘法封闭",
            "零空间: {0} 是最小子空间",
            "全空间: V 是最大子空间",
            "真子空间: 非零非全的子空间",
            "子空间和: W₁ + W₂",
            "直和: W₁ ⊕ W₂，交集为零",
            "子空间交: W₁ ∩ W₂",
        ]
    }

    /// 生成与基
    pub fn span_and_basis(&self) -> Vec<&'static str> {
        vec![
            "生成: 向量集合生成的子空间",
            "线性组合: α₁v₁ + ... + αₙvₙ",
            "生成集: 能生成 V 的向量集合",
            "基定义: 既生成又线性无关的向量集合",
            "标准基: {(1,0,...), (0,1,...), ...}",
            "维数定理: 任意两个基的大小相同",
            "维数: 基的大小 = dim(V)",
            "坐标: 在基下的表示",
        ]
    }

    /// 线性无关
    pub fn linear_independence(&self) -> Vec<&'static str> {
        vec![
            "线性无关: α₁v₁ + ... + αₙvₙ = 0 ⇒ α₁ = ... = αₙ = 0",
            "线性相关: 存在非零系数使线性组合为零",
            "无关判定: 向量集合是否无关",
            "秩: 无关向量的最大个数",
            "基判定: 无关且生成",
            "扩充: 无关集可扩充为基",
            "缩减: 相关集可缩减为基",
            "向量个数: 多于维数必相关",
        ]
    }

    /// 线性变换
    pub fn linear_transformations(&self) -> Vec<&'static str> {
        vec![
            "线性映射定义: T: V → W，保持线性运算",
            "加法保持: T(u + v) = T(u) + T(v)",
            "标量保持: T(αu) = αT(u)",
            "核: Ker(T) = {v | T(v) = 0}",
            "像: Im(T) = {T(v) | v ∈ V}",
            "核-像定理: dim(Ker(T)) + dim(Im(T)) = dim(V)",
            "秩: dim(Im(T))",
            "零度: dim(Ker(T))",
        ]
    }

    /// 矩阵表示
    pub fn matrix_representation(&self) -> Vec<&'static str> {
        vec![
            "矩阵: 线性变换的坐标表示",
            "基变换: 改变基改变矩阵",
            "矩阵运算: 对应线性变换的运算",
            "相似矩阵: 不同基下的表示",
            "矩阵元素: aᵢⱼ 表示基向量映射的坐标",
            "矩阵与变换: 一一对应（给定基）",
            "矩阵秩: 等于变换的秩",
            "可逆矩阵: 对应可逆变换",
        ]
    }

    /// 内积空间
    pub fn inner_product_spaces(&self) -> Vec<&'static str> {
        vec![
            "内积定义: ⟨u, v⟩ 满足正性、对称性、线性性",
            "内积空间: 配备内积的向量空间",
            "范数: ||v|| = √⟨v, v⟩",
            "距离: d(u, v) = ||u - v||",
            "正交: ⟨u, v⟩ = 0",
            "正交基: 基向量相互正交",
            "正交补: W⊥",
            "Gram-Schmidt: 构造正交基的过程",
        ]
    }

    /// 应用实例
    pub fn applications(&self) -> Vec<&'static str> {
        vec![
            "物理学: 物理量的向量表示",
            "工程学: 工程系统的状态向量",
            "计算机图形: 图形变换",
            "信号处理: 信号向量",
            "数据科学: 数据的向量表示",
            "机器学习: 特征向量",
            "量子力学: 状态向量",
            "控制系统: 系统状态向量",
        ]
    }
}

impl Rule for VectorSpaceRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("vector_space")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "向量空间规则",
            &[
                ("向量空间定义", &self.vector_space_definition()),
                ("向量空间性质", &self.vector_space_properties()),
                ("子空间", &self.subspaces()),
                ("生成与基", &self.span_and_basis()),
                ("线性无关", &self.linear_independence()),
                ("线性变换", &self.linear_transformations()),
                ("矩阵表示", &self.matrix_representation()),
                ("内积空间", &self.inner_product_spaces()),
                ("应用实例", &self.applications()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_space_rules() {
        let rules = VectorSpaceRules::new();
        assert_eq!(rules.metadata().name, "向量空间规则");
        assert!(!rules.explain().is_empty());
        assert!(!rules.vector_space_definition().is_empty());
        assert!(!rules.span_and_basis().is_empty());
        assert!(!rules.linear_transformations().is_empty());
    }
}