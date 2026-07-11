//! 离散数学规则
//!
//! 离散数学研究离散结构和离散对象，是计算机科学的基础。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: DiscreteMathRules,
    name: "离散数学规则",
    desc: "离散结构、逻辑与算法",
    origin: "数学",
    tags: ["科学", "数学", "离散"]
}

impl DiscreteMathRules {
    /// 数理逻辑基础
    pub fn mathematical_logic(&self) -> Vec<&'static str> {
        vec![
            "命题逻辑: 真值和命题运算",
            "命题联结词: ∧(与)、∨(或)、¬(非)、→(蕴含)",
            "真值表: 列出命题的所有可能值",
            "逻辑等价: p ≡ q，真值相同",
            "重言式: 永真的命题",
            "矛盾式: 永假的命题",
            "谓词逻辑: 包含量词 ∀、∃",
            "推理规则: 假言推理、拒取式",
        ]
    }

    /// 集合论
    pub fn set_theory(&self) -> Vec<&'static str> {
        vec![
            "集合定义: 元素的集合",
            "集合运算: ∪(并)、∩(交)、差、补",
            "集合关系: ⊆(子集)、∈(属于)",
            "幂集: 所有子集的集合 P(A)",
            "笛卡尔积: A × B = {(a,b) | a∈A, b∈B}",
            "关系: A × B 的子集",
            "函数: 特殊的关系",
            "集合基数: 集合的大小",
        ]
    }

    /// 关系与函数
    pub fn relations_and_functions(&self) -> Vec<&'static str> {
        vec![
            "关系性质: 自反、对称、反对称、传递",
            "等价关系: 自反、对称、传递",
            "等价类: [a] = {x | (a,x) ∈ R}",
            "偏序关系: 自反、反对称、传递",
            "全序关系: 偏序且任意元素可比较",
            "函数映射: f: A → B",
            "函数性质: 单射、满射、双射",
            "逆函数: f^(-1): B → A",
        ]
    }

    /// 组合数学
    pub fn combinatorics(&self) -> Vec<&'static str> {
        vec![
            "排列: P(n,r) = n!/(n-r)!",
            "组合: C(n,r) = n!/(r!(n-r)!)",
            "二项式定理: (x+y)^n = ΣC(n,k)x^k y^(n-k)",
            "杨辉三角: Pascal 三角形",
            "鸽巢原理: n+1 个鸽子放入 n 个巢",
            "容斥原理: |A∪B| = |A| + |B| - |A∩B|",
            "递推关系: Hanoi 塔、斐波那契",
            "生成函数: 用函数表示序列",
        ]
    }

    /// 离散概率
    pub fn discrete_probability(&self) -> Vec<&'static str> {
        vec![
            "概率空间: (Ω, P)",
            "事件: Ω 的子集",
            "概率定义: P(A) = |A|/|Ω|",
            "条件概率: P(B|A) = P(A∩B)/P(A)",
            "独立事件: P(A∩B) = P(A)·P(B)",
            "期望: E(X) = Σx·P(x)",
            "方差: Var(X) = E(X²) - E(X)²",
            "离散分布: 伯努利、二项、泊松",
        ]
    }

    /// 递归与归纳
    pub fn recursion_and_induction(&self) -> Vec<&'static str> {
        vec![
            "递归定义: 用自身定义",
            "递归算法: 分治策略",
            "数学归纳法: 证明步骤",
            "基础步骤: P(1) 成立",
            "归纳步骤: P(n) ⇒ P(n+1)",
            "强归纳: P(1,...,n) ⇒ P(n+1)",
            "递推关系求解: 特征方程法",
            "应用: 算法正确性证明",
        ]
    }

    /// 离散结构
    pub fn discrete_structures(&self) -> Vec<&'static str> {
        vec![
            "序列: 有序的元素列表",
            "字符串: 字符的序列",
            "矩阵: 数的二维阵列",
            "图: 顶点和边的结构",
            "树: 无回路的连通图",
            "布尔代数: {0,1} 上的运算",
            "有限状态机: 状态和转移",
            "形式语言: 字符串的集合",
        ]
    }

    /// 算法基础
    pub fn algorithm_basics(&self) -> Vec<&'static str> {
        vec![
            "算法定义: 解决问题的有限步骤",
            "时间复杂度: 运行时间度量",
            "空间复杂度: 内存使用度量",
            "大 O 表示: O(n)、O(n²)、O(log n)",
            "最好情况: 最优输入下的复杂度",
            "最坏情况: 最差输入下的复杂度",
            "平均情况: 平均输入下的复杂度",
            "P vs NP: 复杂性理论核心问题",
        ]
    }

    /// 计算理论
    pub fn computation_theory(&self) -> Vec<&'static str> {
        vec![
            "有限自动机 DFA/NFA: 状态机",
            "正则表达式: 描述正则语言",
            "上下文无关文法 CFG: 生成 CFL",
            "下推自动机 PDA: 识别 CFL",
            "图灵机: 理论计算模型",
            "可计算性: 可计算函数",
            "停机问题: 不可判定",
            "复杂性类: P、NP、NP-complete",
        ]
    }

    /// 应用领域
    pub fn applications(&self) -> Vec<&'static str> {
        vec![
            "计算机科学: 算法、数据结构",
            "软件工程: 程序验证",
            "数据库系统: 关系代数",
            "人工智能: 知识表示",
            "密码学: 数论基础",
            "网络协议: 状态机设计",
            "编译器: 词法、语法分析",
            "数字逻辑: 电路设计",
        ]
    }
}

impl Rule for DiscreteMathRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("discrete_math")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "离散数学规则",
            &[
                ("数理逻辑基础", &self.mathematical_logic()),
                ("集合论", &self.set_theory()),
                ("关系与函数", &self.relations_and_functions()),
                ("组合数学", &self.combinatorics()),
                ("离散概率", &self.discrete_probability()),
                ("递归与归纳", &self.recursion_and_induction()),
                ("离散结构", &self.discrete_structures()),
                ("算法基础", &self.algorithm_basics()),
                ("计算理论", &self.computation_theory()),
                ("应用领域", &self.applications()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discrete_math_rules() {
        let rules = DiscreteMathRules::new();
        assert_eq!(rules.metadata().name, "离散数学规则");
        assert!(!rules.explain().is_empty());
        assert!(!rules.mathematical_logic().is_empty());
        assert!(!rules.set_theory().is_empty());
        assert!(!rules.relations_and_functions().is_empty());
        assert!(!rules.combinatorics().is_empty());
        assert!(!rules.discrete_probability().is_empty());
        assert!(!rules.recursion_and_induction().is_empty());
        assert!(!rules.discrete_structures().is_empty());
        assert!(!rules.algorithm_basics().is_empty());
        assert!(!rules.computation_theory().is_empty());
        assert!(!rules.applications().is_empty());
    }
}
