//! 蛙泳规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 蛙泳规则
pub struct SwimmingBreaststrokeRules {
    metadata: RuleMetadata,
}

impl SwimmingBreaststrokeRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("蛙泳规则", "蛙泳技术规则详解")
                .with_origin("FINA")
                .with_tags(vec!["体育".into(), "游泳".into(), "蛙泳".into()]),
        }
    }

    /// 基本技术
    pub fn technique(&self) -> Vec<&'static str> {
        vec![
            "双手同时向前伸展",
            "双手同时向后划水",
            "双腿同时蹬腿",
            "禁止海豚腿 (出发/转身后第一次除外)",
            "每次完整动作后头须露出水面",
            "肩部须保持水平",
        ]
    }

    /// 划水规则
    pub fn arm_stroke(&self) -> Vec<&'static str> {
        vec![
            "双手同时向前伸展",
            "双手向后划水至胸线",
            "肘部不可超过臀部线",
            "双手同时出水前伸",
            "禁止不对称划水",
            "禁止单手划水",
        ]
    }

    /// 蹬腿规则
    pub fn kick(&self) -> Vec<&'static str> {
        vec![
            "双腿同时蹬夹",
            "蛙泳腿: 收腿→翻脚→蹬夹",
            "禁止剪刀腿",
            "禁止海豚腿 (除出发/转身后第一次)",
            "双腿同时对称动作",
            "禁止交替打腿",
        ]
    }

    /// 出发和转身
    pub fn start_turn(&self) -> Vec<&'static str> {
        vec![
            "跳台出发",
            "出发后允许一次海豚腿",
            "转身必须双手同时触壁",
            "转身后允许一次海豚腿",
            "出发/转身后可潜泳",
            "潜泳距离: 无限制 (但头须在第二划水前露出)",
        ]
    }

    /// 终点规则
    pub fn finish(&self) -> Vec<&'static str> {
        vec![
            "必须双手同时触壁",
            "触壁时肩部水平",
            "触壁时双手在水面或水下",
            "禁止单手触壁",
        ]
    }

    /// 犯规行为
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "双手不同时触壁",
            "双手不同时划水",
            "双腿不同时蹬腿",
            "使用海豚腿 (除出发/转身后第一次)",
            "使用剪刀腿",
            "肩部翻转超过水平",
            "肘部超过臀部线",
            "潜泳时头未按时露出",
        ]
    }

    /// 比赛距离
    pub fn distances(&self) -> Vec<&'static str> {
        vec![
            "50米蛙泳 (短池)",
            "100米蛙泳",
            "200米蛙泳",
            "混合泳中的蛙泳段",
        ]
    }

    /// 技术要点
    pub fn key_points(&self) -> Vec<&'static str> {
        vec![
            "节奏: 慢划水快蹬腿",
            "保持流线型",
            "呼吸: 划水时抬头",
            "收腿: 膝盖不超肩宽",
            "蹬夹: 宽蹬窄夹",
            "滑行: 每次动作后滑行",
        ]
    }
}

impl Default for SwimmingBreaststrokeRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for SwimmingBreaststrokeRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("swimming_breaststroke")
    }

    fn explain(&self) -> String {
        format!(
            "【蛙泳规则】\n\n\
            基本技术:\n{}\n\n\
            划水规则:\n{}\n\n\
            蹬腿规则:\n{}\n\n\
            犯规行为:\n{}",
            self.technique()
                .iter()
                .map(|t| format!("  • {}", t))
                .collect::<Vec<_>>()
                .join("\n"),
            self.arm_stroke()
                .iter()
                .map(|a| format!("  • {}", a))
                .collect::<Vec<_>>()
                .join("\n"),
            self.kick()
                .iter()
                .map(|k| format!("  • {}", k))
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
