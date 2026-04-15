//! 价格法基础规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 价格法规则
pub struct PriceLawRules {
    metadata: RuleMetadata,
}

impl PriceLawRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "价格法规则",
                "中国价格法基础知识"
            )
            .with_origin("中国")
            .with_tags(vec!["法律".into(), "价格".into()]),
        }
    }

    /// 价格管理原则
    pub fn price_principles(&self) -> Vec<&'static str> {
        vec![
            "公平竞争原则",
            "市场调节原则",
            "政府调控原则",
            "信息公开原则",
            "合理定价原则",
            "价格稳定原则",
            "消费者保护原则",
            "经营者自律原则",
        ]
    }

    /// 价格形式
    pub fn price_forms(&self) -> Vec<&'static str> {
        vec![
            "市场调节价",
            "政府指导价",
            "政府定价",
            "垄断价格管理",
            "重要商品价格管制",
            "服务价格管理",
            "要素价格管理",
            "特殊行业价格管理",
        ]
    }

    /// 经营者价格行为
    pub fn operator_price_behavior(&self) -> Vec<&'static str> {
        vec![
            "定价自主权保障",
            "价格信息公示要求",
            "明码标价规定",
            "价格诚信义务",
            "价格竞争规则",
            "价格歧视禁止",
            "价格欺诈禁止",
            "价格垄断禁止",
        ]
    }

    /// 政府价格管理
    pub fn government_price_management(&self) -> Vec<&'static str> {
        vec![
            "政府定价范围规定",
            "政府定价程序规范",
            "政府定价听证制度",
            "价格调控措施",
            "价格监测预警",
            "价格干预措施",
            "价格紧急措施",
            "价格监督检查",
        ]
    }

    /// 价格听证规则
    pub fn price_hearing(&self) -> Vec<&'static str> {
        vec![
            "听证范围规定",
            "听证组织程序",
            "听证参与人选择",
            "听证信息公开",
            "听证意见采纳",
            "听证结果公告",
            "听证监督机制",
            "听证档案管理",
        ]
    }

    /// 价格监督检查
    pub fn price_supervision(&self) -> Vec<&'static str> {
        vec![
            "价格监督检查权",
            "价格违法查处程序",
            "价格举报投诉处理",
            "价格信息收集分析",
            "价格信用管理",
            "价格公示制度",
            "价格监测报告制度",
            "价格应急处理机制",
        ]
    }

    /// 价格违法行为
    pub fn price_violations(&self) -> Vec<&'static str> {
        vec![
            "价格欺诈违法行为",
            "价格垄断违法行为",
            "哄抬价格违法行为",
            "低价倾销违法行为",
            "价格歧视违法行为",
            "违反政府定价行为",
            "违反明码标价行为",
            "违反价格干预措施",
        ]
    }

    /// 价格法律责任
    pub fn price_legal_responsibility(&self) -> Vec<&'static str> {
        vec![
            "价格违法罚款处罚",
            "价格违法没收所得",
            "价格违法责令整改",
            "价格违法警告处罚",
            "价格违法吊销许可",
            "价格违法民事赔偿",
            "价格违法刑事责任",
            "价格违法信用惩戒",
        ]
    }
}

impl Default for PriceLawRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for PriceLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("price")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【价格法规则】\n\n价格原则:\n{}\n\n价格形式:\n{}\n\n价格违法行为:\n{}\n",
            self.price_principles().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.price_forms().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.price_violations().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_price_law_rules() {
        let rules = PriceLawRules::new();
        assert!(!rules.price_principles().is_empty());
        assert!(!rules.price_forms().is_empty());
    }
}
