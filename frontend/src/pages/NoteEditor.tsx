import { useState, useEffect } from 'react'
import { useParams, useNavigate } from 'react-router-dom'
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query'
import { graphApi } from '../api/graph'
import ReactMarkdown from 'react-markdown'
import './NoteEditor.css'

export default function NoteEditor() {
  const { id } = useParams<{ id: string }>()
  const navigate = useNavigate()
  const queryClient = useQueryClient()
  const [isEditing, setIsEditing] = useState(false)
  const [content, setContent] = useState('')
  const [preview, setPreview] = useState(true)

  const { data: node, isLoading } = useQuery({
    queryKey: ['node', id],
    queryFn: () => (id ? graphApi.getNode(id) : null),
    enabled: !!id,
  })

  useEffect(() => {
    if (node?.properties?.content) {
      setContent(String(node.properties.content))
    }
  }, [node])

  const updateNodeMutation = useMutation({
    mutationFn: async (newContent: string) => {
      if (!id || !node) return
      // In production, would update node via API
      return { id, content: newContent }
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['node', id] })
      setIsEditing(false)
    },
  })

  const handleSave = () => {
    updateNodeMutation.mutate(content)
  }

  if (isLoading) {
    return <div className="note-editor">Loading...</div>
  }

  if (!node) {
    return (
      <div className="note-editor">
        <p>Node not found</p>
        <button onClick={() => navigate('/graph')}>Back to Graph</button>
      </div>
    )
  }

  return (
    <div className="note-editor">
      <div className="editor-header">
        <h1>{node.label}</h1>
        <div className="editor-actions">
          <button
            onClick={() => setPreview(!preview)}
            className="toggle-preview"
          >
            {preview ? 'Edit' : 'Preview'}
          </button>
          {isEditing && (
            <>
              <button onClick={handleSave} className="save-button">
                Save
              </button>
              <button
                onClick={() => {
                  setIsEditing(false)
                  if (node.properties?.content) {
                    setContent(String(node.properties.content))
                  }
                }}
                className="cancel-button"
              >
                Cancel
              </button>
            </>
          )}
          {!isEditing && (
            <button
              onClick={() => setIsEditing(true)}
              className="edit-button"
            >
              Edit
            </button>
          )}
        </div>
      </div>

      <div className="editor-content">
        {isEditing || !preview ? (
          <textarea
            value={content}
            onChange={(e) => setContent(e.target.value)}
            className="markdown-editor"
            placeholder="Write your note in Markdown..."
          />
        ) : (
          <div className="markdown-preview">
            <ReactMarkdown>{content || '*No content*'}</ReactMarkdown>
          </div>
        )}
      </div>
    </div>
  )
}

