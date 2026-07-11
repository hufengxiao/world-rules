//! 冲浪竞赛规则
//!
//! 冲浪是世界泳联正式竞赛项目，
//! 也是奥运会正式比赛项目(2020东京奥运会首次引入)。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 冲浪竞赛规则
pub struct SurfingCompetitionRules {
    metadata: RuleMetadata,
}

impl SurfingCompetitionRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("冲浪竞赛规则", "世界泳联冲浪竞赛规则")
                .with_origin("World Aquatics / ISA")
                .with_tags(vec!["体育".into(), "水上".into(), "冲浪".into()]),
        }
    }

    /// 比赛项目
    pub fn events(&self) -> Vec<&'static str> {
        vec![
            "男子短板冲浪",
            "女子短板冲浪",
            "男子长板冲浪",
            "女子长板冲浪",
            "混合团体赛",
            "大浪冲浪赛",
            "青年组比赛",
        ]
    }

    /// 竞赛形式
    pub fn competition_formats(&self) -> Vec<&'static str> {
        vec![
            "小组赛: 2-4人一组",
            "淘汰赛: 晋级制",
            "比赛时长: 20-30分钟",
            "优先权规则: 近浪者优先",
            "裁判评分: 5位裁判",
            "实时评分显示",
        ]
    }

    /// 评分系统
    pub fn scoring_system(&self) -> Vec<&'static str> {
        vec![
            "单波评分: 0.1-10分",
            "取最佳两波评分",
            "评分因素: 承诺度、难度",
            "评分因素: 创新、组合",
            "评分因素: 速度、力量、流畅",
            "满分: 20分(两波各10分)",
        ]
    }

    /// 评分等级
    pub fn scoring_levels(&self) -> Vec<&'static str> {
        vec![
            "10分: 完美表现",
            "9.0-9.9分: 优秀",
            "7.0-8.9分: 良好",
            "5.0-6.9分: 一般",
            "3.0-4.9分: 较差",
            "0.1-2.9分: 失败",
        ]
    }

    /// 技术要求
    pub fn technical_requirements(&self) -> Vec<&'static str> {
        vec![
            "从绿浪区域划水入浪",
            "完成各种动作技巧",
            "动作种类: 管浪、空翻、旋转",
            "动作组合加分",
            "创新动作加分",
            "流畅衔接加分",
        ]
    }

    /// 优先权规则
    pub fn priority_rules(&self) -> Vec<&'static str> {
        vec![
            "第一优先权: 最近入浪点",
            "优先权按顺序轮换",
            "干扰他人减分",
            "抢浪取消优先权",
            "多人冲浪优先权判断",
            "裁判实时宣布优先权",
        ]
    }

    /// 犯规与处罚
    pub fn penalties(&self) -> Vec<&'static str> {
        vec![
            "干扰优先权者: 扣分",
            "危险动作: 取消资格",
            "违规装备: 取消资格",
            "评分干扰: 取消资格",
            "泳板失控: 扣分",
            "超时: 最后波不评分",
        ]
    }

    /// 装备要求
    pub fn equipment_requirements(&self) -> Vec<&'static str> {
        vec![
            "短板: 5-7英尺",
            "长板: 9英尺以上",
            " leash绳强制佩戴",
            "禁止使用脚绳助滑",
            "泳板材质限制",
            "尾鳍数量限制",
        ]
    }

    /// 场地条件
    pub fn venue_conditions(&self) -> Vec<&'static str> {
        vec![
            "浪高要求: 0.5-3米",
            "风向: 侧风或逆风",
            "水温: 不低于15°C",
            "比赛区域划定",
            "安全区域设置",
            "恶劣天气预案",
        ]
    }

    /// 安全要求
    pub fn safety_requirements(&self) -> Vec<&'static str> {
        vec![
            "救援艇待命",
            "水上巡逻队",
            "医疗救护设施",
            "通信联络设备",
            "天气监测系统",
            "选手安全教育",
        ]
    }
}

impl Default for SurfingCompetitionRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for SurfingCompetitionRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("surfing_competition")
    }

    fn explain(&self) -> String {
        format!(
            "【冲浪竞赛规则】\n\n\
            比赛项目:\n{}\n\n\
            竞赛形式:\n{}\n\n\
            评分系统:\n{}\n",
            self.events()
                .iter()
                .map(|e| format!("  • {}", e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.competition_formats()
                .iter()
                .map(|f| format!("  • {}", f))
                .collect::<Vec<_>>()
                .join("\n"),
            self.scoring_system()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn surfing_competition_rules_basic() {
        let rules = SurfingCompetitionRules::new();
        assert_eq!(rules.metadata().name, "冲浪竞赛规则");
        assert!(!rules.explain().is_empty());
        assert!(matches!(rules.category(), RuleCategory::Sports(_)));
    }

    #[test]
    fn surfing_competition_events() {
        let rules = SurfingCompetitionRules::new();
        let events = rules.events();
        assert!(events.iter().any(|e| e.contains("短板")));
        assert!(events.iter().any(|e| e.contains("长板")));
        assert!(events.len() >= 7);
    }

    #[test]
    fn surfing_competition_scoring() {
        let rules = SurfingCompetitionRules::new();
        let scoring = rules.scoring_system();
        assert!(scoring.iter().any(|s| s.contains("10分")));
        assert!(scoring.iter().any(|s| s.contains("两波")));
        assert!(scoring.len() >= 6);
    }
}