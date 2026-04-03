//! 数学规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 数学公式/定理类型
#[derive(Debug, Clone)]
pub enum MathTheoremType {
    /// 欧几里得几何公理
    EuclidAxioms,
    /// 毕达哥拉斯定理 (勾股定理)
    PythagoreanTheorem,
    /// 圆的面积公式
    CircleArea,
    /// 二次方程求根公式
    QuadraticFormula,
    /// 欧拉公式
    EulerFormula,
    /// 斐波那契数列
    FibonacciSequence,
    /// 正弦定理
    SineLaw,
    /// 余弦定理
    CosineLaw,
}

impl MathTheoremType {
    pub fn name(&self) -> &'static str {
        match self {
            MathTheoremType::EuclidAxioms => "欧几里得几何公理",
            MathTheoremType::PythagoreanTheorem => "勾股定理",
            MathTheoremType::CircleArea => "圆的面积公式",
            MathTheoremType::QuadraticFormula => "二次方程求根公式",
            MathTheoremType::EulerFormula => "欧拉公式",
            MathTheoremType::FibonacciSequence => "斐波那契数列",
            MathTheoremType::SineLaw => "正弦定理",
            MathTheoremType::CosineLaw => "余弦定理",
        }
    }

    pub fn formula(&self) -> &'static str {
        match self {
            MathTheoremType::EuclidAxioms => "五大公理体系",
            MathTheoremType::PythagoreanTheorem => "a² + b² = c²",
            MathTheoremType::CircleArea => "S = πr²",
            MathTheoremType::QuadraticFormula => "x = (-b ± √(b²-4ac)) / 2a",
            MathTheoremType::EulerFormula => "e^(iπ) + 1 = 0",
            MathTheoremType::FibonacciSequence => "Fₙ = Fₙ₋₁ + Fₙ₋₂",
            MathTheoremType::SineLaw => "a/sinA = b/sinB = c/sinC",
            MathTheoremType::CosineLaw => "c² = a² + b² - 2ab·cosC",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            MathTheoremType::EuclidAxioms =>
                "平面几何的基本公理体系，包括点、线、面的定义和基本性质",
            MathTheoremType::PythagoreanTheorem =>
                "直角三角形两直角边的平方和等于斜边的平方",
            MathTheoremType::CircleArea =>
                "圆的面积等于半径平方乘以圆周率",
            MathTheoremType::QuadraticFormula =>
                "一元二次方程 ax²+bx+c=0 的求根公式",
            MathTheoremType::EulerFormula =>
                "连接指数函数、三角函数和复数的最美公式",
            MathTheoremType::FibonacciSequence =>
                "每个数等于前两个数之和: 1,1,2,3,5,8,13...",
            MathTheoremType::SineLaw =>
                "三角形边长与对应角的正弦值比值相等",
            MathTheoremType::CosineLaw =>
                "三角形任意一边的平方等于其他两边平方和减去两倍夹角余弦乘积",
        }
    }
}

/// 数学规则集合
pub struct MathRules {
    metadata: RuleMetadata,
}

impl MathRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "数学公式与定理",
                "基础数学公式和定理"
            )
            .with_origin("数学")
            .with_tags(vec!["科学".into(), "数学".into()]),
        }
    }

    pub fn all_theorems() -> Vec<MathTheoremType> {
        vec![
            MathTheoremType::EuclidAxioms,
            MathTheoremType::PythagoreanTheorem,
            MathTheoremType::CircleArea,
            MathTheoremType::QuadraticFormula,
            MathTheoremType::EulerFormula,
            MathTheoremType::FibonacciSequence,
            MathTheoremType::SineLaw,
            MathTheoremType::CosineLaw,
        ]
    }

    /// 计算勾股定理
    pub fn pythagorean(a: f64, b: f64) -> f64 {
        (a * a + b * b).sqrt()
    }

    /// 计算圆面积
    pub fn circle_area(radius: f64) -> f64 {
        std::f64::consts::PI * radius * radius
    }

    /// 解二次方程
    pub fn solve_quadratic(a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrt_d = discriminant.sqrt();
        Some((
            (-b + sqrt_d) / (2.0 * a),
            (-b - sqrt_d) / (2.0 * a),
        ))
    }

    /// 生成斐波那契数列
    pub fn fibonacci(n: usize) -> Vec<u64> {
        if n == 0 {
            return vec![];
        }
        if n == 1 {
            return vec![1];
        }
        let mut seq = vec![1, 1];
        for i in 2..n {
            seq.push(seq[i-1] + seq[i-2]);
        }
        seq
    }
}

impl Default for MathRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for MathRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("math")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        let theorems = Self::all_theorems();
        format!(
            "【数学公式与定理】\n\n{}\n",
            theorems.iter()
                .map(|t| format!(
                    "▶ {}\n   公式: {}\n   说明: {}\n",
                    t.name(),
                    t.formula(),
                    t.description()
                ))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pythagorean() {
        let c = MathRules::pythagorean(3.0, 4.0);
        assert_eq!(c, 5.0);
    }

    #[test]
    fn test_fibonacci() {
        let seq = MathRules::fibonacci(10);
        assert_eq!(seq, vec![1, 1, 2, 3, 5, 8, 13, 21, 34, 55]);
    }
}