//! 电影院礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 电影院礼仪规则
pub struct CinemaEtiquette {
    metadata: RuleMetadata,
}

impl CinemaEtiquette {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "电影院礼仪",
                "观影礼仪规范"
            )
            .with_origin("通用")
            .with_tags(vec!["社交".into(), "电影".into()]),
        }
    }

    /// 入场礼仪
    pub fn entrance_rules(&self) -> Vec<&'static str> {
        vec![
            "提前入场，不要迟到",
            "对号入座",
            "从他人面前经过时要道歉",
            "不要带气味重的食物",
        ]
    }

    /// 观影礼仪
    pub fn viewing_etiquette(&self) -> Vec<&'static str> {
        vec![
            "手机静音或关机",
            "不要接打电话",
            "不要大声讨论剧情",
            "不要踢前排座椅",
            "不要频繁进出",
            "不要闪光拍照",
        ]
    }

    /// 饮食礼仪
    pub fn eating_etiquette(&self) -> Vec<&'static str> {
        vec![
            "安静进食",
            "不要大声嚼零食",
            "垃圾带离影厅",
            "不要带气味重的食物",
        ]
    }

    /// 散场礼仪
    pub fn exit_rules(&self) -> Vec<&'static str> {
        vec![
            "看完片尾字幕再离场",
            "有序排队离场",
            "带走自己的垃圾",
            "不要推搡拥挤",
        ]
    }

    /// 儿童观影
    pub fn children_rules(&self) -> Vec<&'static str> {
        vec![
            "选择适合儿童的电影",
            "管好孩子不要吵闹",
            "孩子哭闹时带离影厅",
            "教导孩子观影礼仪",
        ]
    }
}

impl Default for CinemaEtiquette {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for CinemaEtiquette {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("cinema")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【电影院礼仪】\n\n\
            入场礼仪:\n{}\n\n\
            观影礼仪:\n{}\n\n\
            饮食礼仪:\n{}\n\n\
            散场礼仪:\n{}\n",
            self.entrance_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.viewing_etiquette().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.eating_etiquette().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.exit_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cinema_etiquette() {
        let etiquette = CinemaEtiquette::new();
        assert!(etiquette.viewing_etiquette().contains(&"手机静音或关机"));
    }
}