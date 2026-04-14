//! 管理科学定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 管理科学定律集合
pub struct ManagementScienceLaws {
    metadata: RuleMetadata,
}

impl ManagementScienceLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "管理科学定律",
                "管理科学基本定律"
            )
            .with_origin("科学")
            .with_tags(vec!["科学".into(), "管理".into()]),
        }
    }

    /// 管理理论定律
    pub fn theory_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("科学管理定律", "效率优化", "泰勒科学管理"),
            ("过程管理定律", "过程控制", "管理过程理论"),
            ("人本管理定律", "以人为本", "人际关系理论"),
            ("系统管理定律", "系统视角", "系统管理理论"),
            ("权变管理定律", "因地制宜", "权变管理理论"),
            ("目标管理定律", "目标导向", "目标管理方法"),
            ("知识管理定律", "知识创造", "知识管理理论"),
        ]
    }

    /// 组织定律
    pub fn organization_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("组织结构定律", "结构设计", "组织结构设计"),
            ("分工定律", "工作分工", "专业化分工"),
            ("层级定律", "管理层次", "管理层级设置"),
            ("授权定律", "权力分配", "权力授权机制"),
            ("协调定律", "工作协调", "组织协调方法"),
            ("沟通定律", "信息流通", "组织沟通机制"),
            ("激励定律", "员工激励", "激励管理方法"),
        ]
    }

    /// 战略定律
    pub fn strategy_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("战略定律", "战略规划", "战略管理过程"),
            ("竞争定律", "竞争优势", "竞争战略分析"),
            ("定位定律", "市场定位", "战略定位方法"),
            ("资源配置定律", "资源分配", "战略资源配置"),
            ("核心能力定律", "核心竞争力", "核心能力建设"),
            ("价值链定律", "价值创造", "价值链分析"),
            ("差异化定律", "差异化竞争", "差异化战略"),
        ]
    }

    /// 运营定律
    pub fn operation_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("运营定律", "运营管理", "运营管理系统"),
            ("流程定律", "流程优化", "业务流程管理"),
            ("质量定律", "质量管理", "质量管理体系"),
            ("成本定律", "成本控制", "成本管理方法"),
            ("效率定律", "效率提升", "运营效率优化"),
            ("精益定律", "精益管理", "精益生产方法"),
            ("供应链定律", "供应链管理", "供应链优化"),
        ]
    }

    /// 管理领域
    pub fn areas(&self) -> Vec<&'static str> {
        vec![
            "人力资源管理",
            "财务管理",
            "市场营销",
            "生产运营",
            "项目管理",
            "战略管理",
            "创新管理",
            "风险管理",
        ]
    }

    /// 管理方法
    pub fn methods(&self) -> Vec<&'static str> {
        vec![
            "目标管理",
            "项目管理",
            "质量管理",
            "流程管理",
            "绩效管理",
            "危机管理",
            "变革管理",
            "知识管理",
        ]
    }
}

impl Default for ManagementScienceLaws {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ManagementScienceLaws {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("management_science")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【管理科学定律】\n\n理论定律:\n{}\n\n组织定律:\n{}\n\n战略定律:\n{}\n",
            self.theory_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.organization_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.strategy_laws().iter()
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
    fn test_management_science_laws() {
        let laws = ManagementScienceLaws::new();
        assert!(!laws.theory_laws().is_empty());
        assert!(!laws.operation_laws().is_empty());
    }
}