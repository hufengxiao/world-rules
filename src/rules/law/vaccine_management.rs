//! 疫苗管理法基础规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 疫苗管理法规则
pub struct VaccineManagementLawRules {
    metadata: RuleMetadata,
}

impl VaccineManagementLawRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "疫苗管理法规则",
                "中国疫苗管理法基础知识"
            )
            .with_origin("中国")
            .with_tags(vec!["法律".into(), "疫苗管理".into()]),
        }
    }

    /// 疫苗研制规则
    pub fn vaccine_research(&self) -> Vec<&'static str> {
        vec![
            "疫苗研制规范",
            "临床试验管理",
            "疫苗注册审批",
            "研制资料管理",
            "研制伦理审查",
            "研制安全评估",
            "研制信息公开",
            "研制监督机制",
        ]
    }

    /// 疫苗生产规则
    pub fn vaccine_production(&self) -> Vec<&'static str> {
        vec![
            "疫苗生产许可",
            "生产质量管理规范",
            "生产过程控制",
            "生产检验要求",
            "生产环境管理",
            "生产变更管理",
            "生产记录管理",
            "生产监督检查",
        ]
    }

    /// 疫苗流通规则
    pub fn vaccine_distribution(&self) -> Vec<&'static str> {
        vec![
            "疫苗流通许可",
            "疫苗采购管理",
            "疫苗储存管理",
            "疫苗运输管理",
            "疫苗冷链管理",
            "疫苗追溯体系",
            "疫苗信息记录",
            "疫苗监督检查",
        ]
    }

    /// 疫苗接种规则
    pub fn vaccine_administration(&self) -> Vec<&'static str> {
        vec![
            "疫苗接种规范",
            "接种单位管理",
            "接种人员资质",
            "接种信息记录",
            "接种知情同意",
            "接种异常反应监测",
            "接种安全保障",
            "接种效果评估",
        ]
    }

    /// 疫苗追溯体系
    pub fn vaccine_traceability(&self) -> Vec<&'static str> {
        vec![
            "疫苗追溯编码",
            "追溯信息记录",
            "追溯平台建设",
            "追溯信息共享",
            "追溯查询服务",
            "追溯监管应用",
            "追溯数据安全",
            "追溯国际合作",
        ]
    }

    /// 疫苗异常反应
    pub fn abnormal_reaction(&self) -> Vec<&'static str> {
        vec![
            "异常反应监测报告",
            "异常反应诊断鉴定",
            "异常反应补偿机制",
            "异常反应救助措施",
            "异常反应预防措施",
            "异常反应信息公开",
            "异常反应责任追究",
            "异常反应保险机制",
        ]
    }

    /// 疫苗监督管理
    pub fn vaccine_supervision(&self) -> Vec<&'static str> {
        vec![
            "疫苗监督检查",
            "疫苗抽检检验",
            "疫苗风险评估",
            "疫苗召回管理",
            "疫苗处罚措施",
            "疫苗信用管理",
            "疫苗信息公开",
            "疫苗举报处理",
        ]
    }

    /// 疫苗违法行为
    pub fn vaccine_violations(&self) -> Vec<&'static str> {
        vec![
            "生产销售假疫苗",
            "生产销售劣疫苗",
            "无证生产经营疫苗",
            "违规生产疫苗行为",
            "违规流通疫苗行为",
            "违规接种疫苗行为",
            "伪造疫苗信息行为",
            "疫苗数据造假行为",
        ]
    }
}

impl Default for VaccineManagementLawRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for VaccineManagementLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("vaccine_management")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【疫苗管理法规则】\n\n疫苗研制:\n{}\n\n疫苗生产:\n{}\n\n疫苗接种:\n{}\n",
            self.vaccine_research().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.vaccine_production().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.vaccine_administration().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vaccine_management_law_rules() {
        let rules = VaccineManagementLawRules::new();
        assert!(!rules.vaccine_research().is_empty());
        assert!(!rules.vaccine_production().is_empty());
    }
}
