//! 松涛馆空手道规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 松涛馆空手道规则
pub struct ShotokanRules {
    metadata: RuleMetadata,
}

impl ShotokanRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("松涛馆空手道规则", "松涛馆空手道比赛基本规则")
                .with_origin("日本")
                .with_tags(vec!["体育".into(), "武术".into(), "空手道".into()]),
        }
    }

    /// 比赛形式
    pub fn competition_types(&self) -> Vec<&'static str> {
        vec![
            "套路比赛: 型表演评分",
            "组手比赛: 对抗比赛",
            "团体比赛: 团队竞赛",
            "综合比赛: 两项结合",
            "表演赛: 技术展示",
        ]
    }

    /// 基本套路
    pub fn kata(&self) -> Vec<&'static str> {
        vec![
            "平安初段至五段: 基础套路",
            "铁骑初段: 进阶套路",
            "拔塞: 高级套路",
            "观空: 最高级套路",
            "慈恩: 传统套路",
        ]
    }

    /// 组手规则
    pub fn kumite_rules(&self) -> Vec<&'static str> {
        vec![
            "定点组手: 预定动作",
            "基本组手: 基础对抗",
            "一本组手: 单次攻击",
            "三本组手: 三次攻击",
            "自由组手: 实战对抗",
        ]
    }

    /// 得分标准
    pub fn scoring_criteria(&self) -> Vec<&'static str> {
        vec![
            "一本(3分): 完美技术",
            "技有(2分): 有效技术",
            "有效(1分): 基本效果",
            "无效: 无得分动作",
            "判定胜: 比赛结束评分",
        ]
    }

    /// 比赛时间
    pub fn match_duration(&self) -> Vec<&'static str> {
        vec![
            "组手正赛: 3分钟",
            "组手决赛: 4分钟",
            "套路时间: 3-5分钟",
            "休息时间: 局间1分钟",
            "加时赛: 平局进行",
        ]
    }

    /// 允许技术
    pub fn permitted_techniques(&self) -> Vec<&'static str> {
        vec![
            "冲拳: 直拳攻击",
            "逆拳: 反拳攻击",
            "前踢: 前腿踢击",
            "侧踢: 横向踢击",
            "回旋踢: 转身踢击",
        ]
    }

    /// 禁止行为
    pub fn prohibited_actions(&self) -> Vec<&'static str> {
        vec![
            "过度接触: 重击对手",
            "攻击危险部位",
            "危险投技",
            "消极比赛",
            "不尊重裁判",
            "故意拖延",
        ]
    }

    /// 段位制度
    pub fn ranking_system(&self) -> Vec<&'static str> {
        vec![
            "初段至十段: 黑带等级",
            "一级至十级: 彩带等级",
            "考核要求: 技术和套路",
            "年限规定: 段位晋升时间",
            "考试制度: 定期考核",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "佩戴护具: 组手必备",
            "控制接触: 安全打击",
            "医疗检查: 赛前体检",
            "裁判监督: 比赛安全",
            "保险要求: 比赛保险",
        ]
    }
}

impl Default for ShotokanRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ShotokanRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("shotokan")
    }

    fn explain(&self) -> String {
        format!(
            "【松涛馆空手道规则】\n\n\
            比赛形式:\n{}\n\n\
            基本套路:\n{}\n\n\
            组手规则:\n{}\n\n\
            得分标准:\n{}\n\n\
            安全规则:\n{}\n",
            self.competition_types()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.kata()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.kumite_rules()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.scoring_criteria()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.safety_rules()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shotokan_rules() {
        let rules = ShotokanRules::new();
        assert!(!rules.competition_types().is_empty());
        assert!(!rules.kata().is_empty());
        assert!(!rules.kumite_rules().is_empty());
    }

    #[test]
    fn test_shotokan_kata() {
        let rules = ShotokanRules::new();
        let katas = rules.kata();
        assert!(katas.contains(&"平安初段至五段: 基础套路"));
        assert!(katas.contains(&"观空: 最高级套路"));
    }
}
