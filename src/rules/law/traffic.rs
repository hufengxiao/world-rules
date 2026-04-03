//! 交通规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 交通规则地区
#[derive(Debug, Clone)]
pub enum TrafficRegion {
    China,
    USA,
    UK,
    Japan,
}

/// 交通规则
pub struct TrafficRules {
    metadata: RuleMetadata,
    region: TrafficRegion,
}

impl TrafficRules {
    pub fn new(region: TrafficRegion) -> Self {
        Self {
            metadata: RuleMetadata::new(
                format!("{}交通规则", Self::region_name(&region)),
                "道路交通法规"
            )
            .with_origin(Self::region_name(&region)),
            region,
        }
    }

    fn region_name(region: &TrafficRegion) -> &'static str {
        match region {
            TrafficRegion::China => "中国",
            TrafficRegion::USA => "美国",
            TrafficRegion::UK => "英国",
            TrafficRegion::Japan => "日本",
        }
    }

    /// 驾驶方向
    pub fn driving_side(&self) -> &'static str {
        match self.region {
            TrafficRegion::China | TrafficRegion::USA => "右侧通行",
            TrafficRegion::UK | TrafficRegion::Japan => "左侧通行",
        }
    }

    /// 限速规则 (城市道路)
    pub fn city_speed_limit(&self) -> u8 {
        match self.region {
            TrafficRegion::China => 60,
            TrafficRegion::USA => 55,
            TrafficRegion::UK => 50,
            TrafficRegion::Japan => 50,
        }
    }

    /// 高速公路限速
    pub fn highway_speed_limit(&self) -> u16 {
        match self.region {
            TrafficRegion::China => 120,
            TrafficRegion::USA => 120,
            TrafficRegion::UK => 110,
            TrafficRegion::Japan => 100,
        }
    }

    /// 酒驾标准 (血液酒精浓度 mg/100ml)
    pub fn drunk_drive_limit(&self) -> u8 {
        match self.region {
            TrafficRegion::China => 20, // 中国: 20mg/100ml 为酒驾
            TrafficRegion::USA => 80,
            TrafficRegion::UK => 80,
            TrafficRegion::Japan => 30,
        }
    }

    /// 基本交通规则
    pub fn basic_rules(&self) -> Vec<&'static str> {
        vec![
            "红灯停、绿灯行",
            "遵守限速规定",
            "礼让行人",
            "禁止酒驾",
            "系好安全带",
            "禁止超载",
        ]
    }

    /// 信号灯含义
    pub fn traffic_light_meanings(&self) -> Vec<&'static str> {
        vec![
            "红灯: 禁止通行",
            "绿灯: 允许通行",
            "黄灯: 警示，准备停止",
            "红灯闪烁: 注意减速",
        ]
    }
}

impl Rule for TrafficRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("traffic")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【{}交通规则】\n\n\
            驾驶方向: {}\n\
            城市限速: {} km/h\n\
            高速限速: {} km/h\n\
            酒驾标准: {} mg/100ml\n\n\
            基本规则:\n{}\n\n\
            信号灯含义:\n{}\n",
            Self::region_name(&self.region),
            self.driving_side(),
            self.city_speed_limit(),
            self.highway_speed_limit(),
            self.drunk_drive_limit(),
            self.basic_rules().iter().map(|s| format!("  • {}", s)).collect::<Vec<_>>().join("\n"),
            self.traffic_light_meanings().iter().map(|s| format!("  • {}", s)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_china_traffic() {
        let rules = TrafficRules::new(TrafficRegion::China);
        assert_eq!(rules.driving_side(), "右侧通行");
        assert_eq!(rules.highway_speed_limit(), 120);
    }
}