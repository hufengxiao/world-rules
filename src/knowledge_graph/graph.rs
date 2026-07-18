//! 规则知识图谱构建模块
//!
//! 构建和管理知识图谱数据结构。

use crate::knowledge_graph::entity::{Entity, EntityType};
use crate::knowledge_graph::relation::{Relation, RelationType};
use std::collections::{HashMap, HashSet};

/// 图谱节点
///
/// 知识图谱中的节点，封装实体信息。
///
/// # 示例
/// ```
/// use world_rules::knowledge_graph::{GraphNode, Entity, EntityType};
///
/// let entity = Entity::new("球员", EntityType::Subject);
/// let node = GraphNode::from_entity(entity);
///
/// assert_eq!(node.entity_type, EntityType::Subject);
/// ```
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GraphNode {
    /// 节点 ID（与实体 ID 相同）
    pub id: String,
    /// 节点名称
    pub name: String,
    /// 实体类型
    pub entity_type: EntityType,
    /// 置信度
    pub confidence: f64,
    /// 入边数量
    pub in_degree: usize,
    /// 出边数量
    pub out_degree: usize,
    /// 节点属性
    pub attributes: HashMap<String, String>,
}

impl GraphNode {
    /// 从实体创建节点
    pub fn from_entity(entity: Entity) -> Self {
        Self {
            id: entity.id.clone(),
            name: entity.name,
            entity_type: entity.entity_type,
            confidence: entity.confidence,
            in_degree: 0,
            out_degree: 0,
            attributes: entity.attributes,
        }
    }

    /// 获取节点度数
    pub fn degree(&self) -> usize {
        self.in_degree + self.out_degree
    }

    /// 检查是否为叶子节点（出度为 0）
    pub fn is_leaf(&self) -> bool {
        self.out_degree == 0
    }

    /// 检查是否为根节点（入度为 0）
    pub fn is_root(&self) -> bool {
        self.in_degree == 0
    }
}

impl std::fmt::Display for GraphNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}] {} (度: {})",
            self.entity_type,
            self.name,
            self.degree()
        )
    }
}

/// 图谱边
///
/// 知识图谱中的边，封装关系信息。
///
/// # 示例
/// ```
/// use world_rules::knowledge_graph::{GraphEdge, Relation, RelationType};
///
/// let relation = Relation::new("node1", "node2", RelationType::Causes);
/// let edge = GraphEdge::from_relation(relation);
///
/// assert_eq!(edge.relation_type, RelationType::Causes);
/// ```
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GraphEdge {
    /// 边 ID（与关系 ID 相同）
    pub id: String,
    /// 源节点 ID
    pub source: String,
    /// 目标节点 ID
    pub target: String,
    /// 关系类型
    pub relation_type: RelationType,
    /// 置信度
    pub confidence: f64,
    /// 边属性
    pub attributes: HashMap<String, String>,
}

impl GraphEdge {
    /// 从关系创建边
    pub fn from_relation(relation: Relation) -> Self {
        Self {
            id: relation.id.clone(),
            source: relation.source_entity,
            target: relation.target_entity,
            relation_type: relation.relation_type,
            confidence: relation.confidence,
            attributes: relation.attributes,
        }
    }
}

impl std::fmt::Display for GraphEdge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} -> [{}] -> {}",
            self.source, self.relation_type, self.target
        )
    }
}

/// 图谱统计信息
///
/// # 示例
/// ```
/// use world_rules::knowledge_graph::GraphStats;
///
/// let stats = GraphStats {
///     node_count: 10,
///     edge_count: 15,
///     entity_type_distribution: std::collections::HashMap::new(),
///     relation_type_distribution: std::collections::HashMap::new(),
///     avg_degree: 1.5,
///     max_degree: 5,
/// };
///
/// assert_eq!(stats.node_count, 10);
/// ```
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GraphStats {
    /// 节点数量
    pub node_count: usize,
    /// 边数量
    pub edge_count: usize,
    /// 实体类型分布
    pub entity_type_distribution: HashMap<String, usize>,
    /// 关系类型分布
    pub relation_type_distribution: HashMap<String, usize>,
    /// 平均度数
    pub avg_degree: f64,
    /// 最大度数
    pub max_degree: usize,
}

impl std::fmt::Display for GraphStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "知识图谱统计: {} 个节点, {} 条边, 平均度数 {:.2}",
            self.node_count, self.edge_count, self.avg_degree
        )
    }
}

