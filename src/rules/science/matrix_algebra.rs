//! 矩阵代数规则
//!
//! 矩阵代数研究矩阵的运算、性质和应用，是线性代数的核心内容。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: MatrixAlgebraRules,
    name: "矩阵代数规则",
    desc: "矩阵运算、性质与分解方法",
    origin: "数学",
    tags: ["科学", "数学", "代数", "线性代数"]
}

impl MatrixAlgebraRules {
    /// 矩阵定义
    pub fn matrix_definition(&self) -> Vec<&'static str> {
        vec![
            "矩阵定义: m×n 矩阵 A = (aᵢⱼ)",
            "元素: aᵢⱼ 是第 i 行第 j 列的元素",
            "方阵: m = n 的矩阵",
            "行向量: 1×n 矩阵",
            "列向量: m×1 矩阵",
            "零矩阵: 所有元素为零",
            "单位矩阵: 对角元素为 1，其余为 0",
            "矩阵大小: 行数 × 列数",
        ]
    }

    /// 矩阵运算
    pub fn matrix_operations(&self) -> Vec<&'static str> {
        vec![
            "矩阵加法: 对应元素相加",
            "矩阵减法: 对应元素相减",
            "标量乘法: 每个元素乘标量",
            "矩阵乘法: (AB)ᵢⱼ = Σₖ AᵢₖBₖⱼ",
            "乘法条件: A 的列数 = B 的行数",
            "矩阵幂: Aⁿ = A·A·...·A",
            "转置: Aᵀ，交换行列",
            "共轭转置: A* = (A̅)ᵀ",
        ]
    }

    /// 特殊矩阵
    pub fn special_matrices(&self) -> Vec<&'static str> {
        vec![
            "对角矩阵: 只有对角元素非零",
            "上三角矩阵: 对角线下元素为零",
            "下三角矩阵: 对角线上元素为零",
            "对称矩阵: A = Aᵀ",
            "反对称矩阵: A = -Aᵀ",
            "正交矩阵: AᵀA = I",
            "Hermitian 矩阵: A = A*",
            "正定矩阵: 所有特征值 > 0",
        ]
    }

    /// 行列式
    pub fn determinants(&self) -> Vec<&'static str> {
        vec![
            "行列式定义: 方阵的标量函数 det(A)",
            "行列式计算: 递归展开、LU 分解",
            "行列式性质: det(AB) = det(A)det(B)",
            "行列式与逆: det(A) ≠ 0 时 A 可逆",
            "行列式与体积: |det(A)| 是变换体积因子",
            "行列式展开: 沿某行或列展开",
            "行列式为零: 矩阵奇异",
            "行列式符号: 行列置换改变符号",
        ]
    }

    /// 矩阵逆
    pub fn matrix_inverse(&self) -> Vec<&'static str> {
        vec![
            "逆矩阵定义: AB = BA = I 时 B = A⁻¹",
            "可逆条件: det(A) ≠ 0",
            "逆矩阵公式: A⁻¹ = adj(A)/det(A)",
            "adj(A): 伴随矩阵",
            "逆矩阵性质: (AB)⁻¹ = B⁻¹A⁻¹",
            "逆矩阵计算: Gauss 消元法",
            "伪逆: 不可逆矩阵的广义逆",
            "逆矩阵应用: 解线性方程组",
        ]
    }

    /// 特征值与特征向量
    pub fn eigenvalues_eigenvectors(&self) -> Vec<&'static str> {
        vec![
            "定义: Av = λv，v ≠ 0",
            "特征值: λ 是标量",
            "特征向量: v 是向量",
            "特征方程: det(A - λI) = 0",
            "特征多项式: det(A - λI)",
            "特征值个数: n×n 矩阵最多有 n 个特征值",
            "特征空间: 特征向量生成的子空间",
            "谱定理: 对称矩阵的特征值是实数",
        ]
    }

    /// 矩阵分解
    pub fn matrix_decompositions(&self) -> Vec<&'static str> {
        vec![
            "LU 分解: A = LU，L 下三角，U 上三角",
            "QR 分解: A = QR，Q 正交，R 上三角",
            "奇异值分解: A = UΣVᵀ",
            "Cholesky 分解: A = LLᵀ，A 正定对称",
            "Schur 分解: A = UTUᵀ，T 上三角",
            "Jordan 分解: Jordan 标准形",
            "对角化: A = PDP⁻¹，D 对角",
            "分解应用: 解方程、计算特征值",
        ]
    }

    /// 矩阵秩
    pub fn matrix_rank(&self) -> Vec<&'static str> {
        vec![
            "秩定义: 线性无关的行或列的最大数",
            "行秩: 线性无关行的个数",
            "列秩: 线性无关列的个数",
            "秩定理: 行秩 = 列秩",
            "满秩:秩等于行数或列数",
            "秩与逆: 满秩方阵可逆",
            "秩与行列式: 满秩方阵行列式非零",
            "秩与方程: 秩决定方程解的性质",
        ]
    }

    /// 应用实例
    pub fn applications(&self) -> Vec<&'static str> {
        vec![
            "线性方程组: Ax = b",
            "图像处理: 图像的矩阵表示",
            "机器学习: 数据矩阵",
            "物理学: 物理系统的矩阵描述",
            "工程学: 系统状态方程",
            "经济学: 经济模型矩阵",
            "计算机图形: 变换矩阵",
            "信号处理: 信号矩阵分析",
        ]
    }
}

impl Rule for MatrixAlgebraRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("matrix_algebra")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "矩阵代数规则",
            &[
                ("矩阵定义", &self.matrix_definition()),
                ("矩阵运算", &self.matrix_operations()),
                ("特殊矩阵", &self.special_matrices()),
                ("行列式", &self.determinants()),
                ("矩阵逆", &self.matrix_inverse()),
                ("特征值与特征向量", &self.eigenvalues_eigenvectors()),
                ("矩阵分解", &self.matrix_decompositions()),
                ("矩阵秩", &self.matrix_rank()),
                ("应用实例", &self.applications()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_algebra_rules() {
        let rules = MatrixAlgebraRules::new();
        assert_eq!(rules.metadata().name, "矩阵代数规则");
        assert!(!rules.explain().is_empty());
        assert!(!rules.matrix_definition().is_empty());
        assert!(!rules.eigenvalues_eigenvectors().is_empty());
        assert!(!rules.matrix_decompositions().is_empty());
    }
}