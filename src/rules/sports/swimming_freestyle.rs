//! 自由泳规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 自由泳规则
pub struct SwimmingFreestyleRules {
    metadata: RuleMetadata,
}

impl SwimmingFreestyleRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("自由泳规则", "自由泳技术规则详解")
                .with_origin("FINA")
                .with_tags(vec!["体育".into(), "游泳".into(), "自由泳".into()]),
        }
    }

    /// 基本定义
    pub fn definition(&self) -> Vec<&'static str> {
        vec![
            "可使用任何泳姿",
            "通常采用爬泳 (Crawl)",
            "混合泳自由泳段: 不可用蝶/仰/蛙泳",
            "最快速的泳姿",
            "最常见的比赛项目",
        ]
    }

    /// 基本技术
    pub fn technique(&self) -> Vec<&'static str> {
        vec![
            "身体俯卧水中",
            "双臂交替划水",
            "双腿上下打水",
            "侧向呼吸",
            "身体绕纵轴滚动",
            "保持流线型",
        ]
    }

    /// 出发规则
    pub fn starting(&self) -> Vec<&'static str> {
        vec![
            "跳台出发",
            "出发姿势: 单脚或双脚",
            "枪响后跳水",
            "出发后可潜泳15米",
            "抢跳: 取消资格",
            "反应时间监测",
        ]
    }

    /// 转身规则
    pub fn turning(&self) -> Vec<&'static str> {
        vec![
            "允许滚翻转身",
            "转身时身体可翻转",
            "脚必须触壁",
            "转身后可潜泳15米",
            "转身后仰卧或俯卧均可",
            "禁止拉分道线转身",
        ]
    }

    /// 终点规则
    pub fn finish(&self) -> Vec<&'static str> {
        vec![
            "可单手触壁",
            "可潜泳触壁",
            "允许最后冲刺不呼吸",
            "触壁时身体任何部位",
        ]
    }

    /// 犯规行为
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "潜泳超过15米",
            "行走或跳跃池底",
            "拉分道线",
            "干扰其他泳道",
            "抢跳",
            "未完成距离",
            "转身时未触壁",
        ]
    }

    /// 比赛距离
    pub fn distances(&self) -> Vec<&'static str> {
        vec![
            "50米自由泳",
            "100米自由泳",
            "200米自由泳",
            "400米自由泳",
            "800米自由泳",
            "1500米自由泳",
            "混合泳中的自由泳段",
        ]
    }

    /// 技术要点
    pub fn key_points(&self) -> Vec<&'static str> {
        vec![
            "高肘抱水",
            "快速划水",
            "连续打腿: 6次腿/2次手",
            "侧向呼吸: 每2-3次划水",
            "身体滚动",
            "手臂充分前伸",
            "保持流线型",
        ]
    }

    /// 世界纪录
    pub fn records(&self) -> Vec<&'static str> {
        vec![
            "男子100米: 46.86秒 (2024)",
            "女子100米: 51.71秒 (2024)",
            "男子200米: 1:42.00 (2024)",
            "女子200米: 1:52.00 (2024)",
            "男子400米: 3:40.00 (2024)",
            "女子400米: 3:56.00 (2024)",
        ]
    }
}

impl Default for SwimmingFreestyleRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for SwimmingFreestyleRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("swimming_freestyle")
    }

    fn explain(&self) -> String {
        format!(
            "【自由泳规则】\n\n\
            基本定义:\n{}\n\n\
            基本技术:\n{}\n\n\
            转身规则:\n{}\n\n\
            犯规行为:\n{}",
            self.definition()
                .iter()
                .map(|d| format!("  • {}", d))
                .collect::<Vec<_>>()
                .join("\n"),
            self.technique()
                .iter()
                .map(|t| format!("  • {}", t))
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
