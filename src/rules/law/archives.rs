//! 档案法基础规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 档案法规则
pub struct ArchivesLawRules {
    metadata: RuleMetadata,
}

impl ArchivesLawRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "档案法规则",
                "中国档案法基础知识"
            )
            .with_origin("中国")
            .with_tags(vec!["法律".into(), "档案".into()]),
        }
    }

    /// 档案管理原则
    pub fn archives_management_principles(&self) -> Vec<&'static str> {
        vec![
            "统一管理原则",
            "分级管理原则",
            "安全保管原则",
            "依法开放原则",
            "服务社会原则",
            "信息化管理原则",
            "完整保存原则",
            "真实记录原则",
        ]
    }

    /// 档案收集规则
    pub fn archives_collection(&self) -> Vec<&'static str> {
        vec![
            "档案归档范围",
            "档案归档要求",
            "档案移交规则",
            "档案接收规则",
            "档案征集规则",
            "档案捐赠规则",
            "档案代管规则",
            "档案收集监督检查",
        ]
    }

    /// 档案保管规则
    pub fn archives_preservation(&self) -> Vec<&'static str> {
        vec![
            "档案保管条件要求",
            "档案库房管理规范",
            "档案安全防护措施",
            "档案保管期限规定",
            "档案定期检查制度",
            "档案抢救修复管理",
            "档案保密管理规定",
            "档案保管监督检查",
        ]
    }

    /// 档案开放利用
    pub fn archives_access(&self) -> Vec<&'static str> {
        vec![
            "档案开放范围规定",
            "档案开放时限要求",
            "档案利用申请程序",
            "档案查阅服务规范",
            "档案复制服务规范",
            "档案公布管理规则",
            "档案利用收费管理",
            "档案利用监督检查",
        ]
    }

    /// 档案信息化管理
    pub fn archives_digitization(&self) -> Vec<&'static str> {
        vec![
            "档案数字化建设规划",
            "档案数字化技术标准",
            "电子档案管理规范",
            "档案信息安全管理",
            "档案数据库建设管理",
            "档案信息化应用推广",
            "档案数字资源共享",
            "档案信息化监督检查",
        ]
    }

    /// 档案机构管理
    pub fn archives_institutions(&self) -> Vec<&'static str> {
        vec![
            "档案馆建设管理",
            "档案机构设置规定",
            "档案人员资质要求",
            "档案人员培训管理",
            "档案机构职责分工",
            "档案机构协作机制",
            "档案机构监督检查",
            "档案机构考核评估",
        ]
    }

    /// 档案违法行为
    pub fn archives_violations(&self) -> Vec<&'static str> {
        vec![
            "丢失档案违法行为",
            "擅自销毁档案行为",
            "涂改伪造档案行为",
            "擅自提供档案行为",
            "擅自公布档案行为",
            "档案保管不当行为",
            "拒不归档移交行为",
            "妨碍档案执法行为",
        ]
    }

    /// 档案法律责任
    pub fn archives_legal_responsibility(&self) -> Vec<&'static str> {
        vec![
            "档案违法行政责任",
            "档案违法民事责任",
            "档案违法刑事责任",
            "档案损害赔偿责任",
            "档案违法处罚措施",
            "档案违法举报处理",
            "档案争议处理机制",
            "档案救济途径保障",
        ]
    }
}

impl Default for ArchivesLawRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ArchivesLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("archives")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【档案法规则】\n\n管理原则:\n{}\n\n档案保管:\n{}\n\n档案开放:\n{}\n",
            self.archives_management_principles().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.archives_preservation().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.archives_access().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_archives_law_rules() {
        let rules = ArchivesLawRules::new();
        assert!(!rules.archives_management_principles().is_empty());
        assert!(!rules.archives_collection().is_empty());
    }
}
