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
            "【系统科学定律】\n\n理论定律:\n{}\n\n分析方法:\n{}\n\n优化定律:\n{}\n",
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