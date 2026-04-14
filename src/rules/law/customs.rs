//! 海关法基础规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 海关法规则
pub struct CustomsLawRules {
    metadata: RuleMetadata,
}

impl CustomsLawRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "海关法规则",
                "中国海关法基础知识"
            )
            .with_origin("中国")
            .with_tags(vec!["法律".into(), "海关".into()]),
        }
    }

    /// 海关监管
    pub fn customs_supervision(&self) -> Vec<&'static str> {
        vec![
            "进出境监管: 海关监管职责",
            "监管区域: 海关监管区域",
            "监管场所: 海关监管场所",
            "监管货物: 海关监管货物",
            "监管运输: 运输工具监管",
            "监管人员: 人员进出监管",
            "监管物品: 个人物品监管",
            "监管邮件: 邮递物品监管",
        ]
    }

    /// 进出境货物规则
    pub fn cargo_rules(&self) -> Vec<&'static str> {
        vec![
            "货物申报: 进出口申报",
            "货物查验: 海关查验程序",
            "货物征税: 关税征收",
            "货物放行: 海关放行",
            "货物结关: 海关结关",
            "货物保税: 保税货物管理",
            "货物减免税: 减免税审批",
            "货物通关: 通关便利措施",
        ]
    }

    /// 进出境运输工具规则
    pub fn transport_rules(&self) -> Vec<&'static str> {
        vec![
            "船舶进出境: 船舶海关监管",
            "航空器进出境: 飞机海关监管",
            "车辆进出境: 车辆海关监管",
            "运输工具申报: 运载工具申报",
            "舱单管理: 舱单申报制度",
            "运输工具查验: 运载工具检查",
            "物料供应: 供应物料管理",
            "运输工具结关: 运载工具结关",
        ]
    }

    /// 进出境物品规则
    pub fn articles_rules(&self) -> Vec<&'static str> {
        vec![
            "个人物品: 个人自用物品",
            "行李物品: 进出境行李",
            "邮递物品: 邮寄物品管理",
            "快件物品: 快递物品管理",
            "物品申报: 个人物品申报",
            "物品查验: 个人物品查验",
            "物品征税: 个人物品征税",
            "物品免税额度: 免税限额规定",
        ]
    }

    /// 关税征收规则
    pub fn duty_collection(&self) -> Vec<&'static str> {
        vec![
            "关税计征: 关税计算征收",
            "完税价格: 完税价格确定",
            "税率适用: 税率适用规则",
            "关税缴纳: 税款缴纳程序",
            "关税减免: 减免税管理",
            "关税退税: 税款退还",
            "关税追征: 税款追补",
            "滞纳金征收: 滞纳金计算",
        ]
    }

    /// 保税制度
    pub fn bonded_system(&self) -> Vec<&'static str> {
        vec![
            "保税区: 保税区管理",
            "保税仓库: 保税仓运营",
            "保税加工: 加工贸易保税",
            "保税物流: 保税物流业务",
            "保税展示: 保税展示交易",
            "保税监管: 保税货物监管",
            "保税核销: 保税账册核销",
            "保税担保: 保税担保制度",
        ]
    }

    /// 海关执法权限
    pub fn customs_authority(&self) -> Vec<&'static str> {
        vec![
            "检查权: 海关检查权限",
            "查验权: 货物查验权限",
            "扣留权: 物品扣留权限",
            "查封权: 财产查封权限",
            "询问权: 案件询问权限",
            "调查权: 违法调查权限",
            "处罚权: 行政处罚权限",
            "强制权: 强制执行权限",
        ]
    }

    /// 海关违法行为
    pub fn customs_violations(&self) -> Vec<&'static str> {
        vec![
            "走私行为: 走私违法犯罪",
            "违规行为: 海关违规行为",
            "逃税行为: 逃避关税行为",
            "瞒报行为: 申报瞒报行为",
            "夹带行为: 物品夹带行为",
            "伪报行为: 申报伪报行为",
            "走私处罚: 走私处罚规定",
            "违规处罚: 违规处罚措施",
        ]
    }
}

impl Default for CustomsLawRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for CustomsLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("customs")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【海关法规则】\n\n海关监管:\n{}\n\n货物规则:\n{}\n\n关税征收:\n{}\n",
            self.customs_supervision().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.cargo_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.duty_collection().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_customs_law_rules() {
        let rules = CustomsLawRules::new();
        assert!(!rules.customs_supervision().is_empty());
        assert!(!rules.cargo_rules().is_empty());
    }
}