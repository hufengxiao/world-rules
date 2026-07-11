//! 仰泳规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 仰泳规则
pub struct SwimmingBackstrokeRules {
    metadata: RuleMetadata,
}

impl SwimmingBackstrokeRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("仰泳规则", "仰泳技术规则详解")
                .with_origin("FINA")
                .with_tags(vec!["体育".into(), "游泳".into(), "仰泳".into()]),
        }
    }

    /// 基本技术
    pub fn technique(&self) -> Vec<&'static str> {
        vec![
            "身体保持仰卧",
            "胸部以上露出水面",
            "双臂交替划水",
            "双腿上下打水",
            "头部可左右转动呼吸",
            "身体可侧倾最多90°",
        ]
    }

    /// 出发规则
    pub fn starting(&self) -> Vec<&'static str> {
        vec![
            "水中出发",
            "面对池壁握住握手器",
            "双脚在水面下",
            "禁止站在排水沟上",
            "枪响后向后蹬壁",
            "出发后可潜泳15米",
        ]
    }

    /// 转身规则
    pub fn turning(&self) -> Vec<&'static str> {
        vec![
            "允许翻滚转身",
            "转身时可翻转成俯卧",
            "翻转后立即转身",
            "转身后可潜泳15米",
            "转身前最后一次划水后可翻转",
            "转身时身体可超过垂直",
        ]
    }

    /// 终点规则
    pub fn finish(&self) -> Vec<&'static str> {
        vec![
            "必须仰卧触壁",
            "可潜泳触壁",
            "禁止翻转触壁",
            "触壁时肩部水平",
            "禁止站立触壁",
        ]
    }

    /// 犯规行为
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "翻转超过90°后未转身",
            "站立或行走",
            "拉分道线",
            "潜泳超过15米",
            "转身后俯卧游进",
            "触壁时翻转身体",
        ]
    }

    /// 比赛距离
    pub fn distances(&self) -> Vec<&'static str> {
        vec!["50米仰泳", "100米仰泳", "200米仰泳", "混合泳中的仰泳段"]
    }

    /// 技术要点
    pub fn key_points(&self) -> Vec<&'static str> {
        vec![
            "保持身体水平",
            "快速连续打腿",
            "手臂高肘抱水",
            "身体微侧滚",
            "头部稳定",
            "节奏: 6次腿2次手",
        ]
    }
}

impl Default for SwimmingBackstrokeRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for SwimmingBackstrokeRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("swimming_backstroke")
    }

    fn explain(&self) -> String {
        format!(
            "【仰泳规则】\n\n\
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
