//! 统计法基础规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 统计法规则
pub struct StatisticsLawRules {
    metadata: RuleMetadata,
}

impl StatisticsLawRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "统计法规则",
                "中国统计法基础知识"
            )
            .with_origin("中国")
            .with_tags(vec!["法律".into(), "统计".into()]),
        }
    }

    /// 统计工作原则
    pub fn statistics_principles(&self) -> Vec<&'static str> {
        vec![
            "真实性原则",
            "科学性原则",
            "独立性原则",
            "统一性原则",
            "及时性原则",
            "完整性原则",
            "规范性原则",
            "公开性原则",
        ]
    }

    /// 统计调查管理
    pub fn statistics_survey(&self) -> Vec<&'static str> {
        vec![
            "统计调查项目管理",
            "统计调查方案审批",
            "统计调查方法规范",
            "统计调查指标设计",
            "统计调查周期管理",
            "统计调查组织实施",
            "统计调查质量控制",
            "统计调查监督检查",
        ]
    }

    /// 统计资料管理
    pub fn statistics_data(&self) -> Vec<&'static str> {
        vec![
            "统计资料收集整理",
            "统计资料审核核实",
            "统计资料报送管理",
            "统计资料存储保管",
            "统计资料保密管理",
            "统计资料档案管理",
            "统计资料质量评估",
            "统计资料监督检查",
        ]
    }

    /// 统计信息发布
    pub fn statistics_publication(&self) -> Vec<&'static str> {
        vec![
            "统计信息发布程序",
            "统计信息发布范围",
            "统计信息发布时效",
            "统计信息发布规范",
            "统计信息公开管理",
            "统计信息服务提供",
            "统计信息解读说明",
            "统计信息监督检查",
        ]
    }

    /// 统计机构管理
    pub fn statistics_institutions(&self) -> Vec<&'static str> {
        vec![
            "统计机构设置规定",
            "统计人员配备要求",
            "统计人员资质管理",
            "统计人员培训教育",
            "统计机构职责分工",
            "统计经费保障管理",
            "统计机构考核评估",
            "统计机构监督检查",
        ]
    }

    /// 统计监督机制
    pub fn statistics_supervision(&self) -> Vec<&'static str> {
        vec![
            "统计监督检查制度",
            "统计执法检查程序",
            "统计违法案件查处",
            "统计质量评估监督",
            "统计举报投诉处理",
            "统计信用管理机制",
            "统计责任追究制度",
            "统计监督公开机制",
        ]
    }

    /// 统计违法行为
    pub fn statistics_violations(&self) -> Vec<&'static str> {
        vec![
            "提供虚假统计资料",
            "拒绝提供统计资料",
            "迟报拒报统计资料",
            "伪造篡改统计资料",
            "强迫授意统计造假",
            "统计调查违法行为",
            "统计信息违法行为",
            "妨碍统计执法行为",
        ]
    }

    /// 统计法律责任
    pub fn statistics_legal_responsibility(&self) -> Vec<&'static str> {
        vec![
            "统计违法行政责任",
            "统计违法处分措施",
            "统计违法刑事责任",
            "统计违法民事责任",
            "统计违法处罚程序",
            "统计违法救济途径",
            "统计违法举报奖励",
            "统计违法责任追究",
        ]
    }
}

impl Default for StatisticsLawRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for StatisticsLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("statistics")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【统计法规则】\n\n工作原则:\n{}\n\n统计调查:\n{}\n\n统计资料:\n{}\n",
            self.statistics_principles().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.statistics_survey().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.statistics_data().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_statistics_law_rules() {
        let rules = StatisticsLawRules::new();
        assert!(!rules.statistics_principles().is_empty());
        assert!(!rules.statistics_survey().is_empty());
    }
}
