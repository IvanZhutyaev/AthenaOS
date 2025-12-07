import { Link, useLocation } from 'react-router-dom'
import './Layout.css'

interface LayoutProps {
  children: React.ReactNode
}

export default function Layout({ children }: LayoutProps) {
  const location = useLocation()

  return (
    <div className="layout">
      <nav className="sidebar">
        <div className="logo">
          <h1>Athena OS</h1>
        </div>
        <ul className="nav-menu">
          <li>
            <Link
              to="/"
              className={location.pathname === '/' ? 'active' : ''}
            >
              Dashboard
            </Link>
          </li>
          <li>
            <Link
              to="/graph"
              className={location.pathname === '/graph' ? 'active' : ''}
            >
              Knowledge Graph
            </Link>
          </li>
          <li>
            <Link
              to="/agents"
              className={location.pathname === '/agents' ? 'active' : ''}
            >
              Agents
            </Link>
          </li>
        </ul>
      </nav>
      <main className="content">{children}</main>
    </div>
  )
}

