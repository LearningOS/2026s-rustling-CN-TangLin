/*
	bfs
	This problem requires you to implement a basic BFS algorithm
*/

use std::collections::VecDeque;

// Define a graph (邻接表表示)
struct Graph {
    adj: Vec<Vec<usize>>, // adj[i] 存储节点i的所有邻接节点
}

impl Graph {
    // Create a new graph with n vertices
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![vec![]; n],
        }
    }

    // Add an edge to the graph (无向边：双向添加)
    fn add_edge(&mut self, src: usize, dest: usize) {
        self.adj[src].push(dest);
        self.adj[dest].push(src);
    }

    // Perform a breadth-first search on the graph, return the order of visited nodes
    fn bfs_with_return(&self, start: usize) -> Vec<usize> {
        // 1. 初始化访问标记数组：false表示未访问，长度等于节点数
        let mut visited = vec![false; self.adj.len()];
        // 2. 初始化队列（BFS核心数据结构），存入起始节点
        let mut queue = VecDeque::new();
        // 3. 初始化访问顺序列表
        let mut visit_order = vec![];

        // 边界：起始节点超出范围（测试用例不会触发，仅鲁棒性）
        if start >= self.adj.len() {
            return visit_order;
        }

        // 4. 标记起始节点为已访问，并加入队列
        visited[start] = true;
        queue.push_back(start);

        // 5. 核心BFS循环：队列非空时持续处理
        while let Some(current) = queue.pop_front() {
            // 将当前节点加入访问顺序
            visit_order.push(current);

            // 遍历当前节点的所有邻接节点
            for &neighbor in &self.adj[current] {
                // 未访问的邻接节点：标记+入队
                if !visited[neighbor] {
                    visited[neighbor] = true;
                    queue.push_back(neighbor);
                }
            }
        }

        visit_order
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs_all_nodes_visited() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 4);
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(1, 4);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 4, 2, 3]);
    }

    #[test]
    fn test_bfs_different_start() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let visited_order = graph.bfs_with_return(2);
        assert_eq!(visited_order, vec![2, 1, 0]);
    }

    #[test]
    fn test_bfs_with_cycle() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 2]);
    }

    #[test]
    fn test_bfs_single_node() {
        let mut graph = Graph::new(1);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0]);
    }
}