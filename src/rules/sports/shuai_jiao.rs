//! 中国摔跤规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 中国摔跤规则
pub struct ShuaiJiaoRules {
    metadata: RuleMetadata,
}

impl ShuaiJiaoRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("中国摔跤规则", "中国摔跤比赛基本规则")
                .with_origin("中国")
                .with_tags(vec!["体育".into(), "武术".into(), "摔跤".into()]),
        }
    }

    /// 比赛时间
    pub fn match_duration(&self) -> Vec<&'static str> {
        vec![
            "正式比赛: 每局3分钟",
            "决赛比赛: 每局5分钟",
            "休息时间: 局间1分钟",
            "加时赛: 平局时进行",
            "青少年: 时间缩短",
        ]
    }

    /// 体重级别
    pub fn weight_classes(&self) -> Vec<&'static str> {
        vec![
            "男子: 52kg, 56kg, 62kg, 68kg, 74kg, 82kg, 90kg, 100kg, +100kg",
            "女子: 48kg, 52kg, 56kg, 60kg, 65kg, 70kg, 75kg",
            "青少年级别: 年龄分组",
            "业余级别: 体重分组",
            "职业级别: 更细划分",
        ]
    }

    /// 得分标准
    pub fn scoring_criteria(&self) -> Vec<&'static str> {
        vec![
            "一本(4分): 完美摔倒对手",
            "有技(3分): 大幅度摔倒",
            "有效(2分): 有效摔倒",
            "效果(1分): 小幅度摔倒",
            "无效: 无得分动作",
        ]
    }

    /// 有效技术
    pub fn valid_techniques(&self) -> Vec<&'static str> {
        vec![
            "揣跤: 抱腰摔倒",
            "勾子: 勾腿摔",
            "别子: 别腿摔",
            "入跤: 进身摔",
            "踢跤: 踢腿摔",
            "切子: 切腿摔",
            "抱腿摔: 抱腿技术",
            "过胸摔: 大幅动作",
        ]
    }

    /// 禁止行为
    pub fn prohibited_actions(&self) -> Vec<&'static str> {
        vec![
            "故意伤害对手",
            "攻击危险部位",
            "锁喉窒息动作",
            "故意拖延比赛",
            "使用非法技术",
            "消极比赛行为",
            "不服从裁判",
        ]
    }

    /// 比赛服装
    pub fn uniform_requirements(&self) -> Vec<&'static str> {
        vec![
            "跤衣: 传统摔跤服",
            "跤裤: 专用裤子",
            "跤鞋: 专用摔跤鞋",
            "腰带: 级别标识",
            "护具: 安全装备",
        ]
    }

    /// 场地要求
    pub fn competition_area(&self) -> Vec<&'static str> {
        vec![
            "比赛区: 直径9米圆形",
            "安全区: 外围2米宽度",
            "场地材质: 摔跤垫",
            "场地标识: 区域划分",
            "观众距离: 安全距离",
        ]
    }

    /// 裁判制度
    pub fn referee_system(&self) -> Vec<&'static str> {
        vec![
            "主裁判: 场上裁判",
            "副裁判: 边裁辅助",
            "裁判长: 评分监督",
            "计时员: 时间记录",
            "记录员: 成绩记录",
        ]
    }
}

impl Default for ShuaiJiaoRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ShuaiJiaoRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("shuai_jiao")
    }

    fn explain(&self) -> String {
        format!(
            "【中国摔跤规则】\n\n\
            比赛时间:\n{}\n\n\
            得分标准:\n{}\n\n\
            有效技术:\n{}\n\n\
            禁止行为:\n{}\n\n\
            场地要求:\n{}\n",
            self.match_duration()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.scoring_criteria()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.valid_techniques()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.prohibited_actions()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.competition_area()
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
    fn test_shuai_jiao_rules() {
        let rules = ShuaiJiaoRules::new();
        assert!(!rules.match_duration().is_empty());
        assert!(!rules.weight_classes().is_empty());
        assert!(!rules.scoring_criteria().is_empty());
    }

    #[test]
    fn test_shuai_jiao_techniques() {
        let rules = ShuaiJiaoRules::new();
        let techniques = rules.valid_techniques();
        assert!(techniques.contains(&"揣跤: 抱腰摔倒"));
        assert!(techniques.contains(&"勾子: 勾腿摔"));
    }
}