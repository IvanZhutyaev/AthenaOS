import { useQuery } from '@tanstack/react-query'
import { graphApi } from '../api/graph'
import './Dashboard.css'

export default function Dashboard() {
  const { data: nodes, isLoading } = useQuery({
    queryKey: ['nodes'],
    queryFn: graphApi.listNodes,
  })

  return (
    <div className="dashboard">
      <h1>Dashboard</h1>
      <div className="stats">
        <div className="stat-card">
          <h3>Nodes</h3>
          <p className="stat-value">{isLoading ? '...' : nodes?.length || 0}</p>
        </div>
        <div className="stat-card">
          <h3>Status</h3>
          <p className="stat-value">Online</p>
        </div>
      </div>
      <div className="recent-nodes">
        <h2>Recent Nodes</h2>
        {isLoading ? (
          <p>Loading...</p>
        ) : nodes && nodes.length > 0 ? (
          <ul className="node-list">
            {nodes.slice(0, 10).map((node) => (
              <li key={node.id} className="node-item">
                <span className="node-label">{node.label}</span>
                <span className="node-id">{node.id.slice(0, 8)}...</span>
              </li>
            ))}
          </ul>
        ) : (
          <p>No nodes yet. Create your first node!</p>
        )}
      </div>
    </div>
  )
}

