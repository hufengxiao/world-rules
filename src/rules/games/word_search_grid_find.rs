//! 字母网格找词
//!
//! 在字母方阵中沿方向找出隐藏单词的益智游戏

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: WordSearchGridFindRules,
    name: "字母网格找词",
    desc: "在字母方阵中沿方向找出隐藏单词的益智游戏",
    origin: "美国",
    tags: ["游戏", "找词", "益智", "字母"]
}

impl WordSearchGridFindRules {
    /// 玩法规则
    pub fn basic(&self) -> Vec<&'static str> {
        vec!["字母填入网格", "沿行或列或斜", "按序找单词", "找到划出词形"]
    }

    /// 查找方法
    pub fn search(&self) -> Vec<&'static str> {
        vec![
            "先看词首字母",
            "在网格寻首字符",
            "再沿八个方向",
            "核对后续字母",
        ]
    }

    /// 技巧提示
    pub fn technique(&self) -> Vec<&'static str> {
        vec!["从罕见字母入手", "词长优先锁定", "不重复跳过", "标记已找词"]
    }

    /// 收尾完成
    pub fn finish(&self) -> Vec<&'static str> {
        vec!["全部词找到", "核对清单不漏", "回查误找", "完成后读完全部"]
    }
}

impl Rule for WordSearchGridFindRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("word_search")
    }

    fn explain(&self) -> String {
        format!(
            "【字母网格找词】\n{}",
            [
                format!(
                    "玩法规则：\\n{}",
                    self.basic()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "查找方法：\\n{}",
                    self.search()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "技巧提示：\\n{}",
                    self.technique()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "收尾完成：\\n{}",
                    self.finish()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
            ]
            .join("\n\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::core::ValidateContext;

    #[test]
    fn test_wordsearchgridfindrules_basic() {
        let rules = WordSearchGridFindRules::new();
        assert_eq!(rules.metadata().name, "字母网格找词");
        assert!(!rules.basic().is_empty());
        assert!(!rules.search().is_empty());
        assert!(!rules.technique().is_empty());
        assert!(!rules.finish().is_empty());
    }

    #[test]
    fn test_wordsearchgridfindrules_validation() {
        let rules = WordSearchGridFindRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("word_search"));
    }

    #[test]
    fn test_wordsearchgridfindrules_explain() {
        let rules = WordSearchGridFindRules::new();
        let e = rules.explain();
        assert!(e.contains("玩法规则"));
        assert!(e.contains("查找方法"));
        assert!(e.contains("技巧提示"));
    }
}
