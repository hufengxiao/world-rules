//! 经济学定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 经济学规则
pub struct EconomicsRules {
    metadata: RuleMetadata,
}

impl EconomicsRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "经济学定律",
                "经济学基本定律和理论"
            )
            .with_origin("经济学")
            .with_tags(vec!["科学".into(), "经济学".into()]),
        }
    }

    /// 供需定律
    pub fn supply_demand_law(&self) -> Vec<&'static str> {
        vec![
            "需求定律: 价格上升，需求量下降",
            "供给定律: 价格上升，供给量增加",
            "均衡价格: 供给等于需求时的价格",
            "市场机制: 价格调节供需",
        ]
    }

    /// 边际效用递减
    pub fn diminishing_marginal_utility(&self) -> Vec<&'static str> {
        vec![
            "每增加一单位消费，效用增量递减",
            "解释钻石与水的悖论",
            "消费者均衡的条件",
        ]
    }

    /// 比较优势理论
    pub fn comparative_advantage(&self) -> Vec<&'static str> {
        vec![
            "每个国家应专业化生产比较优势产品",
            "贸易对双方都有利",
            "解释国际贸易的基础",
        ]
    }

    /// 帕累托效率
    pub fn pareto_efficiency(&self) -> Vec<&'static str> {
        vec![
            "资源重新配置无法使一人变好而不使他人变坏",
            "帕累托改进: 使一人变好而不使他人变坏",
            "市场均衡是帕累托有效的",
        ]
    }

    /// 机会成本
    pub fn opportunity_cost(&self) -> Vec<&'static str> {
        vec![
            "选择的机会成本是放弃的最佳替代选择",
            "用于评估决策的价值",
            "体现了资源的稀缺性",
        ]
    }

    /// 经济周期
    pub fn business_cycle(&self) -> Vec<&'static str> {
        vec![
            "繁荣: 经济增长，就业增加",
            "衰退: 经济下滑，失业增加",
            "萧条: 经济严重下滑",
            "复苏: 经济开始回升",
        ]
    }

    /// 货币理论
    pub fn monetary_theory(&self) -> Vec<&'static str> {
        vec![
            "MV = PQ (货币数量方程)",
            "M: 货币供应量",
            "V: 货币流通速度",
            "P: 价格水平",
            "Q: 实际产出",
        ]
    }
}

impl Default for EconomicsRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for EconomicsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("economics")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【经济学定律】\n\n\
            供需定律:\n{}\n\n\
            边际效用递减:\n{}\n\n\
            比较优势理论:\n{}\n\n\
            帕累托效率:\n{}\n\n\
            机会成本:\n{}\n\n\
            经济周期:\n{}\n",
            self.supply_demand_law().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.diminishing_marginal_utility().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.comparative_advantage().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.pareto_efficiency().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.opportunity_cost().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.business_cycle().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_economics_rules() {
        let rules = EconomicsRules::new();
        assert!(!rules.supply_demand_law().is_empty());
    }
}