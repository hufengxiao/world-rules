//! 劳动法规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 劳动法规则
pub struct LaborLawRules {
    metadata: RuleMetadata,
}

impl LaborLawRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "劳动法规则",
                "中国劳动法基本规则"
            )
            .with_origin("中国")
            .with_tags(vec!["法律".into(), "劳动".into()]),
        }
    }

    /// 工作时间规定
    pub fn working_hours(&self) -> Vec<&'static str> {
        vec![
            "标准工作时间: 每日8小时，每周40小时",
            "加班每日不超过1小时 (特殊情况不超过3小时)",
            "每月加班不超过36小时",
            "加班工资: 工作日1.5倍，休息日2倍，法定节假日3倍",
        ]
    }

    /// 休假规定
    pub fn leave_rules(&self) -> Vec<&'static str> {
        vec![
            "法定节假日: 11天 (春节、清明、五一、端午、中秋、国庆等)",
            "带薪年假: 工作1-10年5天，10-20年10天，20年以上15天",
            "婚假: 3天 (各地可能有延长)",
            "产假: 98天基础 + 各地延长",
            "病假: 根据工龄确定工资比例",
        ]
    }

    /// 劳动合同规定
    pub fn contract_rules(&self) -> Vec<&'static str> {
        vec![
            "入职一个月内必须签订书面劳动合同",
            "未签合同: 用人单位需支付双倍工资",
            "试用期: 合同期限3月-1年试用期不超过1月",
            "试用期: 合同期限1-3年试用期不超过2月",
            "试用期工资不低于正式工资80%",
            "同一用人单位与同一劳动者只能约定一次试用期",
        ]
    }

    /// 解除劳动合同规定
    pub fn termination_rules(&self) -> Vec<&'static str> {
        vec![
            "用人单位单方解除需提前30天书面通知",
            "经济性裁员需提前30天向工会说明",
            "用人单位违法解除需支付双倍赔偿金",
            "劳动者解除需提前30天书面通知 (试用期提前3天)",
            "经济补偿: 每满一年支付一个月工资",
            "赔偿金: 经济补偿标准的二倍",
        ]
    }

    /// 工资规定
    pub fn wage_rules(&self) -> Vec<&'static str> {
        vec![
            "工资必须以货币形式按月支付",
            "不得克扣或无故拖欠工资",
            "最低工资标准由各省市规定",
            "工资支付周期不得超过一个月",
            "加班工资按规定标准支付",
        ]
    }

    /// 社会保险规定
    pub fn social_insurance(&self) -> Vec<&'static str> {
        vec![
            "五险: 养老、医疗、失业、工伤、生育",
            "用人单位和劳动者共同缴纳",
            "工伤保险由用人单位全额缴纳",
            "生育保险由用人单位全额缴纳",
            "入职30日内必须办理社保登记",
        ]
    }
}

impl Default for LaborLawRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for LaborLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("labor")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【劳动法规则】\n\n\
            工作时间:\n{}\n\n\
            休假规定:\n{}\n\n\
            劳动合同:\n{}\n\n\
            解除合同:\n{}\n\n\
            工资规定:\n{}\n\n\
            社会保险:\n{}\n",
            self.working_hours().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.leave_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.contract_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.termination_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.wage_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.social_insurance().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_labor_law() {
        let rules = LaborLawRules::new();
        assert!(rules.working_hours().contains(&"标准工作时间: 每日8小时，每周40小时"));
    }
}