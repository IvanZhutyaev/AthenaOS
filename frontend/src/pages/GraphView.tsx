import { useState } from 'react'
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query'
import { graphApi } from '../api/graph'
import './GraphView.css'

export default function GraphView() {
  const [newNodeLabel, setNewNodeLabel] = useState('')
  const queryClient = useQueryClient()

  const { data: nodes, isLoading } = useQuery({
    queryKey: ['nodes'],
    queryFn: graphApi.listNodes,
  })

  const createNodeMutation = useMutation({
    mutationFn: (label: string) => graphApi.createNode(label),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['nodes'] })
      setNewNodeLabel('')
    },
  })

  const handleCreateNode = (e: React.FormEvent) => {
    e.preventDefault()
    if (newNodeLabel.trim()) {
      createNodeMutation.mutate(newNodeLabel.trim())
    }
  }

  return (
    <div className="graph-view">
      <div className="graph-header">
        <h1>Knowledge Graph</h1>
        <form onSubmit={handleCreateNode} className="create-node-form">
          <input
            type="text"
            value={newNodeLabel}
            onChange={(e) => setNewNodeLabel(e.target.value)}
            placeholder="Enter node label..."
            className="node-input"
          />
          <button type="submit" className="create-button">
            Create Node
          </button>
        </form>
      </div>

      {isLoading ? (
        <p>Loading graph...</p>
      ) : nodes && nodes.length > 0 ? (
        <div className="graph-nodes">
          {nodes.map((node) => (
            <div key={node.id} className="graph-node">
              <h3>{node.label}</h3>
              <p className="node-meta">ID: {node.id.slice(0, 8)}...</p>
              <p className="node-meta">
                Created: {new Date(node.created_at * 1000).toLocaleDateString()}
              </p>
            </div>
          ))}
        </div>
      ) : (
        <div className="empty-state">
          <p>No nodes in the graph yet.</p>
          <p>Create your first node to get started!</p>
        </div>
      )}
    </div>
  )
}

