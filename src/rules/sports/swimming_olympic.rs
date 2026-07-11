//! 奥运游泳规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 奥运游泳规则
pub struct SwimmingOlympicRules {
    metadata: RuleMetadata,
}

impl SwimmingOlympicRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("奥运游泳规则", "奥运会游泳比赛规则")
                .with_origin("IOC/FINA")
                .with_tags(vec!["体育".into(), "游泳".into(), "奥运".into()]),
        }
    }

    /// 比赛项目
    pub fn events(&self) -> Vec<&'static str> {
        vec![
            "男子/女子 自由泳 50m/100m/200m/400m/800m/1500m",
            "男子/女子 仰泳 100m/200m",
            "男子/女子 蛙泳 100m/200m",
            "男子/女子 蝶泳 100m/200m",
            "男子/女子 个人混合泳 200m/400m",
            "男子/女子 自由泳接力 4×100m/4×200m",
            "男子/女子 混合泳接力 4×100m",
            "男女混合 自由泳接力 4×100m",
            "男女混合 混合泳接力 4×100m",
            "马拉松游泳 10公里",
        ]
    }

    /// 泳池规格
    pub fn pool_specifications(&self) -> Vec<&'static str> {
        vec![
            "长度: 50米 (长池)",
            "宽度: 25米 (10条泳道)",
            "深度: 至少2米",
            "泳道宽度: 2.5米",
            "水温: 25-28°C",
            "电子计时系统: 精确到0.01秒",
            "自动出发裁判系统",
            "水下视频裁判系统",
        ]
    }

    /// 资格赛制
    pub fn qualification_system(&self) -> Vec<&'static str> {
        vec![
            "奥运A标: 直接获得资格",
            "奥运B标: 需通过选拔赛",
            "每国每项目最多2名运动员",
            "接力项目: 16支队伍",
            "东道主国家保送名额",
            "外卡制度: 发展中国家支持",
        ]
    }

    /// 比赛轮次
    pub fn competition_rounds(&self) -> Vec<&'static str> {
        vec![
            "预赛: 所有运动员参赛",
            "半决赛: 前16名晋级 (50m/100m/200m)",
            "决赛: 前8名晋级",
            "慢组决赛: 第9-16名 (B组决赛)",
            "接力项目: 预赛→决赛",
            "马拉松: 直接决赛",
        ]
    }

    /// 出发规则
    pub fn starting_rules(&self) -> Vec<&'static str> {
        vec![
            "自由泳/蛙泳/蝶泳: 跳台出发",
            "仰泳: 水中出发",
            "接力: 触壁后下一棒出发",
            "抢跳警告: 第一次警告",
            "第二次抢跳: 取消资格",
            "反应时间监测: 最小0.10秒",
        ]
    }

    /// 世界纪录奖励
    pub fn record_bonuses(&self) -> Vec<&'static str> {
        vec![
            "世界纪录: 奖金 $50,000",
            "奥运会纪录: 奖金 $25,000",
            "破纪录奖金可叠加",
            "接力破纪录奖金平分",
        ]
    }
}

impl Default for SwimmingOlympicRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for SwimmingOlympicRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("swimming_olympic")
    }

    fn explain(&self) -> String {
        format!(
            "【奥运游泳规则】\n\n\
            比赛项目:\n{}\n\n\
            泳池规格:\n{}\n\n\
            资格赛制:\n{}\n\n\
            比赛轮次:\n{}\n\n\
            出发规则:\n{}",
            self.events()
                .iter()
                .map(|e| format!("  • {}", e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.pool_specifications()
                .iter()
                .map(|p| format!("  • {}", p))
                .collect::<Vec<_>>()
                .join("\n"),
            self.qualification_system()
                .iter()
                .map(|q| format!("  • {}", q))
                .collect::<Vec<_>>()
                .join("\n"),
            self.competition_rounds()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.starting_rules()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
        )
    }
}
