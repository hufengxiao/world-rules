//! 数值分析规则
//!
//! 数值分析研究数值计算方法和误差分析。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: NumericalAnalysisRules,
    name: "数值分析规则",
    desc: "数值方法、误差分析与算法设计",
    origin: "数学",
    tags: ["科学", "数学", "数值"]
}

impl NumericalAnalysisRules {
    /// 误差分析基础
    pub fn error_analysis(&self) -> Vec<&'static str> {
        vec![
            "绝对误差: ε = |x - x*|",
            "相对误差: ε_r = |x - x*|/|x|",
            "有效数字: 从第一个非零位开始",
            "舍入误差: 计算中的精度损失",
            "截断误差: 方法近似产生的误差",
            "误差传播: 计算过程中误差的累积",
            "数值稳定性: 误差不急剧增长",
            "机器精度: 计算机的精度限制",
        ]
    }

    /// 数值线性代数
    pub fn numerical_linear_algebra(&self) -> Vec<&'static str> {
        vec![
            "矩阵分解: LU、QR、Cholesky",
            "线性方程组: Ax = b 的求解",
            "Gauss 消元: 基本的求解方法",
            "迭代方法: Jacobi、Gauss-Seidel",
            "条件数: cond(A) = ||A||·||A^(-1)||",
            "奇异值分解 SVD: 矩阵的重要分解",
            "最小二乘法: 超定方程的求解",
            "稀疏矩阵: 特殊存储和算法",
        ]
    }

    /// 数值积分
    pub fn numerical_integration(&self) -> Vec<&'static str> {
        vec![
            "梯形公式: ∫f(x)dx ≈ h/2·(f₀ + f₁)",
            "Simpson 公式: 更精确的积分方法",
            "Newton-Cotes 公式: 等距节点的积分",
            "Gauss 积分: 最优节点选择",
            "复合积分公式: 提高精度的方法",
            "自适应积分: 根据误差调整",
            "积分误差估计: 误差界限",
            "多重积分: Monte Carlo 方法",
        ]
    }

    /// 数值微分
    pub fn numerical_differentiation(&self) -> Vec<&'static str> {
        vec![
            "前差公式: f'(x) ≈ (f(x+h) - f(x))/h",
            "后差公式: f'(x) ≈ (f(x) - f(x-h))/h",
            "中心差分: f'(x) ≈ (f(x+h) - f(x-h))/(2h)",
            "高阶导数: 多点差分公式",
            "Richardson 外推: 提高精度",
            "误差分析: 截断和舍入误差",
            "步长选择: 平衡两类误差",
            "应用: 求解微分方程",
        ]
    }

    /// 方程求根
    pub fn root_finding(&self) -> Vec<&'static str> {
        vec![
            "二分法: 在区间 [a,b] 中搜索",
            "Newton 法: x_{n+1} = x_n - f(x_n)/f'(x_n)",
            "割线法: 用差分代替导数",
            "迭代法: x_{n+1} = g(x_n)",
            "收敛性: 方法收敛的条件",
            "收敛速度: 线性、超线性、二次",
            "多根问题: 复数根的处理",
            "不动点方法: 求解 g(x) = x",
        ]
    }

    /// 数值求解ODE
    pub fn solving_odes(&self) -> Vec<&'static str> {
        vec![
            "Euler 方法: y_{n+1} = y_n + h·f(x_n,y_n)",
            "改进 Euler: 两步方法",
            "Runge-Kutta 法: RK4 最常用",
            "步长控制: 自适应步长",
            "稳定性: 数值方法的稳定性",
            "刚性方程: 需要特殊方法",
            "边值问题: 边界条件处理",
            "误差估计: 局部和全局误差",
        ]
    }

    /// 插值与拟合
    pub fn interpolation_and_fitting(&self) -> Vec<&'static str> {
        vec![
            "拉格朗日插值: 多项式插值",
            "Newton 插值: 逐步增加节点",
            "样条插值: 分段多项式",
            "三次样条: 最常用的样条",
            "最小二乘拟合: 数据拟合",
            "多项式拟合: 用多项式拟合",
            "插值误差: Runge 现象",
            "拟合质量: R²、残差分析",
        ]
    }

    /// 优化算法
    pub fn optimization_algorithms(&self) -> Vec<&'static str> {
        vec![
            "梯度下降: 沿负梯度方向",
            "Newton 优化: 二阶导数方法",
            "共轭梯度: 针对二次问题",
            "拟 Newton: BFGS、DFP",
            "信赖域方法: 区域内优化",
            "线搜索: 寻找最优步长",
            "全局优化: 避免局部最优",
            "约束优化: KKT 条件",
        ]
    }

    /// 特殊函数计算
    pub fn special_functions(&self) -> Vec<&'static str> {
        vec![
            "Gamma 函数: Γ(x) = ∫t^(x-1)e^(-t)dt",
            "Bessel 函数: 微分方程的解",
            "Legendre 函数: 球坐标系中的解",
            "Chebyshev 函数: 数值逼近",
            "椭圆函数: 特殊积分",
            "误差函数: erf(x) = 2/√π·∫e^(-t²)dt",
            "计算方法: 递推、级数",
            "应用: 物理和工程",
        ]
    }

    /// 应用领域
    pub fn applications(&self) -> Vec<&'static str> {
        vec![
            "科学计算: 物理模拟、天气预报",
            "工程设计: CAD、CAM",
            "金融计算: 期权定价、风险评估",
            "图像处理: 图像重建、增强",
            "机器学习: 参数优化、训练",
            "数据分析: 统计计算",
            "信号处理: 滤波、变换",
            "生物科学: 分子模拟",
        ]
    }
}

impl Rule for NumericalAnalysisRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("numerical_analysis")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "数值分析规则",
            &[
                ("误差分析基础", &self.error_analysis()),
                ("数值线性代数", &self.numerical_linear_algebra()),
                ("数值积分", &self.numerical_integration()),
                ("数值微分", &self.numerical_differentiation()),
                ("方程求根", &self.root_finding()),
                ("数值求解ODE", &self.solving_odes()),
                ("插值与拟合", &self.interpolation_and_fitting()),
                ("优化算法", &self.optimization_algorithms()),
                ("特殊函数计算", &self.special_functions()),
                ("应用领域", &self.applications()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numerical_analysis_rules() {
        let rules = NumericalAnalysisRules::new();
        assert_eq!(rules.metadata().name, "数值分析规则");
        assert!(!rules.explain().is_empty());
        assert!(!rules.error_analysis().is_empty());
        assert!(!rules.numerical_linear_algebra().is_empty());
        assert!(!rules.numerical_integration().is_empty());
        assert!(!rules.numerical_differentiation().is_empty());
        assert!(!rules.root_finding().is_empty());
        assert!(!rules.solving_odes().is_empty());
        assert!(!rules.interpolation_and_fitting().is_empty());
        assert!(!rules.optimization_algorithms().is_empty());
        assert!(!rules.special_functions().is_empty());
        assert!(!rules.applications().is_empty());
    }
}
