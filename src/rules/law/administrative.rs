//! 行政法基础规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 行政法规则
pub struct AdministrativeLawRules {
    metadata: RuleMetadata,
}

impl AdministrativeLawRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "行政法规则",
                "中国行政法基础知识"
            )
            .with_origin("中国")
            .with_tags(vec!["法律".into(), "行政法".into()]),
        }
    }

    /// 行政主体
    pub fn administrative_subjects(&self) -> Vec<&'static str> {
        vec![
            "行政机关: 各级政府及其部门",
            "授权组织: 法律授权行使行政权",
            "委托组织: 受委托行使行政权",
            "行政机构: 内设机构",
            "派出机构: 行政机关派出",
            "派出机关: 政府派出(街道办等)",
        ]
    }

    /// 行政行为类型
    pub fn administrative_actions(&self) -> Vec<&'static str> {
        vec![
            "行政许可: 批准从事特定活动",
            "行政处罚: 制裁违法行为",
            "行政强制: 强制履行义务",
            "行政征收: 征收税费财产",
            "行政给付: 提供物质帮助",
            "行政奖励: 给予表彰奖励",
            "行政裁决: 解决民事纠纷",
            "行政指导: 建议引导行为",
        ]
    }

    /// 行政许可规则
    pub fn administrative_license(&self) -> Vec<&'static str> {
        vec![
            "许可设定: 法律法规设定",
            "许可申请: 提出许可申请",
            "许可审查: 审查申请材料",
            "许可决定: 作出许可决定",
            "许可期限: 一般20日内",
            "许可听证: 重大许可听证",
            "许可变更: 变更许可内容",
            "许可延续: 续期申请",
        ]
    }

    /// 行政处罚规则
    pub fn administrative_penalty(&self) -> Vec<&'static str> {
        vec![
            "警告: 诫性处罚",
            "罚款: 经济性处罚",
            "没收违法所得/非法财物",
            "暂扣许可证件",
            "吊销许可证件",
            "责令停产停业",
            "行政拘留: 限制人身自由",
            "处罚设定权限分级",
        ]
    }

    /// 行政程序规则
    pub fn administrative_procedure(&self) -> Vec<&'static str> {
        vec![
            "信息公开: 公开政务信息",
            "告知义务: 告知当事人权利",
            "听取陈述申辩",
            "听证程序: 重大事项听证",
            "回避制度: 利益相关人员回避",
            "说明理由: 说明行为依据",
            "时效制度: 行为时间限制",
            "简易程序: 简单案件快速处理",
        ]
    }

    /// 行政复议规则
    pub fn administrative_review(&self) -> Vec<&'static str> {
        vec![
            "复议范围: 具体行政行为",
            "复议机关: 作出行为的上级机关",
            "复议申请: 60日内提出",
            "复议受理: 5日内决定受理",
            "复议审理: 审查行政行为",
            "复议决定: 维持/撤销/变更",
            "复议期限: 60日内作出决定",
            "复议效力: 行政救济手段",
        ]
    }

    /// 行政诉讼规则
    pub fn administrative_litigation(&self) -> Vec<&'static str> {
        vec![
            "诉讼范围: 具体行政行为",
            "管辖法院: 行为发生地法院",
            "起诉期限: 6个月内起诉",
            "被告主体: 作出行为的机关",
            "举证责任: 被告举证行为合法",
            "审理程序: 开庭审理",
            "判决类型: 维持/撤销/变更/确认",
            "执行程序: 强制执行判决",
        ]
    }

    /// 行政强制规则
    pub fn administrative_compulsion(&self) -> Vec<&'static str> {
        vec![
            "行政强制措施: 限制权利",
            "行政强制执行: 强制履行",
            "查封扣押: 控制财产",
            "冻结存款: 限制资金",
            "强制拆除: 拆除违建",
            "代履行: 代为履行义务",
            "执行催告: 催告履行",
            "强制执行程序规范",
        ]
    }

    /// 行政救济
    pub fn administrative_remedy(&self) -> Vec<&'static str> {
        vec![
            "行政复议: 内部救济",
            "行政诉讼: 司法救济",
            "行政赔偿: 损害赔偿",
            "行政补偿: 合法损失补偿",
            "信访: 民意反映渠道",
            "申诉: 申请复查",
            "检举控告: 监督举报",
            "仲裁: 特定纠纷仲裁",
        ]
    }
}

impl Default for AdministrativeLawRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for AdministrativeLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("administrative")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【行政法规则】\n\n行政行为类型:\n{}\n\n行政处罚规则:\n{}\n\n行政复议:\n{}\n",
            self.administrative_actions().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.administrative_penalty().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.administrative_review().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_administrative_law_rules() {
        let rules = AdministrativeLawRules::new();
        assert!(!rules.administrative_subjects().is_empty());
        assert!(!rules.administrative_actions().is_empty());
    }
}