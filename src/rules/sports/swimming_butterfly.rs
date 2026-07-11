//! 蝶泳规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 蝶泳规则
pub struct SwimmingButterflyRules {
    metadata: RuleMetadata,
}

impl SwimmingButterflyRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("蝶泳规则", "蝶泳技术规则详解")
                .with_origin("FINA")
                .with_tags(vec!["体育".into(), "游泳".into(), "蝶泳".into()]),
        }
    }

    /// 基本技术
    pub fn technique(&self) -> Vec<&'static str> {
        vec![
            "双臂同时向前挥动",
            "双腿同时上下打水 (海豚腿)",
            "身体保持俯卧",
            "肩部与水面平行",
            "头部可在水下呼吸",
            "每次划水后头须露出水面",
        ]
    }

    /// 出发规则
    pub fn starting(&self) -> Vec<&'static str> {
        vec![
            "跳台出发",
            "出发后可潜泳15米",
            "出发后允许多次海豚腿",
            "第一次划水前须出水",
            "抢跳: 取消资格",
        ]
    }

    /// 转身规则
    pub fn turning(&self) -> Vec<&'static str> {
        vec![
            "必须双手同时触壁",
            "触壁前最后一次划水后可潜泳",
            "转身后可潜泳15米",
            "允许翻滚转身",
            "转身后可打海豚腿",
        ]
    }

    /// 终点规则
    pub fn finish(&self) -> Vec<&'static str> {
        vec![
            "必须双手同时触壁",
            "触壁后停止计时",
            "肩部须保持水平",
            "允许最后冲刺不呼吸",
        ]
    }

    /// 犯规行为
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "单手触壁",
            "双手不同时触壁",
            "使用蛙泳腿",
            "使用爬泳腿",
            "潜泳超过15米",
            "身体翻转超过90°",
            "手臂不同时划水",
        ]
    }

    /// 比赛距离
    pub fn distances(&self) -> Vec<&'static str> {
        vec![
            "50米蝶泳 (短池)",
            "100米蝶泳",
            "200米蝶泳",
            "混合泳中的蝶泳段",
        ]
    }

    /// 技术要点
    pub fn key_points(&self) -> Vec<&'static str> {
        vec![
            "节奏: 2次腿1次手",
            "呼吸: 每2次划水呼吸1次",
            "身体波浪运动",
            "手臂高肘划水",
            "快速有力打腿",
            "保持身体流线型",
        ]
    }
}

impl Default for SwimmingButterflyRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for SwimmingButterflyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("swimming_butterfly")
    }

    fn explain(&self) -> String {
        format!(
            "【蝶泳规则】\n\n\
            基本技术:\n{}\n\n\
            出发规则:\n{}\n\n\
            转身规则:\n{}\n\n\
            犯规行为:\n{}",
            self.technique()
                .iter()
                .map(|t| format!("  • {}", t))
                .collect::<Vec<_>>()
                .join("\n"),
            self.starting()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.turning()
                .iter()
                .map(|t| format!("  • {}", t))
                .collect::<Vec<_>>()
                .join("\n"),
            self.fouls()
                .iter()
                .map(|f| format!("  • {}", f))
                .collect::<Vec<_>>()
                .join("\n"),
        )
    }
}
