//! 艺术学定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 艺术学定律集合
pub struct ArtScienceLaws {
    metadata: RuleMetadata,
}

impl ArtScienceLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "艺术学定律",
                "艺术学基本定律"
            )
            .with_origin("人文科学")
            .with_tags(vec!["科学".into(), "艺术".into()]),
        }
    }

    /// 艺术创作定律
    pub fn creation_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("创作动机定律", "创作冲动", "艺术创作动机来源"),
            ("创作构思定律", "构思过程", "创作构思规律"),
            ("创作表达定律", "表达技巧", "艺术表达技巧"),
            ("创作风格定律", "风格形成", "个人风格形成"),
            ("创作情感定律", "情感表达", "情感在创作中的作用"),
            ("创作技法定律", "技法运用", "技法运用规律"),
            ("创作材料定律", "材料选择", "材料选择与运用"),
        ]
    }

    /// 艺术形式定律
    pub fn form_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("构图定律", "画面布局", "构图基本规律"),
            ("色彩定律", "色彩运用", "色彩搭配规律"),
            ("线条定律", "线条表现", "线条表现力"),
            ("空间定律", "空间表现", "空间深度表现"),
            ("节奏定律", "韵律控制", "作品节奏控制"),
            ("比例定律", "比例关系", "比例协调关系"),
            ("平衡定律", "视觉平衡", "视觉平衡原理"),
        ]
    }

    /// 艺术审美定律
    pub fn aesthetics_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("审美定律", "审美体验", "审美体验规律"),
            ("美感定律", "美感产生", "美感产生机制"),
            ("审美判断定律", "审美评价", "审美判断标准"),
            ("审美趣味定律", "趣味形成", "审美趣味培养"),
            ("审美距离定律", "距离效应", "审美距离效应"),
            ("审美共鸣定律", "情感共鸣", "审美共鸣产生"),
            ("审美期待定律", "期待视野", "审美期待视野"),
        ]
    }

    /// 艺术发展定律
    pub fn development_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("艺术演变定律", "艺术变化", "艺术风格演变"),
            ("艺术传承定律", "传统继承", "艺术传统继承"),
            ("艺术创新定律", "创新发展", "艺术创新规律"),
            ("艺术流派定律", "流派形成", "艺术流派发展"),
            ("艺术影响定律", "相互影响", "艺术相互影响"),
            ("艺术传播定律", "传播扩散", "艺术传播规律"),
            ("艺术市场定律", "市场规律", "艺术市场规律"),
        ]
    }

    /// 艺术类型
    pub fn art_forms(&self) -> Vec<&'static str> {
        vec![
            "绘画",
            "雕塑",
            "音乐",
            "舞蹈",
            "戏剧",
            "电影",
            "摄影",
            "设计",
        ]
    }

    /// 艺术理论
    pub fn theories(&self) -> Vec<&'static str> {
        vec![
            "模仿说",
            "表现说",
            "形式说",
            "象征说",
            "符号说",
            "情感说",
            "游戏说",
            "巫术说",
        ]
    }

    /// 艺术心理学定律
    pub fn psychology_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("艺术感知定律", "感知过程", "艺术感知心理机制"),
            ("艺术记忆定律", "记忆效应", "艺术记忆规律"),
            ("艺术想象定律", "想象机制", "艺术想象力规律"),
            ("艺术联想定律", "联想触发", "艺术联想规律"),
            ("艺术直觉定律", "直觉判断", "艺术直觉认知"),
            ("艺术通感定律", "感官联通", "跨感官艺术体验"),
            ("艺术移情定律", "情感投射", "审美移情作用"),
            ("艺术完形定律", "格式塔", "视觉完形倾向"),
        ]
    }

    /// 艺术文化定律
    pub fn culture_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("艺术符号定律", "符号系统", "艺术符号编码规律"),
            ("艺术仪式定律", "仪式功能", "艺术与仪式关系"),
            ("艺术身份定律", "身份建构", "艺术与身份认同"),
            ("艺术权力定律", "权力表达", "艺术与权力关系"),
            ("艺术全球化定律", "文化交融", "艺术全球化趋势"),
            ("艺术民族定律", "民族特色", "艺术民族性表达"),
            ("艺术世俗定律", "世俗化", "艺术世俗化进程"),
            ("艺术遗产定律", "遗产保护", "艺术遗产传承"),
        ]
    }

    /// 艺术批评定律
    pub fn criticism_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("形式批评定律", "形式分析", "形式主义批评方法"),
            ("社会批评定律", "社会视角", "社会学批评方法"),
            ("心理批评定律", "心理分析", "精神分析批评"),
            ("女性批评定律", "性别视角", "女性主义批评"),
            ("后殖民批评定律", "殖民反思", "后殖民主义批评"),
            ("解构批评定律", "文本解构", "解构主义批评"),
            ("接受批评定律", "读者反应", "接受美学批评"),
            ("生态批评定律", "生态视角", "生态美学批评"),
        ]
    }
}

impl Default for ArtScienceLaws {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ArtScienceLaws {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("art_science")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【艺术学定律】\n\n创作定律:\n{}\n\n形式定律:\n{}\n\n审美定律:\n{}\n\n心理学定律:\n{}\n\n文化定律:\n{}\n\n批评定律:\n{}\n",
            self.creation_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.form_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.aesthetics_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.psychology_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.culture_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.criticism_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_art_science_laws() {
        let laws = ArtScienceLaws::new();
        assert!(!laws.creation_laws().is_empty());
        assert!(!laws.form_laws().is_empty());
    }
}