//! 乘机礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 乘机礼仪规则
pub struct FlightEtiquette {
    metadata: RuleMetadata,
}

impl FlightEtiquette {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "乘机礼仪",
                "乘坐飞机礼仪规范"
            )
            .with_origin("通用")
            .with_tags(vec!["社交".into(), "飞行".into()]),
        }
    }

    /// 登机礼仪
    pub fn boarding_rules(&self) -> Vec<&'static str> {
        vec![
            "提前到达机场",
            "按座位号排队登机",
            "大件行李托运",
            "随身行李放头顶行李架或前排座椅下",
        ]
    }

    /// 座位礼仪
    pub fn seating_etiquette(&self) -> Vec<&'static str> {
        vec![
            "找到座位后尽快入座",
            "放倒座椅前先告知后排乘客",
            "不要踢前排座椅",
            "保持过道畅通",
        ]
    }

    /// 用餐礼仪
    pub fn dining_etiquette(&self) -> Vec<&'static str> {
        vec![
            "用餐时将座椅调直",
            "不要带气味重的食物",
            "收起小桌板",
        ]
    }

    /// 下机礼仪
    pub fn disembarkation_rules(&self) -> Vec<&'static str> {
        vec![
            "飞机停稳后再起身",
            "按顺序下机",
            "检查随身物品",
            "不要拥挤",
        ]
    }

    /// 禁止行为
    pub fn prohibited_behaviors(&self) -> Vec<&'static str> {
        vec![
            "不要在飞机上使用手机(起飞降落)",
            "不要在厕所吸烟",
            "不要大声喧哗",
            "不要醉酒乘机",
            "不要打开应急舱门",
        ]
    }

    /// 安全须知
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "系好安全带",
            "收起小桌板(起飞降落)",
            "打开遮光板(起飞降落)",
            "了解紧急出口位置",
            "阅读安全须知卡",
        ]
    }
}

impl Default for FlightEtiquette {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for FlightEtiquette {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("flight")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【乘机礼仪】\n\n\
            登机礼仪:\n{}\n\n\
            座位礼仪:\n{}\n\n\
            安全须知:\n{}\n\n\
            禁止行为:\n{}\n",
            self.boarding_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.seating_etiquette().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.safety_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.prohibited_behaviors().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flight_etiquette() {
        let etiquette = FlightEtiquette::new();
        assert!(etiquette.safety_rules().contains(&"系好安全带"));
    }
}