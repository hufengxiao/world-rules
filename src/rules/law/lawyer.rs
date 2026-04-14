//! 律师法基础规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 律师法规则
pub struct LawyerLawRules {
    metadata: RuleMetadata,
}

impl LawyerLawRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "律师法规则",
                "中国律师法基础知识"
            )
            .with_origin("中国")
            .with_tags(vec!["法律".into(), "律师".into()]),
        }
    }

    /// 律师执业条件
    pub fn practice_conditions(&self) -> Vec<&'static str> {
        vec![
            "律师资格取得",
            "律师执业申请",
            "律师执业许可",
            "律师执业证书",
            "实习律师要求",
            "律师执业限制",
            "律师执业注销",
            "律师执业恢复",
        ]
    }

    /// 律师权利
    pub fn lawyer_rights(&self) -> Vec<&'static str> {
        vec![
            "会见权: 会见当事人",
            "阅卷权: 查阅案卷",
            "调查取证权",
            "辩护权: 辩护表达",
            "代理权: 代理诉讼",
            "言论权: 发表意见",
            "人身权: 人身保护",
            "保密权: 保密特权",
        ]
    }

    /// 律师义务
    pub fn lawyer_obligations(&self) -> Vec<&'static str> {
        vec![
            "忠实义务: 忠于委托人",
            "保密义务: 信息保密",
            "勤勉义务: 勤勉尽责",
            "诚信义务: 诚实守信",
            "独立义务: 独立执业",
            "合规义务: 遵守法律",
            "竞业禁止义务",
            "利益冲突禁止",
        ]
    }

    /// 律师事务所规则
    pub fn law_firm_rules(&self) -> Vec<&'static str> {
        vec![
            "律师事务所设立",
            "律师事务所类型",
            "合伙律师事务所",
            "个人律师事务所",
            "律师事务所管理",
            "律师事务所分所",
            "律师事务所责任",
            "律师事务所监管",
        ]
    }

    /// 律师执业规范
    pub fn practice_standards(&self) -> Vec<&'static str> {
        vec![
            "执业行为规范",
            "职业道德规范",
            "收费规范: 收费标准",
            "广告规范: 广告限制",
            "竞争规范: 竞业规则",
            "利益冲突规范",
            "委托代理规范",
            "证据规范: 证据规则",
        ]
    }

    /// 律师业务范围
    pub fn business_scope(&self) -> Vec<&'static str> {
        vec![
            "刑事辩护代理",
            "民事诉讼代理",
            "行政诉讼代理",
            "非诉讼法律事务",
            "法律咨询服务",
            "法律文书起草",
            "法律顾问服务",
            "仲裁代理服务",
        ]
    }

    /// 律师禁止行为
    pub fn prohibited_behaviors(&self) -> Vec<&'static str> {
        vec![
            "虚假宣传行为",
            "不正当竞争行为",
            "利益冲突行为",
            "泄露秘密行为",
            "贿赂司法人员",
            "妨害司法公正",
            "违反收费规定",
            "违反执业规范",
        ]
    }

    /// 律师违法处罚
    pub fn lawyer_penalties(&self) -> Vec<&'static str> {
        vec![
            "警告处罚: 律师警告",
            "罚款处罚: 律师罚款",
            "停止执业处罚",
            "吊销执业证书",
            "吊销资格证书",
            "行业纪律处分",
            "民事赔偿责任",
            "刑事法律责任",
        ]
    }
}

impl Default for LawyerLawRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for LawyerLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("lawyer")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【律师法规则】\n\n执业条件:\n{}\n\n律师权利:\n{}\n\n律师义务:\n{}\n",
            self.practice_conditions().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.lawyer_rights().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.lawyer_obligations().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lawyer_law_rules() {
        let rules = LawyerLawRules::new();
        assert!(!rules.practice_conditions().is_empty());
        assert!(!rules.lawyer_rights().is_empty());
    }
}