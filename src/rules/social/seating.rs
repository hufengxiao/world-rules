//! 座次礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 座次礼仪
pub struct SeatingEtiquette {
    metadata: RuleMetadata,
}

impl SeatingEtiquette {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "座次礼仪",
                "中国传统座次礼仪规范"
            )
            .with_origin("中国")
            .with_tags(vec!["社交".into(), "餐饮".into()]),
        }
    }

    /// 宴会座次原则
    pub fn banquet_seating(&self) -> Vec<&'static str> {
        vec![
            "主座正对门口",
            "主宾坐主人右侧",
            "副主宾坐主人左侧",
            "主人对面为副主人",
            "按身份地位依次排列",
        ]
    }

    /// 会议座次
    pub fn meeting_seating(&self) -> Vec<&'static str> {
        vec![
            "主持人或领导居中",
            "重要嘉宾在领导右侧",
            "按职位高低排列",
            "记录员在侧面或后排",
            "参会人员按部门或职能排列",
        ]
    }

    /// 车辆座次
    pub fn car_seating(&self) -> Vec<&'static str> {
        vec![
            "轿车: 后排右座为尊",
            "后排左座次之",
            "副驾驶座最低",
            "主人驾车时副驾为尊",
            "越野车: 副驾为尊(视野好)",
        ]
    }

    /// 电梯站位
    pub fn elevator_position(&self) -> Vec<&'static str> {
        vec![
            "领导或长辈在电梯内侧",
            "操作按钮者靠近控制面板",
            "客人先进先出",
            "拥挤时让出空间",
            "勿堵住门口",
        ]
    }

    /// 合影站位
    pub fn photo_position(&self) -> Vec<&'static str> {
        vec![
            "领导居中",
            "重要嘉宾在领导两侧",
            "按职位高低向两侧排列",
            "女士可在前排",
            "长辈优先站前排",
        ]
    }

    /// 走路顺序
    pub fn walking_order(&self) -> Vec<&'static str> {
        vec![
            "两人并行: 内侧为尊",
            "两人前后: 前面为尊",
            "多人同行: 中间为尊",
            "进门出门: 客人先行",
            "上下楼梯: 长辈先上后下",
        ]
    }

    /// 住宅座次
    pub fn home_seating(&self) -> Vec<&'static str> {
        vec![
            "主人坐靠近门口位置(便于招待)",
            "客人坐主位",
            "长辈坐尊位",
            "避免让客人坐背对门的位置",
            "茶几周围按尊卑排列",
        ]
    }
}

impl Default for SeatingEtiquette {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for SeatingEtiquette {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("seating")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【座次礼仪】\n\n\
            宴会座次:\n{}\n\n\
            会议座次:\n{}\n\n\
            车辆座次:\n{}\n\n\
            走路顺序:\n{}\n",
            self.banquet_seating().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.meeting_seating().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.car_seating().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.walking_order().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seating_etiquette() {
        let rules = SeatingEtiquette::new();
        assert!(!rules.banquet_seating().is_empty());
    }
}