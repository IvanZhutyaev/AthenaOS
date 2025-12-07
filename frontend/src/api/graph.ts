import apiClient from './client'

export interface Node {
  id: string
  label: string
  properties: Record<string, any>
  created_at: number
  updated_at: number
  version: number
}

export interface Edge {
  id: string
  from: string
  to: string
  label: string
  properties: Record<string, any>
  created_at: number
  version: number
}

export interface QueryResult {
  nodes: Node[]
  edges: Edge[]
}

export const graphApi = {
  listNodes: async (): Promise<Node[]> => {
    const response = await apiClient.get('/nodes')
    return response.data.nodes
  },

  getNode: async (id: string): Promise<Node> => {
    const response = await apiClient.get(`/nodes/${id}`)
    return response.data
  },

  createNode: async (label: string, properties?: Record<string, any>): Promise<Node> => {
    const response = await apiClient.post('/nodes', { label, properties })
    return response.data
  },

  deleteNode: async (id: string): Promise<void> => {
    await apiClient.delete(`/nodes/${id}`)
  },

  listEdges: async (): Promise<Edge[]> => {
    const response = await apiClient.get('/edges')
    return response.data.edges
  },

  createEdge: async (from: string, to: string, label: string): Promise<Edge> => {
    const response = await apiClient.post('/edges', { from, to, label })
    return response.data
  },

  query: async (pattern: any): Promise<QueryResult> => {
    const response = await apiClient.post('/query', { pattern })
    return response.data
  },
}

