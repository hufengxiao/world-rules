//! 医疗法基础规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 医疗法规则
pub struct MedicalLawRules {
    metadata: RuleMetadata,
}

impl MedicalLawRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "医疗法规则",
                "中国医疗法基础知识"
            )
            .with_origin("中国")
            .with_tags(vec!["法律".into(), "医疗".into()]),
        }
    }

    /// 医疗机构管理
    pub fn medical_institution(&self) -> Vec<&'static str> {
        vec![
            "医疗机构许可: 执业许可证",
            "医疗机构分类: 公立私立民营",
            "医疗机构等级: 一级二级三级",
            "医疗机构设置: 设置规划审批",
            "医疗机构执业: 执业规则要求",
            "医疗机构登记: 登记注册管理",
            "医疗机构变更: 变更审批程序",
            "医疗机构终止: 注销终止程序",
        ]
    }

    /// 医师执业规则
    pub fn physician_practice(&self) -> Vec<&'static str> {
        vec![
            "医师资格: 医师资格考试",
            "医师执业注册: 注册执业地点",
            "医师执业范围: 专业范围限制",
            "医师执业权利: 诊疗处方权利",
            "医师执业义务: 诊疗救治义务",
            "医师执业禁止: 禁止违法行为",
            "医师多点执业: 多点执业规定",
            "医师职称晋升: 职称评定制度",
        ]
    }

    /// 护士执业规则
    pub fn nurse_practice(&self) -> Vec<&'static str> {
        vec![
            "护士资格: 护士资格考试",
            "护士执业注册: 注册执业管理",
            "护士执业权利: 护理操作权利",
            "护士执业义务: 护理服务义务",
            "护士执业禁止: 禁止违规行为",
            "护士职称晋升: 职称评定制度",
            "护士继续教育: 培训教育要求",
            "护士权利保护: 权利保障机制",
        ]
    }

    /// 医疗行为规则
    pub fn medical_practice(&self) -> Vec<&'static str> {
        vec![
            "诊疗规范: 诊疗技术规范",
            "处方管理: 处方开具规则",
            "病历书写: 病历书写规范",
            "知情同意: 患者知情同意",
            "转诊规则: 转诊送医规则",
            "急救救治: 急救救治义务",
            "医疗检查: 检查操作规范",
            "手术规则: 手术操作要求",
        ]
    }

    /// 患者权利保护
    pub fn patient_rights(&self) -> Vec<&'static str> {
        vec![
            "生命健康权: 基本健康权利",
            "知情同意权: 了解诊疗信息",
            "隐私保护权: 隐私信息保护",
            "选择权: 选择医疗机构医师",
            "病历查阅权: 查阅复印病历",
            "投诉举报权: 投诉举报渠道",
            "索赔权利: 医疗损害索赔",
            "尊严保护权: 人格尊严保护",
        ]
    }

    /// 医疗纠纷处理
    pub fn medical_dispute(&self) -> Vec<&'static str> {
        vec![
            "医疗纠纷报告: 纠纷报告制度",
            "医疗纠纷调解: 人民调解制度",
            "医疗纠纷仲裁: 仲裁解决途径",
            "医疗纠纷诉讼: 诉讼解决途径",
            "医疗损害鉴定: 损害鉴定程序",
            "医疗损害赔偿: 损害赔偿计算",
            "医疗责任保险: 责任保险制度",
            "医疗纠纷预防: 预防处理措施",
        ]
    }

    /// 医疗事故处理
    pub fn medical_malpractice(&self) -> Vec<&'static str> {
        vec![
            "医疗事故分级: 一级至四级",
            "医疗事故报告: 事故报告义务",
            "医疗事故调查: 调查处理程序",
            "医疗事故鉴定: 鉴定程序规则",
            "医疗事故处理: 处理程序规定",
            "医疗事故赔偿: 赔偿项目和标准",
            "医疗事故处罚: 行政处罚规定",
            "医疗事故预防: 预防措施要求",
        ]
    }

    /// 药品管理规则
    pub fn drug_management(&self) -> Vec<&'static str> {
        vec![
            "药品生产许可: 生产许可证",
            "药品经营许可: 经营许可证",
            "药品注册审批: 注册批准程序",
            "药品质量标准: 质量标准要求",
            "药品储存运输: 储运条件要求",
            "药品不良反应: 反应监测报告",
            "药品广告管理: 广告审批规定",
            "特殊药品管理: 麻醉精神药品",
        ]
    }
}

impl Default for MedicalLawRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for MedicalLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("medical")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【医疗法规则】\n\n医疗机构管理:\n{}\n\n患者权利:\n{}\n\n医疗纠纷处理:\n{}\n",
            self.medical_institution().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.patient_rights().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.medical_dispute().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_medical_law_rules() {
        let rules = MedicalLawRules::new();
        assert!(!rules.medical_institution().is_empty());
        assert!(!rules.patient_rights().is_empty());
    }
}