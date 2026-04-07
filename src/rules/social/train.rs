//! 火车礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 火车礼仪规则
pub struct TrainEtiquette {
    metadata: RuleMetadata,
}

impl TrainEtiquette {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "火车礼仪",
                "乘坐火车礼仪规范"
            )
            .with_origin("通用")
            .with_tags(vec!["社交".into(), "火车".into()]),
        }
    }

    /// 乘车礼仪
    pub fn boarding_rules(&self) -> Vec<&'static str> {
        vec![
            "提前到站候车",
            "排队检票上车",
            "对号入座",
            "大件行李放行李架",
        ]
    }

    /// 座位礼仪
    pub fn seating_etiquette(&self) -> Vec<&'static str> {
        vec![
            "不占他人座位",
            "不横躺占多个座位",
            "腿不要伸到过道",
            "如需换座先征得同意",
        ]
    }

    /// 噪音控制
    pub fn noise_control(&self) -> Vec<&'static str> {
        vec![
            "手机静音或戴耳机",
            "通话到车厢连接处",
            "不要大声聊天",
            "看视频戴耳机",
        ]
    }

    /// 用餐礼仪
    pub fn dining_etiquette(&self) -> Vec<&'static str> {
        vec![
            "不要带气味重的食物",
            "用餐后清理垃圾",
            "不要在座位上饮酒",
        ]
    }

    /// 睡卧礼仪(卧铺)
    pub fn sleeping_etiquette(&self) -> Vec<&'static str> {
        vec![
            "熄灯后保持安静",
            "脚不要朝向过道",
            "整理好个人物品",
            "早上及时收起床铺",
        ]
    }

    /// 禁止行为
    pub fn prohibited_behaviors(&self) -> Vec<&'static str> {
        vec![
            "不要在车厢内吸烟",
            "不要脱鞋晾脚",
            "不要霸座",
            "不要携带违禁品",
        ]
    }
}

impl Default for TrainEtiquette {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for TrainEtiquette {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("train")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【火车礼仪】\n\n\
            乘车礼仪:\n{}\n\n\
            座位礼仪:\n{}\n\n\
            噪音控制:\n{}\n\n\
            禁止行为:\n{}\n",
            self.boarding_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.seating_etiquette().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.noise_control().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.prohibited_behaviors().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_train_etiquette() {
        let etiquette = TrainEtiquette::new();
        assert!(etiquette.boarding_rules().contains(&"对号入座"));
    }
}