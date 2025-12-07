import { useQuery } from '@tanstack/react-query'
import apiClient from '../api/client'
import './Agents.css'

export default function Agents() {
  const { data: agents, isLoading } = useQuery({
    queryKey: ['agents'],
    queryFn: async () => {
      const response = await apiClient.get('/agents')
      return response.data.agents
    },
  })

  return (
    <div className="agents">
      <h1>Agents</h1>
      <div className="agents-info">
        <p>Manage and monitor your autonomous agents.</p>
      </div>

      {isLoading ? (
        <p>Loading agents...</p>
      ) : agents && agents.length > 0 ? (
        <div className="agents-list">
          {agents.map((agentId: string) => (
            <div key={agentId} className="agent-card">
              <h3>Agent {agentId.slice(0, 8)}...</h3>
              <p className="agent-id">ID: {agentId}</p>
              <div className="agent-status">
                <span className="status-indicator active"></span>
                <span>Active</span>
              </div>
            </div>
          ))}
        </div>
      ) : (
        <div className="empty-state">
          <p>No agents loaded.</p>
          <p>Load an agent to get started!</p>
        </div>
      )}
    </div>
  )
}

