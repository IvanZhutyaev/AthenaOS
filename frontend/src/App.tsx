import { BrowserRouter, Routes, Route } from 'react-router-dom'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import Dashboard from './pages/Dashboard'
import GraphView from './pages/GraphView'
import Agents from './pages/Agents'
import NoteEditor from './pages/NoteEditor'
import Layout from './components/Layout'
import './App.css'

const queryClient = new QueryClient()

function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <BrowserRouter>
        <Layout>
          <Routes>
            <Route path="/" element={<Dashboard />} />
            <Route path="/graph" element={<GraphView />} />
            <Route path="/note/:id" element={<NoteEditor />} />
            <Route path="/agents" element={<Agents />} />
          </Routes>
        </Layout>
      </BrowserRouter>
    </QueryClientProvider>
  )
}

export default App

