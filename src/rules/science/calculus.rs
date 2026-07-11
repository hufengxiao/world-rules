//! 微积分规则
//!
//! 微积分是研究函数的微分、积分及其应用的数学分支。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: CalculusRules,
    name: "微积分规则",
    desc: "微分、积分及其应用的规则",
    origin: "数学",
    tags: ["科学", "数学", "微积分"]
}

impl CalculusRules {
    /// 微分基础
    pub fn differential_basics(&self) -> Vec<&'static str> {
        vec![
            "导数定义: f'(x) = lim[h→0] (f(x+h) - f(x))/h",
            "几何意义: 导数是曲线在某点的切线斜率",
            "物理意义: 导数表示瞬时变化率",
            "连续性: 可导函数必连续，连续函数未必可导",
            "基本导数公式: (x^n)' = n·x^(n-1)",
            "指数导数: (e^x)' = e^x, (a^x)' = a^x·ln(a)",
            "对数导数: (ln x)' = 1/x, (log_a x)' = 1/(x·ln a)",
            "三角导数: (sin x)' = cos x, (cos x)' = -sin x",
        ]
    }

    /// 求导法则
    pub fn differentiation_rules(&self) -> Vec<&'static str> {
        vec![
            "链式法则: (f(g(x)))' = f'(g(x))·g'(x)",
            "乘积法则: (f·g)' = f'·g + f·g'",
            "商法则: (f/g)' = (f'·g - f·g')/g²",
            "常数倍法则: (c·f)' = c·f'",
            "和法则: (f+g)' = f' + g'",
            "幂函数法则: (x^n)' = n·x^(n-1)",
            "反函数导数: 若 y = f(x)，则 dx/dy = 1/(dy/dx)",
            "隐函数求导: 对等式两边同时对 x 求导",
        ]
    }

    /// 积分基础
    pub fn integral_basics(&self) -> Vec<&'static str> {
        vec![
            "不定积分: F(x) = ∫f(x)dx，F'(x) = f(x)",
            "定积分: ∫[a,b]f(x)dx 表示曲线下的面积",
            "牛顿-莱布尼茨公式: ∫[a,b]f(x)dx = F(b) - F(a)",
            "基本积分公式: ∫x^n dx = x^(n+1)/(n+1) + C（n ≠ -1）",
            "指数积分: ∫e^x dx = e^x + C",
            "对数积分: ∫1/x dx = ln|x| + C",
            "三角积分: ∫sin x dx = -cos x + C, ∫cos x dx = sin x + C",
            "积分常数: 不定积分必加常数 C",
        ]
    }

    /// 积分技巧
    pub fn integration_techniques(&self) -> Vec<&'static str> {
        vec![
            "换元积分: ∫f(g(x))g'(x)dx = ∫f(u)du",
            "分部积分: ∫u dv = uv - ∫v du",
            "部分分式: 将有理函数分解为简单分式",
            "三角换元: √(a²-x²) 用 x = a·sinθ",
            "对称积分: 利用对称性简化计算",
            "递推公式: 通过递推关系求积分",
            "数值积分: 梯形法、辛普森法",
            "瑕积分: 积分区间有无限点或函数有奇点",
        ]
    }

    /// 应用问题
    pub fn applications(&self) -> Vec<&'static str> {
        vec![
            "极值问题: f'(x₀) = 0，f''(x₀) > 0 极小，f''(x₀) < 0 极大",
            "最值问题: 比较端点和极值点的函数值",
            "单调性: f'(x) > 0 增函数，f'(x) < 0 减函数",
            "凹凸性: f''(x) > 0 凹（上凸），f''(x) < 0 凸（下凸）",
            "拐点: f''(x₀) = 0 且凹凸性改变的点",
            "面积计算: ∫[a,b]f(x)dx 计算曲线与 x 轴间面积",
            "体积计算: V = ∫π[f(x)]²dx（旋转体体积）",
            "弧长计算: L = ∫√(1 + [f'(x)]²)dx",
        ]
    }

    /// 多元函数微积分
    pub fn multivariable_calculus(&self) -> Vec<&'static str> {
        vec![
            "偏导数: ∂f/∂x，对其中一个变量求导",
            "梯度: grad f = (∂f/∂x, ∂f/∂y, ∂f/∂z)",
            "方向导数: D_u f = grad f · u",
            "全微分: df = ∂f/∂x·dx + ∂f/∂y·dy",
            "链式法则: 多元复合函数求导",
            "极值条件: ∂f/∂x = 0, ∂f/∂y = 0",
            "拉格朗日乘数法: 条件极值问题",
            "重积分: ∫∫f(x,y)dxdy 计算曲面积",
        ]
    }

    /// 微分方程
    pub fn differential_equations(&self) -> Vec<&'static str> {
        vec![
            "常微分方程: 含有导数的方程",
            "阶数定义: 最高阶导数的阶数",
            "一阶ODE: dy/dx = f(x,y)",
            "可分离变量: dy/dx = g(x)·h(y)",
            "线性ODE: dy/dx + p(x)y = q(x)",
            "齐次方程: dy/dx = f(y/x)",
            "二阶线性ODE: y'' + py' + qy = 0",
            "特征方程: 求解线性ODE的关键",
        ]
    }

    /// 级数理论
    pub fn series_theory(&self) -> Vec<&'static str> {
        vec![
            "数列收敛: lim[n→∞] a_n 存在",
            "级数收敛: ∑a_n 收敛 iff 部分和序列收敛",
            "收敛判别: 比较判别法、比值判别法、根值判别法",
            "绝对收敛: ∑|a_n| 收敛",
            "条件收敛: ∑a_n 收敛但 ∑|a_n| 不收敛",
            "幂级数: ∑a_n·x^n，收敛半径 R",
            "泰勒级数: f(x) = ∑f^(n)(a)/n!·(x-a)^n",
            "麦克劳林级数: 泰勒级数的 a = 0 特例",
        ]
    }

    /// 数值方法
    pub fn numerical_methods(&self) -> Vec<&'static str> {
        vec![
            "数值微分: 用差分近似导数",
            "数值积分: 梯形公式、辛普森公式",
            "牛顿法: 求方程根的迭代方法",
            "欧拉法: 解ODE的简单方法",
            "龙格-库塔法: 更精确的ODE数值解法",
            "误差分析: 截断误差和舍入误差",
            "收敛速度: 方法收敛的快慢",
            "稳定性: 数值方法的稳定性分析",
        ]
    }

    /// 历史与应用
    pub fn history_and_applications(&self) -> Vec<&'static str> {
        vec![
            "牛顿与莱布尼茨: 微积分的创始人",
            "极限理论: 19世纪严格化微积分基础",
            "物理学: 运动学、电磁学的基础工具",
            "经济学: 边际分析、优化问题",
            "工程学: 信号处理、控制系统",
            "生物学: 种群动态、药物代谢",
            "计算机科学: 算法分析、数值计算",
            "现代发展: 非标准分析、微分几何",
        ]
    }
}

