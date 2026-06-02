//! 系统科学定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 系统科学定律集合
pub struct SystemsScienceLaws {
    metadata: RuleMetadata,
}

impl SystemsScienceLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "系统科学定律",
                "系统科学基本定律"
            )
            .with_origin("科学")
            .with_tags(vec!["科学".into(), "系统".into()]),
        }
    }

    /// 系统理论定律
    pub fn theory_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("整体性定律", "整体大于部分", "系统整体性质"),
            ("层次性定律", "层次结构", "系统层次组织"),
            ("开放性定律", "开放系统", "系统与环境交互"),
            ("目的性定律", "目的行为", "系统目标导向"),
            ("动态性定律", "动态变化", "系统动态特性"),
            ("稳定性定律", "稳定状态", "系统稳定性"),
            ("适应性定律", "适应环境", "系统适应性"),
            ("涌现性定律", "涌现特性", "系统涌现现象"),
        ]
    }

    /// 系统分析方法
    pub fn analysis_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("系统分析定律", "整体分析", "系统分析方法"),
            ("结构分析定律", "结构解析", "系统结构分析"),
            ("功能分析定律", "功能解析", "系统功能分析"),
            ("环境分析定律", "环境关系", "系统环境分析"),
            ("信息分析定律", "信息流", "系统信息分析"),
            ("反馈分析定律", "反馈回路", "系统反馈分析"),
            ("模型分析定律", "模型建立", "系统建模方法"),
        ]
    }

    /// 系统优化定律
    pub fn optimization_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("优化定律", "最优设计", "系统优化原则"),
            ("目标定律", "目标设定", "优化目标确定"),
            ("约束定律", "约束条件", "优化约束条件"),
            ("决策定律", "决策方法", "系统决策理论"),
            ("权衡定律", "权衡取舍", "优化权衡原则"),
            ("效率定律", "效率优化", "系统效率提高"),
            ("可靠性定律", "可靠优化", "可靠性优化"),
        ]
    }

    /// 复杂系统定律
    pub fn complexity_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("复杂定律", "复杂性", "复杂系统特性"),
            ("非线性定律", "非线性关系", "非线性系统"),
            ("自组织定律", "自组织", "自组织现象"),
            ("混沌定律", "混沌行为", "混沌系统特性"),
            ("分形定律", "分形结构", "分形系统"),
            ("网络定律", "网络结构", "网络系统"),
            ("演化定律", "演化过程", "系统演化规律"),
        ]
    }

    /// 系统类型
    pub fn system_types(&self) -> Vec<&'static str> {
        vec![
            "物理系统",
            "生物系统",
            "社会系统",
            "经济系统",
            "信息系统",
            "管理系统",
            "技术系统",
            "生态系统",
        ]
    }

    /// 系统方法
    pub fn methods(&self) -> Vec<&'static str> {
        vec![
            "系统工程",
            "系统分析",
            "系统设计",
            "系统评价",
            "系统管理",
            "系统优化",
            "系统建模",
            "系统仿真",
        ]
    }

    /// 信息论定律
    pub fn information_theory_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("信息熵定律", "H = -Σp·log(p)", "信息熵度量信息量"),
            ("信道容量定律", "C = B·log(1+S/N)", "香农信道容量公式"),
            ("编码定理", "信源编码", "无损压缩编码极限"),
            ("率失真定律", "率失真函数", "有损压缩理论极限"),
            ("互信息定律", "I(X;Y)", "变量间互信息度量"),
            ("数据处理不等式", "信息不增", "数据处理不会增加信息"),
            ("最大熵定律", "最大熵原理", "无约束下最大熵分布"),
            ("最小描述长度定律", "MDL", "模型选择最短描述"),
        ]
    }

    /// 控制论定律
    pub fn cybernetics_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("必要多样性定律", "Ashby定律", "控制器多样性需匹配被控对象"),
            ("反馈控制定律", "负反馈", "负反馈维持系统稳定"),
            ("正反馈定律", "正反馈", "正反馈导致系统失控"),
            ("黑箱定律", "黑箱方法", "通过输入输出理解系统"),
            ("循环因果定律", "因果环", "系统中因果循环关系"),
            ("自适应定律", "自适应", "系统自适应调节机制"),
            ("学习系统定律", "学习能力", "系统通过反馈学习改进"),
            ("目的论定律", "目的导向", "系统目的导向行为"),
        ]
    }

    /// 协同学定律
    pub fn synergetics_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("序参量定律", "序参量", "序参量支配系统行为"),
            ("伺服原理定律", "快变量消除", "快变量受慢变量支配"),
            ("协同效应定律", "协同作用", "子系统协同产生宏观有序"),
            ("对称破缺定律", "对称破缺", "相变中对称性降低"),
            ("临界涨落定律", "临界涨落", "临界点附近涨落放大"),
            ("自组织临界定律", "SOC", "系统自发演化到临界态"),
            ("模式形成定律", "空间模式", "耗散结构空间模式形成"),
            ("竞争协作定律", "竞争协作", "子系统竞争与协作并存"),
        ]
    }

    /// 系统思维
    pub fn systems_thinking(&self) -> Vec<&'static str> {
        vec![
            "整体涌现: 整体具有部分所没有的性质",
            "反馈环路: 正反馈增强变化负反馈维持稳定",
            "非线性: 输出与输入不成比例的系统行为",
            "自组织: 系统在没有外部指令下自发形成有序结构",
            "耗散结构: 开放系统远离平衡态时形成的有序结构",
            "协同学: 不同子系统协同作用产生宏观有序",
        ]
    }

    /// 复杂系统
    pub fn complex_systems(&self) -> Vec<&'static str> {
        vec![
            "复杂适应系统: 由适应性主体相互作用形成的系统",
            "涌现行为: 系统整体表现出的微观层面没有的行为",
            "幂律分布: 许多复杂系统中事件规模服从幂律",
            "小世界网络: 高聚集系数和短平均路径长度的网络",
            "无标度网络: 节点度分布服从幂律的网络",
            "鲁棒性与脆弱性: 复杂系统对随机故障鲁棒对蓄意攻击脆弱",
        ]
    }

}

impl Default for SystemsScienceLaws {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for SystemsScienceLaws {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("systems_science")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【系统科学定律】\n\n理论定律:\n{}\n\n分析方法:\n{}\n\n优化定律:\n{}\n\n信息论定律:\n{}\n\n控制论定律:\n{}\n\n协同学定律:\n{}\n",
            self.theory_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.analysis_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.optimization_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.information_theory_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.cybernetics_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.synergetics_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_systems_science_laws() {
        let laws = SystemsScienceLaws::new();
        assert!(!laws.theory_laws().is_empty());
        assert!(!laws.complexity_laws().is_empty());
    }
}