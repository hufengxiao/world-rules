//! 残疾人游泳规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 残疾人游泳规则
pub struct SwimmingParalympicRules {
    metadata: RuleMetadata,
}

impl SwimmingParalympicRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("残奥游泳规则", "残疾人游泳比赛规则")
                .with_origin("IPC")
                .with_tags(vec!["体育".into(), "游泳".into(), "残奥".into()]),
        }
    }

    /// 运动分级
    pub fn classification(&self) -> Vec<&'static str> {
        vec![
            "S级: 自由泳/仰泳/蝶泳",
            "SB级: 蛙泳",
            "SM级: 混合泳",
            "1-10级: 肢体残疾",
            "11-13级: 视力残疾",
            "14级: 智力残疾",
            "数字越小 = 残疾程度越重",
        ]
    }

    /// 比赛项目
    pub fn events(&self) -> Vec<&'static str> {
        vec![
            "男子/女子 自由泳 50m-400m",
            "男子/女子 仰泳 50m-100m",
            "男子/女子 蛙泳 50m-100m",
            "男子/女子 蝶泳 50m-100m",
            "男子/女子 混合泳 150m-200m",
            "接力项目: 4×100m",
            "各分级有不同项目",
        ]
    }

    /// 适应性规则
    pub fn adaptations(&self) -> Vec<&'static str> {
        vec![
            "允许辅助出发",
            "允许使用绳索出发",
            "允许教练提示 (视力残疾)",
            "转身辅助: 拍头提示",
            "允许使用假肢",
            "站立出发/水中出发可选",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "触碰分道线",
            "假肢脱落影响比赛",
            "泳衣违规",
            "出发违例",
            "未完成规定泳姿",
            "进入他人泳道",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "假肢: 需审批",
            "义眼: 必须固定",
            "泳镜: 允许",
            "泳帽: 必须佩戴",
            "禁止: 电子辅助设备",
        ]
    }

    /// 参赛资格
    pub fn eligibility(&self) -> Vec<&'static str> {
        vec![
            "必须通过分级评估",
            "最低残疾等级要求",
            "国际分级认证",
            "注册国家残奥委会",
            "达标成绩要求",
        ]
    }
}

impl Default for SwimmingParalympicRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for SwimmingParalympicRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("swimming_paralympic")
    }

    fn explain(&self) -> String {
        format!(
            "【残疾人游泳规则】\n\n\
            运动分级:\n{}\n\n\
            比赛项目:\n{}\n\n\
            适应性规则:\n{}\n\n\
            装备要求:\n{}",
            self.classification()
                .iter()
                .map(|c| format!("  • {}", c))
                .collect::<Vec<_>>()
                .join("\n"),
            self.events()
                .iter()
                .map(|e| format!("  • {}", e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.adaptations()
                .iter()
                .map(|a| format!("  • {}", a))
                .collect::<Vec<_>>()
                .join("\n"),
            self.equipment()
                .iter()
                .map(|e| format!("  • {}", e))
                .collect::<Vec<_>>()
                .join("\n"),
        )
    }
}