impl Rule for CalculusRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("calculus")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "微积分规则",
            &[
                ("微分基础", &self.differential_basics()),
                ("求导法则", &self.differentiation_rules()),
                ("积分基础", &self.integral_basics()),
                ("积分技巧", &self.integration_techniques()),
                ("应用问题", &self.applications()),
                ("多元函数微积分", &self.multivariable_calculus()),
                ("微分方程", &self.differential_equations()),
                ("级数理论", &self.series_theory()),
                ("数值方法", &self.numerical_methods()),
                ("历史与应用", &self.history_and_applications()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculus_rules() {
        let rules = CalculusRules::new();
        assert_eq!(rules.metadata().name, "微积分规则");
        assert!(!rules.explain().is_empty());
        assert!(!rules.differential_basics().is_empty());
        assert!(!rules.differentiation_rules().is_empty());
        assert!(!rules.integral_basics().is_empty());
        assert!(!rules.integration_techniques().is_empty());
        assert!(!rules.applications().is_empty());
        assert!(!rules.multivariable_calculus().is_empty());
        assert!(!rules.differential_equations().is_empty());
        assert!(!rules.series_theory().is_empty());
        assert!(!rules.numerical_methods().is_empty());
        assert!(!rules.history_and_applications().is_empty());
    }
}
