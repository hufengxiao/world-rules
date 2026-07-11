//! 优化理论规则
//!
//! 优化理论研究如何在约束条件下寻找最优解。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: OptimizationRules,
    name: "优化理论规则",
    desc: "最优解的寻找方法与应用",
    origin: "数学",
    tags: ["科学", "数学", "优化"]
}

impl OptimizationRules {
    /// 优化问题基础
    pub fn optimization_basics(&self) -> Vec<&'static str> {
        vec![
            "目标函数: f(x)，需要优化的函数",
            "最大化问题: 求 f(x) 的最大值",
            "最小化问题: 求 f(x) 的最小值",
            "约束条件: g(x) ≤ 0, h(x) = 0",
            "可行域: 满足约束的解空间",
            "全局最优: 整个可行域中的最优解",
            "局部最优: 某邻域中的最优解",
            "最优性条件: 梯度、导数条件",
        ]
    }

    /// 无约束优化
    pub fn unconstrained_optimization(&self) -> Vec<&'static str> {
        vec![
            "梯度下降: 沿梯度负方向迭代",
            "牛顿法: 用二阶导数加速收敛",
            "最速下降法: 每步选最优方向",
            "共轭梯度法: 对二次函数高效",
            "拟牛顿法: 用近似代替精确二阶导",
            "信赖域方法: 在区域内求解",
            "线搜索: 寻找最优步长",
            "收敛性分析: 收敛速度、条件",
        ]
    }

    /// 约束优化
    pub fn constrained_optimization(&self) -> Vec<&'static str> {
        vec![
            "拉格朗日乘数法: 等式约束",
            "KKT 条件: 不等式约束的最优性条件",
            "惩罚函数: 将约束转化为惩罚项",
            "障碍函数: 阻止离开可行域",
            "可行方向法: 沿可行方向移动",
            "序列二次规划 SQP: 求解非线性约束",
            "投影梯度法: 每步投影到可行域",
            "增广拉格朗日: 结合惩罚和拉格朗日",
        ]
    }

    /// 线性规划
    pub fn linear_programming(&self) -> Vec<&'static str> {
        vec![
            "标准形式: min c^T·x, Ax ≤ b, x ≥ 0",
            "可行域: 多面体",
            "最优解: 在顶点或边界",
            "单纯形法: 沿边界搜索",
            "改进单纯形法: 减少计算量",
            "对偶理论: 原问题和对偶问题",
            "灵敏度分析: 参数变化的影响",
            "内点法: 从内部接近最优",
        ]
    }

    /// 整数规划
    pub fn integer_programming(&self) -> Vec<&'static str> {
        vec![
            "整数约束: x ∈ Z",
            "混合整数规划: 部分 x 为整数",
            "分支定界: 分支和剪枝",
            "割平面法: 切掉非整数解",
            "松弛: 放松整数约束",
            "启发式方法: 近似求解",
            "NP-hard: 大多数整数规划问题",
            "应用: 排班、选址、投资",
        ]
    }

    /// 非线性规划
    pub fn nonlinear_programming(&self) -> Vec<&'static str> {
        vec![
            "凸优化: f 为凸函数，约束为凸集",
            "凸性重要性: 局部最优即全局最优",
            "凸函数判定: 二阶导数 ≥ 0",
            "凸集判定: 包含任意连线",
            "凸优化算法: 可高效求解",
            "非凸优化: 可能多个局部最优",
            "全局优化: 分支定界、随机搜索",
            "应用: 机器学习、工程设计",
        ]
    }

    /// 动态规划
    pub fn dynamic_programming(&self) -> Vec<&'static str> {
        vec![
            "Bellman 方程: 递归关系",
            "最优子结构: 子问题最优导致整体最优",
            "无后效性: 未来只依赖当前状态",
            "状态转移方程: 状态间的转移规则",
            "递推求解: 从子问题到原问题",
            "记忆化搜索: 避免重复计算",
            "时间复杂度: O(n·m)",
            "应用: 路径规划、序列问题",
        ]
    }

    /// 组合优化
    pub fn combinatorial_optimization(&self) -> Vec<&'static str> {
        vec![
            "组合问题: 从有限集合中选最优",
            "背包问题: 选物品使价值最大",
            "旅行商问题 TSP: 最短巡回路径",
            "匹配问题: 最大匹配",
            "排序问题: 最优排序",
            "启发式算法: 贪心、局部搜索",
            "元启发式: GA、蚁群、PSO",
            "近似算法: 保证一定精度",
        ]
    }

    /// 随机优化
    pub fn stochastic_optimization(&self) -> Vec<&'static str> {
        vec![
            "随机目标函数: 含随机噪声",
            "随机梯度下降 SGD: 每步用部分数据",
            "样本平均近似 SAA: 用样本近似",
            "鲁棒优化: 考虑不确定性",
            "机会约束: 概率形式的约束",
            "贝叶斯优化: 用贝叶斯方法",
            "模拟退火: 随机接受较差解",
            "遗传算法: 模拟自然进化",
        ]
    }

    /// 应用领域
    pub fn applications(&self) -> Vec<&'static str> {
        vec![
            "机器学习: 参数优化、模型训练",
            "工程设计: 结构优化、参数选择",
            "经济学: 资源配置、投资组合",
            "运筹学: 排班、调度、物流",
            "控制系统: 最优控制",
            "信号处理: 滤波设计",
            "金融: 风险管理、资产配置",
            "生物学: 基因序列分析",
        ]
    }
}

impl Rule for OptimizationRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("optimization")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "优化理论规则",
            &[
                ("优化问题基础", &self.optimization_basics()),
                ("无约束优化", &self.unconstrained_optimization()),
                ("约束优化", &self.constrained_optimization()),
                ("线性规划", &self.linear_programming()),
                ("整数规划", &self.integer_programming()),
                ("非线性规划", &self.nonlinear_programming()),
                ("动态规划", &self.dynamic_programming()),
                ("组合优化", &self.combinatorial_optimization()),
                ("随机优化", &self.stochastic_optimization()),
                ("应用领域", &self.applications()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimization_rules() {
        let rules = OptimizationRules::new();
        assert_eq!(rules.metadata().name, "优化理论规则");
        assert!(!rules.explain().is_empty());
        assert!(!rules.optimization_basics().is_empty());
        assert!(!rules.unconstrained_optimization().is_empty());
        assert!(!rules.constrained_optimization().is_empty());
        assert!(!rules.linear_programming().is_empty());
        assert!(!rules.integer_programming().is_empty());
        assert!(!rules.nonlinear_programming().is_empty());
        assert!(!rules.dynamic_programming().is_empty());
        assert!(!rules.combinatorial_optimization().is_empty());
        assert!(!rules.stochastic_optimization().is_empty());
        assert!(!rules.applications().is_empty());
    }
}
