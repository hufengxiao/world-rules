//! 水法基础规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 水法规则
pub struct WaterLawRules {
    metadata: RuleMetadata,
}

impl WaterLawRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "水法规则",
                "中国水法基础知识"
            )
            .with_origin("中国")
            .with_tags(vec!["法律".into(), "水资源".into()]),
        }
    }

    /// 水资源管理原则
    pub fn management_principles(&self) -> Vec<&'static str> {
        vec![
            "节约用水原则",
            "水资源保护原则",
            "统一管理原则",
            "合理开发原则",
            "综合利用原则",
            "有偿使用原则",
            "供需平衡原则",
            "可持续发展原则",
        ]
    }

    /// 水资源规划
    pub fn water_planning(&self) -> Vec<&'static str> {
        vec![
            "水资源综合规划",
            "流域规划: 河流规划",
            "区域规划: 地区规划",
            "专项规划: 专门规划",
            "水资源评价",
            "水资源配置规划",
            "水源地保护规划",
            "水资源监测规划",
        ]
    }

    /// 水资源开发利用
    pub fn water_utilization(&self) -> Vec<&'static str> {
        vec![
            "水资源开发许可",
            "水资源利用审批",
            "水资源取用规则",
            "水利工程建设",
            "水资源调度",
            "跨流域调水",
            "水资源综合利用",
            "水资源开发限制",
        ]
    }

    /// 水资源保护
    pub fn water_protection(&self) -> Vec<&'static str> {
        vec![
            "水源地保护",
            "水域功能区划",
            "水质保护标准",
            "水量保护措施",
            "水生态保护",
            "水污染防治",
            "水土保持",
            "地下水保护",
        ]
    }

    /// 取水许可制度
    pub fn water_intake_permit(&self) -> Vec<&'static str> {
        vec![
            "取水许可申请",
            "取水许可审批",
            "取水许可条件",
            "取水许可期限",
            "取水许可变更",
            "取水许可撤销",
            "取水许可年检",
            "取水许可监管",
        ]
    }

    /// 水资源费征收
    pub fn water_fee_collection(&self) -> Vec<&'static str> {
        vec![
            "水资源费征收对象",
            "水资源费征收标准",
            "水资源费计算方法",
            "水资源费缴纳程序",
            "水资源费减免情形",
            "水资源费使用管理",
            "水资源费监督检查",
            "水资源费违法行为",
        ]
    }

    /// 洪水防治
    pub fn flood_control(&self) -> Vec<&'static str> {
        vec![
            "防洪规划制定",
            "防洪工程建设",
            "防洪应急预案",
            "防汛指挥调度",
            "洪水预警预报",
            "防洪物资储备",
            "防洪责任制度",
            "防洪违法行为",
        ]
    }

    /// 水事违法行为
    pub fn water_violations(&self) -> Vec<&'static str> {
        vec![
            "无证取水违法行为",
            "超量取水违法行为",
            "破坏水利工程行为",
            "污染水资源行为",
            "侵占水域违法行为",
            "妨碍防洪违法行为",
            "拒不缴纳水资源费",
            "破坏水源地行为",
        ]
    }
}

impl Default for WaterLawRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for WaterLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("water")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【水法规则】\n\n管理原则:\n{}\n\n水资源保护:\n{}\n\n取水许可:\n{}\n",
            self.management_principles().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.water_protection().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.water_intake_permit().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_water_law_rules() {
        let rules = WaterLawRules::new();
        assert!(!rules.management_principles().is_empty());
        assert!(!rules.water_protection().is_empty());
    }
}