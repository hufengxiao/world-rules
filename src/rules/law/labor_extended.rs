//! 劳动法扩展规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 劳动法扩展规则
pub struct LaborLawExtendedRules {
    metadata: RuleMetadata,
}

impl LaborLawExtendedRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "劳动法扩展规则",
                "中国劳动法详细知识"
            )
            .with_origin("中国")
            .with_tags(vec!["法律".into(), "劳动法".into()]),
        }
    }

    /// 劳动合同类型
    pub fn contract_types(&self) -> Vec<&'static str> {
        vec![
            "固定期限劳动合同: 约定终止时间",
            "无固定期限劳动合同: 无终止时间",
            "以完成一定任务为期限劳动合同",
            "全日制劳动合同: 标准工时",
            "非全日制劳动合同: 不满标准工时",
            "劳务派遣合同: 派遣用工形式",
            "试用期规定: 试用期期限限制",
            "连续订立两次固定期限合同应转无固定期限",
        ]
    }

    /// 工时休假制度
    pub fn working_hours(&self) -> Vec<&'static str> {
        vec![
            "标准工时: 每日8小时每周40小时",
            "综合计算工时: 周期综合计算",
            "不定时工作制: 无固定时间",
            "延长工时限制: 每日不超过3小时",
            "每月延长工时: 不超过36小时",
            "法定节假日: 11天法定假期",
            "年休假: 5-15天年假",
            "加班工资: 150%-300%工资",
        ]
    }

    /// 工资报酬规则
    pub fn wage_rules(&self) -> Vec<&'static str> {
        vec![
            "最低工资标准: 地区最低工资",
            "工资支付周期: 至少每月支付",
            "工资支付形式: 货币支付",
            "工资支付记录: 工资清单保存",
            "不得克扣工资: 合法克扣情形",
            "不得拖欠工资: 及时支付义务",
            "加班工资计算: 基数计算方法",
            "特殊情形工资: 假期病假工资",
        ]
    }

    /// 劳动保护规则
    pub fn labor_protection(&self) -> Vec<&'static str> {
        vec![
            "安全生产义务: 提供安全条件",
            "安全培训义务: 安全教育培训",
            "安全设施配置: 安全防护设施",
            "职业病防护: 职业病防治",
            "劳动防护用品: 防护用品提供",
            "健康检查: 定期健康体检",
            "女职工保护: 特殊保护规定",
            "未成年工保护: 禁止高危作业",
        ]
    }

    /// 社会保险规则
    pub fn social_insurance(&self) -> Vec<&'static str> {
        vec![
            "基本养老保险: 养老保险缴纳",
            "基本医疗保险: 医疗保险缴纳",
            "工伤保险: 工伤保险缴纳",
            "失业保险: 失业保险缴纳",
            "生育保险: 生育保险缴纳",
            "缴费基数: 缴费计算基数",
            "缴费比例: 各险种比例",
            "社保待遇享受: 享受条件标准",
        ]
    }

    /// 劳动争议处理
    pub fn dispute_handling(&self) -> Vec<&'static str> {
        vec![
            "协商解决: 双方协商处理",
            "调解解决: 劳动争议调解",
            "仲裁解决: 劳动争议仲裁",
            "诉讼解决: 法院诉讼程序",
            "仲裁时效: 一年仲裁时效",
            "仲裁前置: 仲裁必经程序",
            "举证责任: 部分举证倒置",
            "一裁终局: 特定案件终局",
        ]
    }

    /// 解除终止劳动合同
    pub fn contract_termination(&self) -> Vec<&'static str> {
        vec![
            "协商解除: 双方协商解除",
            "劳动者单方解除: 劳动者辞职",
            "用人单位单方解除: 单位解雇",
            "过失性解除: 严重过失解除",
            "非过失性解除: 预告解除",
            "经济性裁员: 大规模裁员",
            "不得解除情形: 特定情形保护",
            "解除经济补偿: 补偿金标准",
        ]
    }

    /// 劳动监察
    pub fn labor_inspection(&self) -> Vec<&'static str> {
        vec![
            "劳动监察机构: 劳动监察部门",
            "监察范围: 劳动法规执行",
            "监察方式: 日常监察专项监察",
            "监察权限: 调查处理权限",
            "违法处理: 行政处罚措施",
            "投诉举报: 劳动者投诉渠道",
            "监察配合: 用人单位配合义务",
            "监察公开: 处理结果公开",
        ]
    }
}

impl Default for LaborLawExtendedRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for LaborLawExtendedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("labor_extended")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【劳动法扩展规则】\n\n合同类型:\n{}\n\n工时休假:\n{}\n\n社会保险:\n{}\n",
            self.contract_types().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.working_hours().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.social_insurance().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_labor_law_extended_rules() {
        let rules = LaborLawExtendedRules::new();
        assert!(!rules.contract_types().is_empty());
        assert!(!rules.working_hours().is_empty());
    }
}