/// 知识图谱
///
/// 管理规则实体和关系构成的图谱结构。
///
/// # 示例
/// ```
/// use world_rules::knowledge_graph::KnowledgeGraph;
///
/// let mut graph = KnowledgeGraph::new("体育规则");
/// assert_eq!(graph.name, "体育规则");
/// ```
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KnowledgeGraph {
    /// 图谱名称
    pub name: String,
    /// 节点集合
    nodes: HashMap<String, GraphNode>,
    /// 边集合（使用源节点 ID 索引）
    edges: HashMap<String, Vec<GraphEdge>>,
    /// 所有边的扁平列表（用于遍历）
    all_edges: Vec<GraphEdge>,
}

impl KnowledgeGraph {
    /// 创建新的知识图谱
    ///
    /// # Examples
    /// ```
    /// use world_rules::knowledge_graph::KnowledgeGraph;
    ///
    /// let graph = KnowledgeGraph::new("测试图谱");
    /// assert_eq!(graph.name, "测试图谱");
    /// ```
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            nodes: HashMap::new(),
            edges: HashMap::new(),
            all_edges: Vec::new(),
        }
    }

    /// 添加实体（转换为节点）
    ///
    /// # Examples
    /// ```
    /// use world_rules::knowledge_graph::{KnowledgeGraph, Entity, EntityType};
    ///
    /// let mut graph = KnowledgeGraph::new("测试");
    /// let entity = Entity::new("球员", EntityType::Subject);
    /// graph.add_entity(entity);
    ///
    /// assert_eq!(graph.node_count(), 1);
    /// ```
    pub fn add_entity(&mut self, entity: Entity) {
        let node = GraphNode::from_entity(entity);
        self.nodes.insert(node.id.clone(), node);
    }

    /// 批量添加实体
    ///
    /// # Examples
    /// ```
    /// use world_rules::knowledge_graph::{KnowledgeGraph, Entity, EntityType};
    ///
    /// let mut graph = KnowledgeGraph::new("测试");
    /// let entities = vec![
    ///     Entity::new("球员", EntityType::Subject),
    ///     Entity::new("犯规", EntityType::Action),
    /// ];
    /// graph.add_entities(entities);
    ///
    /// assert_eq!(graph.node_count(), 2);
    /// ```
    pub fn add_entities(&mut self, entities: Vec<Entity>) {
        for entity in entities {
            self.add_entity(entity);
        }
    }

    /// 添加关系（转换为边）
    ///
    /// # Examples
    /// ```
    /// use world_rules::knowledge_graph::{KnowledgeGraph, Relation, RelationType, Entity, EntityType};
    ///
    /// let mut graph = KnowledgeGraph::new("测试");
    /// graph.add_entity(Entity::new("球员", EntityType::Subject));
    /// graph.add_entity(Entity::new("犯规", EntityType::Action));
    ///
    /// let relation = Relation::new("主体_球员", "动作_犯规", RelationType::Participates);
    /// graph.add_relation(relation);
    ///
    /// assert_eq!(graph.edge_count(), 1);
    /// ```
    pub fn add_relation(&mut self, relation: Relation) {
        let edge = GraphEdge::from_relation(relation);

        // 更新节点的度数
        if let Some(source_node) = self.nodes.get_mut(&edge.source) {
            source_node.out_degree += 1;
        }
        if let Some(target_node) = self.nodes.get_mut(&edge.target) {
            target_node.in_degree += 1;
        }

        // 添加边
        self.edges
            .entry(edge.source.clone())
            .or_default()
            .push(edge.clone());
        self.all_edges.push(edge);
    }

    /// 批量添加关系
    pub fn add_relations(&mut self, relations: Vec<Relation>) {
        for relation in relations {
            self.add_relation(relation);
        }
    }

    /// 获取节点数量
    ///
    /// # Examples
    /// ```
    /// use world_rules::knowledge_graph::KnowledgeGraph;
    ///
    /// let graph = KnowledgeGraph::new("测试");
    /// assert_eq!(graph.node_count(), 0);
    /// ```
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// 获取边数量
    ///
    /// # Examples
    /// ```
    /// use world_rules::knowledge_graph::KnowledgeGraph;
    ///
    /// let graph = KnowledgeGraph::new("测试");
    /// assert_eq!(graph.edge_count(), 0);
    /// ```
    pub fn edge_count(&self) -> usize {
        self.all_edges.len()
    }

    /// 获取节点
    ///
    /// # Examples
    /// ```
    /// use world_rules::knowledge_graph::{KnowledgeGraph, Entity, EntityType};
    ///
    /// let mut graph = KnowledgeGraph::new("测试");
    /// graph.add_entity(Entity::new("球员", EntityType::Subject));
    ///
    /// let node = graph.get_node("主体_球员");
    /// assert!(node.is_some());
    /// ```
    pub fn get_node(&self, id: &str) -> Option<&GraphNode> {
        self.nodes.get(id)
    }

    /// 获取节点的所有出边
    ///
    /// # Examples
    /// ```
    /// use world_rules::knowledge_graph::{KnowledgeGraph, Entity, EntityType, Relation, RelationType};
    ///
    /// let mut graph = KnowledgeGraph::new("测试");
    /// graph.add_entity(Entity::new("A", EntityType::Subject));
    /// graph.add_entity(Entity::new("B", EntityType::Object));
    /// graph.add_relation(Relation::new("主体_A", "客体_B", RelationType::Causes));
    ///
    /// let out_edges = graph.get_out_edges("主体_A");
    /// assert_eq!(out_edges.len(), 1);
    /// ```
    pub fn get_out_edges(&self, node_id: &str) -> Vec<&GraphEdge> {
        self.edges
            .get(node_id)
            .map(|edges| edges.iter().collect())
            .unwrap_or_default()
    }

    /// 获取所有节点
    pub fn get_all_nodes(&self) -> Vec<&GraphNode> {
        self.nodes.values().collect()
    }

    /// 获取所有边
    pub fn get_all_edges(&self) -> Vec<&GraphEdge> {
        self.all_edges.iter().collect()
    }

    /// 按实体类型过滤节点
    ///
    /// # Examples
    /// ```
    /// use world_rules::knowledge_graph::{KnowledgeGraph, Entity, EntityType};
    ///
    /// let mut graph = KnowledgeGraph::new("测试");
    /// graph.add_entity(Entity::new("球员", EntityType::Subject));
    /// graph.add_entity(Entity::new("犯规", EntityType::Action));
    ///
    /// let subjects = graph.filter_nodes_by_type(EntityType::Subject);
    /// assert_eq!(subjects.len(), 1);
    /// ```
    pub fn filter_nodes_by_type(&self, entity_type: EntityType) -> Vec<&GraphNode> {
        self.nodes
            .values()
            .filter(|n| n.entity_type == entity_type)
            .collect()
    }

    /// 按关系类型过滤边
    pub fn filter_edges_by_type(&self, relation_type: RelationType) -> Vec<&GraphEdge> {
        self.all_edges
            .iter()
            .filter(|e| e.relation_type == relation_type)
            .collect()
    }

    /// 查找路径（BFS）
    ///
    /// # Examples
    /// ```
    /// use world_rules::knowledge_graph::{KnowledgeGraph, Entity, EntityType, Relation, RelationType};
    ///
    /// let mut graph = KnowledgeGraph::new("测试");
    /// graph.add_entity(Entity::new("A", EntityType::Subject));
    /// graph.add_entity(Entity::new("B", EntityType::Object));
    /// graph.add_entity(Entity::new("C", EntityType::Result));
    /// graph.add_relation(Relation::new("主体_A", "客体_B", RelationType::Causes));
    /// graph.add_relation(Relation::new("客体_B", "结果_C", RelationType::Causes));
    ///
    /// let path = graph.find_path("主体_A", "结果_C");
    /// assert!(path.is_some());
    /// ```
    pub fn find_path(&self, source: &str, target: &str) -> Option<Vec<String>> {
        use std::collections::VecDeque;

        if !self.nodes.contains_key(source) || !self.nodes.contains_key(target) {
            return None;
        }

        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut parent: HashMap<String, String> = HashMap::new();

        visited.insert(source.to_string());
        queue.push_back(source.to_string());

        while let Some(current) = queue.pop_front() {
            if current == target {
                // 重建路径
                let mut path = vec![target.to_string()];
                let mut node = target.to_string();
                while let Some(p) = parent.get(&node) {
                    path.push(p.clone());
                    node = p.clone();
                }
                path.reverse();
                return Some(path);
            }

            for edge in self.get_out_edges(&current) {
                if !visited.contains(&edge.target) {
                    visited.insert(edge.target.clone());
                    parent.insert(edge.target.clone(), current.clone());
                    queue.push_back(edge.target.clone());
                }
            }
        }

        None
    }

    /// 获取子图
    ///
    /// # Examples
    /// ```
    /// use world_rules::knowledge_graph::{KnowledgeGraph, Entity, EntityType};
    ///
    /// let mut graph = KnowledgeGraph::new("测试");
    /// graph.add_entity(Entity::new("A", EntityType::Subject));
    /// graph.add_entity(Entity::new("B", EntityType::Object));
    ///
    /// let subgraph = graph.get_subgraph(&["主体_A".to_string()]);
    /// assert_eq!(subgraph.node_count(), 1);
    /// ```
    pub fn get_subgraph(&self, node_ids: &[String]) -> KnowledgeGraph {
        let mut subgraph = KnowledgeGraph::new(format!("{} - 子图", self.name));

        for id in node_ids {
            if let Some(node) = self.nodes.get(id) {
                subgraph.nodes.insert(id.clone(), node.clone());
            }
        }

        for edge in &self.all_edges {
            if subgraph.nodes.contains_key(&edge.source)
                && subgraph.nodes.contains_key(&edge.target)
            {
                subgraph
                    .edges
                    .entry(edge.source.clone())
                    .or_default()
                    .push(edge.clone());
                subgraph.all_edges.push(edge.clone());
            }
        }

        subgraph
    }

    /// 获取图谱统计信息
    ///
    /// # Examples
    /// ```
    /// use world_rules::knowledge_graph::{KnowledgeGraph, Entity, EntityType};
    ///
    /// let mut graph = KnowledgeGraph::new("测试");
    /// graph.add_entity(Entity::new("球员", EntityType::Subject));
    /// let stats = graph.get_stats();
    ///
    /// assert_eq!(stats.node_count, 1);
    /// ```
    pub fn get_stats(&self) -> GraphStats {
        let mut entity_type_distribution = HashMap::new();
        let mut relation_type_distribution = HashMap::new();

        for node in self.nodes.values() {
            let type_name = node.entity_type.display_name().to_string();
            *entity_type_distribution.entry(type_name).or_insert(0) += 1;
        }

        for edge in &self.all_edges {
            let type_name = edge.relation_type.display_name().to_string();
            *relation_type_distribution.entry(type_name).or_insert(0) += 1;
        }

        let total_degree: usize = self.nodes.values().map(|n| n.degree()).sum();
        let avg_degree = if !self.nodes.is_empty() {
            total_degree as f64 / self.nodes.len() as f64
        } else {
            0.0
        };

        let max_degree = self.nodes.values().map(|n| n.degree()).max().unwrap_or(0);

        GraphStats {
            node_count: self.nodes.len(),
            edge_count: self.all_edges.len(),
            entity_type_distribution,
            relation_type_distribution,
            avg_degree,
            max_degree,
        }
    }

    /// 导出为 JSON
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// 从 JSON 导入
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// 导出为 DOT 格式（用于 Graphviz）
    pub fn to_dot(&self) -> String {
        let mut dot = String::new();
        dot.push_str(&format!("digraph \"{}\" {{\n", self.name));
        dot.push_str("  rankdir=LR;\n");
        dot.push_str("  node [shape=box];\n\n");

        // 添加节点
        for node in self.nodes.values() {
            let label = format!("{}\\n[{}]", node.name, node.entity_type);
            dot.push_str(&format!("  \"{}\" [label=\"{}\"];\n", node.id, label));
        }

        dot.push('\n');

        // 添加边
        for edge in &self.all_edges {
            dot.push_str(&format!(
                "  \"{}\" -> \"{}\" [label=\"{}\"];\n",
                edge.source, edge.target, edge.relation_type
            ));
        }

        dot.push_str("}\n");
        dot
    }
}

