//! 摔跤规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 摔跤规则
pub struct WrestlingRules {
    metadata: RuleMetadata,
}

impl WrestlingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "摔跤规则",
                "摔跤比赛基本规则"
            )
            .with_origin("古代奥运会")
            .with_tags(vec!["体育".into(), "格斗".into()]),
        }
    }

    /// 比赛类型
    pub fn wrestling_styles(&self) -> Vec<&'static str> {
        vec![
            "自由式摔跤: 可攻击全身",
            "古典式摔跤: 只能攻击腰部以上",
            "女子摔跤: 自由式规则",
            "奥运会正式项目",
            "按体重分级",
        ]
    }

    /// 比赛时间
    pub fn match_duration(&self) -> Vec<&'static str> {
        vec![
            "比赛分为两局",
            "每局3分钟",
            "局间休息30秒",
            "总分高者获胜",
            "技术优势领先10分获胜",
        ]
    }

    /// 得分规则
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "1分: 小动作摔倒对手",
            "2分: 中等动作摔倒对手",
            "4分: 大动作摔倒对手",
            "5分: 高难度动作摔倒对手",
            "压肩双肩着地直接获胜",
        ]
    }

    /// 体重级别
    pub fn weight_classes(&self) -> Vec<&'static str> {
        vec![
            "男子自由式: 57kg-125kg共10级",
            "男子古典式: 60kg-130kg共10级",
            "女子自由式: 50kg-76kg共6级",
            "赛前称重",
            "体重合格才能参赛",
        ]
    }

    /// 有效动作
    pub fn valid_moves(&self) -> Vec<&'static str> {
        vec![
            "摔: 将对手摔倒",
            "抱: 控制对手身体",
            "翻: 将对手翻转",
            "压: 压制对手",
            "投: 大幅度摔法",
        ]
    }

    /// 犯规行为
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "抓衣服",
            "咬人",
            "掐人",
            "攻击要害部位",
            "危险动作",
        ]
    }

    /// 比赛场地
    pub fn competition_area(&self) -> Vec<&'static str> {
        vec![
            "比赛区: 直径9米圆形",
            "中心圆: 直径1米",
            "保护垫: 围绕比赛区",
            "摔跤垫: 厚度符合标准",
            "红色和蓝色区域区分选手",
        ]
    }
}

impl Default for WrestlingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for WrestlingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("wrestling")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【摔跤规则】\n\n\
            比赛类型:\n{}\n\n\
            得分规则:\n{}\n\n\
            有效动作:\n{}\n\n\
            犯规行为:\n{}\n",
            self.wrestling_styles().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.valid_moves().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.fouls().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrestling_rules() {
        let rules = WrestlingRules::new();
        assert!(!rules.wrestling_styles().is_empty());
    }
}