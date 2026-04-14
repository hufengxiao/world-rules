//! 社会保险法基础规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 社会保险法规则
pub struct SocialInsuranceLawRules {
    metadata: RuleMetadata,
}

impl SocialInsuranceLawRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "社会保险法规则",
                "中国社会保险法基础知识"
            )
            .with_origin("中国")
            .with_tags(vec!["法律".into(), "社会保险".into()]),
        }
    }

    /// 社会保险类型
    pub fn insurance_types(&self) -> Vec<&'static str> {
        vec![
            "基本养老保险",
            "基本医疗保险",
            "工伤保险",
            "失业保险",
            "生育保险",
            "城乡居民养老保险",
            "城乡居民医疗保险",
            "住房公积金",
        ]
    }

    /// 养老保险规则
    pub fn pension_rules(&self) -> Vec<&'static str> {
        vec![
            "参保登记: 养老保险参保",
            "缴费基数: 缴费基数确定",
            "缴费比例: 单位个人比例",
            "缴费年限: 最低缴费年限",
            "养老金领取条件",
            "养老金计算方法",
            "养老金调整机制",
            "养老金转移接续",
        ]
    }

    /// 医疗保险规则
    pub fn medical_insurance_rules(&self) -> Vec<&'static str> {
        vec![
            "参保范围: 医保参保对象",
            "缴费标准: 医保缴费标准",
            "医保待遇: 医疗待遇范围",
            "门诊报销: 门诊费用报销",
            "住院报销: 住院费用报销",
            "大病保险: 大病医疗保障",
            "医保目录: 药品诊疗目录",
            "医保结算: 医保结算方式",
        ]
    }

    /// 工伤保险规则
    pub fn work_injury_rules(&self) -> Vec<&'static str> {
        vec![
            "工伤认定: 工伤认定条件",
            "工伤认定程序",
            "工伤认定时效",
            "工伤医疗待遇",
            "工伤伤残待遇",
            "工伤死亡待遇",
            "工伤康复待遇",
            "工伤预防措施",
        ]
    }

    /// 失业保险规则
    pub fn unemployment_rules(&self) -> Vec<&'static str> {
        vec![
            "参保条件: 失业保险参保",
            "缴费标准: 失业保险缴费",
            "领取条件: 失业金领取条件",
            "领取期限: 失业金领取期限",
            "领取标准: 失业金标准",
            "失业登记: 失业登记程序",
            "再就业服务: 就业促进服务",
            "失业保险转移",
        ]
    }

    /// 生育保险规则
    pub fn maternity_rules(&self) -> Vec<&'static str> {
        vec![
            "参保范围: 生育保险参保",
            "生育医疗待遇",
            "生育津贴待遇",
            "产假天数规定",
            "生育津贴计算",
            "计划生育待遇",
            "生育保险合并",
            "生育保险报销",
        ]
    }

    /// 社保缴费规则
    pub fn payment_rules(&self) -> Vec<&'static str> {
        vec![
            "缴费主体: 单位个人缴费",
            "缴费基数: 工资基数确定",
            "缴费比例: 各险种比例",
            "缴费时间: 缴费期限规定",
            "缴费方式: 缴费渠道方式",
            "欠费处理: 欠费追缴处理",
            "缴费记录: 缴费信息记录",
            "缴费证明: 缴费证明开具",
        ]
    }

    /// 社保权益保护
    pub fn rights_protection(&self) -> Vec<&'static str> {
        vec![
            "参保权利: 参保权利保障",
            "待遇领取权利",
            "信息查询权利",
            "转移接续权利",
            "异议申诉权利",
            "行政诉讼权利",
            "违规查处: 违法行为查处",
            "权益救济途径",
        ]
    }
}

impl Default for SocialInsuranceLawRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for SocialInsuranceLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("social_insurance")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【社会保险法规则】\n\n保险类型:\n{}\n\n养老保险:\n{}\n\n医疗保险:\n{}\n",
            self.insurance_types().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.pension_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.medical_insurance_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_social_insurance_law_rules() {
        let rules = SocialInsuranceLawRules::new();
        assert!(!rules.insurance_types().is_empty());
        assert!(!rules.pension_rules().is_empty());
    }
}