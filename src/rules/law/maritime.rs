//! 海商法基础规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 海商法规则
pub struct MaritimeLawRules {
    metadata: RuleMetadata,
}

impl MaritimeLawRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "海商法规则",
                "中国海商法基础知识"
            )
            .with_origin("中国")
            .with_tags(vec!["法律".into(), "海商".into()]),
        }
    }

    /// 船舶法律规则
    pub fn ship_rules(&self) -> Vec<&'static str> {
        vec![
            "船舶所有权: 船舶权属登记",
            "船舶抵押权: 船舶抵押登记",
            "船舶优先权: 海事优先权",
            "船舶留置权: 船舶留置权利",
            "船舶登记: 船舶权属登记制度",
            "船舶国籍: 船舶国籍登记",
            "船舶检验: 船舶安全检验",
            "船舶买卖: 船舶交易规则",
        ]
    }

    /// 船员法律规则
    pub fn crew_rules(&self) -> Vec<&'static str> {
        vec![
            "船员资格: 船员适任证书",
            "船员培训: 船员培训要求",
            "船员权利: 劳动报酬权利",
            "船员义务: 船员职责义务",
            "船长职责: 船长管理职责",
            "船员配备: 船舶配员要求",
            "船员合同: 劳动合同签订",
            "船员权益保护: 权益保障机制",
        ]
    }

    /// 海上货物运输规则
    pub fn cargo_transport(&self) -> Vec<&'static str> {
        vec![
            "运输合同: 海上货物运输合同",
            "承运人责任: 承运人责任义务",
            "托运人义务: 托运人交付义务",
            "提单制度: 提单签发流转",
            "货物交付: 货物交接规则",
            "货物灭失赔偿: 赔偿限额规定",
            "货物损坏赔偿: 损害赔偿计算",
            "免责事项: 承运人免责情形",
        ]
    }

    /// 海上旅客运输规则
    pub fn passenger_transport(&self) -> Vec<&'static str> {
        vec![
            "客票制度: 旅客运输客票",
            "承运人责任: 人身财产安全",
            "旅客权利: 旅客权利保护",
            "行李运输: 行李携带规则",
            "人身损害赔偿: 伤亡赔偿标准",
            "行李损害赔偿: 行李损失赔偿",
            "免责情形: 承运人免责条款",
            "运输安全: 安全运输保障",
        ]
    }

    /// 海事赔偿责任限制
    pub fn liability_limit(&self) -> Vec<&'static str> {
        vec![
            "责任限制权利: 海事赔偿限额",
            "责任限制基金: 设立基金程序",
            "限制性债权: 适用限制债权",
            "非限制性债权: 不适用限制",
            "责任限额计算: 限额计算方法",
            "责任限制程序: 申请限制程序",
            "责任限制效力: 限制生效条件",
            "责任限制丧失: 丧失限制情形",
        ]
    }

    /// 海难救助规则
    pub fn salvage_rules(&self) -> Vec<&'static str> {
        vec![
            "救助义务: 人命救助义务",
            "救助报酬: 救助报酬计算",
            "特别补偿: 环境损害补偿",
            "救助合同: 救助协议签订",
            "无效果无报酬原则",
            "救助效果认定: 救助成功认定",
            "报酬分配: 救助报酬分配",
            "救助争议: 争议处理程序",
        ]
    }

    /// 共同海损规则
    pub fn general_average(&self) -> Vec<&'static str> {
        vec![
            "共同海损定义: 共同危险牺牲",
            "共同海损构成要件",
            "共同海损牺牲: 共损牺牲范围",
            "共同海损费用: 共损费用范围",
            "共同海损分摊: 分摊计算方法",
            "共同海损理算: 理算程序规则",
            "共同海损担保: 担保提供要求",
            "共同海损宣告: 海损宣告程序",
        ]
    }

    /// 海事保险规则
    pub fn maritime_insurance(&self) -> Vec<&'static str> {
        vec![
            "船舶保险: 船舶损失保险",
            "货物保险: 货物运输保险",
            "责任保险: 海事责任保险",
            "保险合同: 海事保险合同",
            "保险标的: 保险标的范围",
            "保险责任: 承保责任范围",
            "保险索赔: 索赔理赔程序",
            "代位求偿: 保险代位权利",
        ]
    }
}

impl Default for MaritimeLawRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for MaritimeLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("maritime")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【海商法规则】\n\n船舶规则:\n{}\n\n货物运输:\n{}\n\n海难救助:\n{}\n",
            self.ship_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.cargo_transport().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.salvage_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maritime_law_rules() {
        let rules = MaritimeLawRules::new();
        assert!(!rules.ship_rules().is_empty());
        assert!(!rules.cargo_transport().is_empty());
    }
}