impl std::fmt::Display for KnowledgeGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let stats = self.get_stats();
        write!(f, "知识图谱【{}】: {}", self.name, stats)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_node_creation() {
        let entity = Entity::new("球员", EntityType::Subject).with_confidence(0.9);
        let node = GraphNode::from_entity(entity);

        assert_eq!(node.name, "球员");
        assert_eq!(node.entity_type, EntityType::Subject);
        assert_eq!(node.confidence, 0.9);
        assert_eq!(node.in_degree, 0);
        assert_eq!(node.out_degree, 0);
    }

    #[test]
    fn test_graph_node_degree() {
        let mut node = GraphNode::from_entity(Entity::new("A", EntityType::Subject));
        node.in_degree = 2;
        node.out_degree = 3;

        assert_eq!(node.degree(), 5);
        assert!(!node.is_leaf());
        assert!(!node.is_root());
    }

    #[test]
    fn test_knowledge_graph_basic() {
        let mut graph = KnowledgeGraph::new("测试图谱");

        assert_eq!(graph.node_count(), 0);
        assert_eq!(graph.edge_count(), 0);

        graph.add_entity(Entity::new("球员", EntityType::Subject));
        assert_eq!(graph.node_count(), 1);

        graph.add_entities(vec![
            Entity::new("犯规", EntityType::Action),
            Entity::new("黄牌", EntityType::Result),
        ]);
        assert_eq!(graph.node_count(), 3);
    }

    #[test]
    fn test_knowledge_graph_relations() {
        let mut graph = KnowledgeGraph::new("测试图谱");

        graph.add_entity(Entity::new("球员", EntityType::Subject));
        graph.add_entity(Entity::new("犯规", EntityType::Action));

        graph.add_relation(Relation::new(
            "主体_球员",
            "动作_犯规",
            RelationType::Participates,
        ));

        assert_eq!(graph.edge_count(), 1);

        // 检查节点度数更新
        let source_node = graph.get_node("主体_球员").unwrap();
        assert_eq!(source_node.out_degree, 1);

        let target_node = graph.get_node("动作_犯规").unwrap();
        assert_eq!(target_node.in_degree, 1);
    }

    #[test]
    fn test_filter_nodes() {
        let mut graph = KnowledgeGraph::new("测试图谱");

        graph.add_entities(vec![
            Entity::new("球员", EntityType::Subject),
            Entity::new("教练", EntityType::Subject),
            Entity::new("犯规", EntityType::Action),
        ]);

        let subjects = graph.filter_nodes_by_type(EntityType::Subject);
        assert_eq!(subjects.len(), 2);

        let actions = graph.filter_nodes_by_type(EntityType::Action);
        assert_eq!(actions.len(), 1);
    }

    #[test]
    fn test_find_path() {
        let mut graph = KnowledgeGraph::new("测试图谱");

        graph.add_entities(vec![
            Entity::new("A", EntityType::Subject),
            Entity::new("B", EntityType::Object),
            Entity::new("C", EntityType::Result),
        ]);

        graph.add_relation(Relation::new("主体_A", "客体_B", RelationType::Causes));
        graph.add_relation(Relation::new("客体_B", "结果_C", RelationType::Causes));

        let path = graph.find_path("主体_A", "结果_C");
        assert!(path.is_some());

        let path = path.unwrap();
        assert_eq!(path.len(), 3);
        assert_eq!(path[0], "主体_A");
        assert_eq!(path[2], "结果_C");
    }

    #[test]
    fn test_get_stats() {
        let mut graph = KnowledgeGraph::new("测试图谱");

        graph.add_entities(vec![
            Entity::new("球员", EntityType::Subject),
            Entity::new("犯规", EntityType::Action),
        ]);

        graph.add_relation(Relation::new(
            "主体_球员",
            "动作_犯规",
            RelationType::Participates,
        ));

        let stats = graph.get_stats();

        assert_eq!(stats.node_count, 2);
        assert_eq!(stats.edge_count, 1);
        assert!(stats.avg_degree > 0.0);
    }

    #[test]
    fn test_to_dot() {
        let mut graph = KnowledgeGraph::new("测试图谱");

        graph.add_entity(Entity::new("A", EntityType::Subject));
        graph.add_entity(Entity::new("B", EntityType::Action));
        graph.add_relation(Relation::new("主体_A", "动作_B", RelationType::Causes));

        let dot = graph.to_dot();
        assert!(dot.contains("digraph"));
        assert!(dot.contains("主体_A"));
        assert!(dot.contains("动作_B"));
    }

    #[test]
    fn test_subgraph() {
        let mut graph = KnowledgeGraph::new("测试图谱");

        graph.add_entities(vec![
            Entity::new("A", EntityType::Subject),
            Entity::new("B", EntityType::Object),
            Entity::new("C", EntityType::Result),
        ]);

        let subgraph = graph.get_subgraph(&["主体_A".to_string(), "客体_B".to_string()]);
        assert_eq!(subgraph.node_count(), 2);
    }
}
