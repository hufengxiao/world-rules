//! 安全生产法基础规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 安全生产法规则
pub struct SafetyProductionLawRules {
    metadata: RuleMetadata,
}

impl SafetyProductionLawRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "安全生产法规则",
                "中国安全生产法基础知识"
            )
            .with_origin("中国")
            .with_tags(vec!["法律".into(), "安全生产".into()]),
        }
    }

    /// 安全生产原则
    pub fn safety_principles(&self) -> Vec<&'static str> {
        vec![
            "安全第一原则",
            "预防为主原则",
            "综合治理原则",
            "以人为本原则",
            "全员参与原则",
            "持续改进原则",
            "依法治理原则",
            "科学管理原则",
        ]
    }

    /// 企业安全责任
    pub fn enterprise_responsibility(&self) -> Vec<&'static str> {
        vec![
            "主要负责人责任",
            "安全管理人员责任",
            "从业人员安全责任",
            "安全投入保障责任",
            "安全教育培训责任",
            "安全设施配备责任",
            "隐患排查治理责任",
            "事故报告处理责任",
        ]
    }

    /// 安全管理制度
    pub fn safety_management_system(&self) -> Vec<&'static str> {
        vec![
            "安全生产责任制",
            "安全检查制度",
            "隐患排查制度",
            "安全培训制度",
            "应急预案制度",
            "事故报告制度",
            "安全奖惩制度",
            "安全档案管理制度",
        ]
    }

    /// 安全生产条件
    pub fn safety_conditions(&self) -> Vec<&'static str> {
        vec![
            "安全设施配备",
            "安全设备维护",
            "安全警示标识",
            "安全操作规程",
            "安全防护用品",
            "应急救援设备",
            "安全通道设置",
            "作业环境安全",
        ]
    }

    /// 安全培训要求
    pub fn safety_training(&self) -> Vec<&'static str> {
        vec![
            "主要负责人培训",
            "安全管理人员培训",
            "特种作业人员培训",
            "新员工三级培训",
            "转岗培训要求",
            "复训周期要求",
            "培训档案管理",
            "培训考核要求",
        ]
    }

    /// 危险源管理
    pub fn hazard_management(&self) -> Vec<&'static str> {
        vec![
            "重大危险源辨识",
            "危险源登记建档",
            "危险源评估分级",
            "危险源监控监测",
            "危险源应急预案",
            "危险源告知公示",
            "危险源检查整改",
            "危险源销号管理",
        ]
    }

    /// 事故应急处理
    pub fn emergency_response(&self) -> Vec<&'static str> {
        vec![
            "应急预案制定",
            "应急演练开展",
            "应急物资储备",
            "应急队伍建设",
            "事故报告程序",
            "事故应急处置",
            "事故调查分析",
            "事故整改落实",
        ]
    }

    /// 安全违法行为
    pub fn safety_violations(&self) -> Vec<&'static str> {
        vec![
            "无证生产违法",
            "安全设施缺失",
            "培训不到位",
            "隐患未整改",
            "超范围生产",
            "违章指挥作业",
            "冒险作业行为",
            "瞒报事故行为",
        ]
    }
}

impl Default for SafetyProductionLawRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for SafetyProductionLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("safety_production")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【安全生产法规则】\n\n安全原则:\n{}\n\n企业责任:\n{}\n\n管理制度:\n{}\n",
            self.safety_principles().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.enterprise_responsibility().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.safety_management_system().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safety_production_law_rules() {
        let rules = SafetyProductionLawRules::new();
        assert!(!rules.safety_principles().is_empty());
        assert!(!rules.enterprise_responsibility().is_empty());
    }
}