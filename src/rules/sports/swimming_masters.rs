//! 大师游泳规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 大师游泳规则
pub struct SwimmingMastersRules {
    metadata: RuleMetadata,
}

impl SwimmingMastersRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("大师游泳规则", "成人游泳比赛规则 (25岁以上)")
                .with_origin("FINA")
                .with_tags(vec!["体育".into(), "游泳".into(), "大师".into()]),
        }
    }

    /// 年龄组别
    pub fn age_groups(&self) -> Vec<&'static str> {
        vec![
            "25-29岁组",
            "30-34岁组",
            "35-39岁组",
            "40-44岁组",
            "45-49岁组",
            "50-54岁组",
            "55-59岁组",
            "60-64岁组",
            "65-69岁组",
            "70-74岁组",
            "75-79岁组",
            "80+岁组",
        ]
    }

    /// 比赛项目
    pub fn events(&self) -> Vec<&'static str> {
        vec![
            "自由泳: 50m/100m/200m/400m/800m/1500m",
            "仰泳: 50m/100m/200m",
            "蛙泳: 50m/100m/200m",
            "蝶泳: 50m/100m/200m",
            "个人混合泳: 100m/200m/400m",
            "接力: 4×50m/4×100m",
            "混合接力: 男女混合",
        ]
    }

    /// 参赛资格
    pub fn eligibility(&self) -> Vec<&'static str> {
        vec![
            "最低年龄: 25岁",
            "注册俱乐部或协会",
            "年费: 各协会自定",
            "不需要达标成绩",
            "鼓励参与为主",
            "年龄以年底计算",
        ]
    }

    /// 世界纪录
    pub fn records(&self) -> Vec<&'static str> {
        vec![
            "大师世界纪录: FINA认证",
            "大师洲际纪录",
            "大师国家纪录",
            "分年龄组纪录",
            "每年更新纪录表",
        ]
    }

    /// 比赛规则调整
    pub fn rule_adjustments(&self) -> Vec<&'static str> {
        vec![
            "允许使用浮漂 (部分赛事)",
            "出发: 可从水中出发",
            "转身: 宽松执法",
            "接力: 年龄组累计计算",
            "预决赛制: 通常直接决赛",
            "鼓励健康参与",
        ]
    }

    /// 健康要求
    pub fn health_requirements(&self) -> Vec<&'static str> {
        vec![
            "年度体检建议",
            "心脏检查推荐",
            "比赛前热身必需",
            "急救医疗现场",
            "保险要求",
        ]
    }

    /// 赛事类型
    pub fn competition_types(&self) -> Vec<&'static str> {
        vec![
            "大师世锦赛: 每两年",
            "洲际大师锦标赛",
            "国家大师锦标赛",
            "地区大师联赛",
            "俱乐部大师赛",
        ]
    }
}

impl Default for SwimmingMastersRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for SwimmingMastersRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("swimming_masters")
    }

    fn explain(&self) -> String {
        format!(
            "【大师游泳规则】\n\n\
            年龄组别:\n{}\n\n\
            比赛项目:\n{}\n\n\
            参赛资格:\n{}\n\n\
            规则调整:\n{}",
            self.age_groups()
                .iter()
                .map(|a| format!("  • {}", a))
                .collect::<Vec<_>>()
                .join("\n"),
            self.events()
                .iter()
                .map(|e| format!("  • {}", e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.eligibility()
                .iter()
                .map(|e| format!("  • {}", e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.rule_adjustments()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
        )
    }
}
