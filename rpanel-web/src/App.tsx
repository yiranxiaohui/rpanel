import { useEffect, useState } from 'react';
import './App.css';

interface SystemStatus {
  id: number;
  agent_id: string;
  cpu_usage: number;
  mem_used: number;
  mem_total: number;
  disk_used: number;
  disk_total: number;
  create_time: string;
  update_time: string;
}

function formatBytes(bytes: number, decimals = 2) {
  if (bytes === 0) return '0 B';
  const k = 1024;
  const dm = decimals < 0 ? 0 : decimals;
  const sizes = ['GB', 'TB', 'PB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(dm)) + ' ' + sizes[i];
}

function formatPercent(value: number) {
  return `${value.toFixed(1)}%`;
}

function App() {
  const [agents, setAgents] = useState<SystemStatus[]>([]);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);

  const fetchAgents = async () => {
    try {
      const response = await fetch('/api/v1/agent/list');
      if (!response.ok) {
        throw new Error(`Error fetching agents: ${response.statusText}`);
      }
      const data = await response.json();
      // Sort by update_time desc
      data.sort((a: SystemStatus, b: SystemStatus) => new Date(b.update_time).getTime() - new Date(a.update_time).getTime());
      setAgents(data);
      setError(null);
    } catch (err) {
      console.error(err);
      setError(err instanceof Error ? err.message : 'Unknown error');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchAgents();
    const interval = setInterval(fetchAgents, 5000); // Auto-refresh every 5s
    return () => clearInterval(interval);
  }, []);

  return (
    <div className="dashboard-container">
      <header className="header">
        <h1>Agent Monitor</h1>
      </header>

      {loading && agents.length === 0 && <p>Loading agents...</p>}
      
      {error && <div className="error-message">Error: {error}</div>}

      <div className="agent-grid">
        {agents.map((agent) => {
          const memPercent = (agent.mem_used / agent.mem_total) * 100;
          const diskPercent = (agent.disk_used / agent.disk_total) * 100;
          const isOnline = (Date.now() - new Date(agent.update_time).getTime()) < 30000; // < 30s considered online

          return (
            <div key={agent.id} className="agent-card" style={{ borderColor: isOnline ? '#333' : '#551111' }}>
              <div className="agent-header">
                <span className="agent-name">{agent.agent_id}</span>
                <span className={`status-badge`} style={{ 
                  backgroundColor: isOnline ? '#1a3c1a' : '#3c1a1a', 
                  color: isOnline ? '#4cc74c' : '#c74c4c',
                  borderColor: isOnline ? '#2d5a2d' : '#5a2d2d'
                }}>
                  {isOnline ? 'Online' : 'Offline'}
                </span>
              </div>

              {/* CPU */}
              <div className="stat-group">
                <div className="stat-row">
                  <span>CPU Usage</span>
                  <span className="stat-value">{formatPercent(agent.cpu_usage)}</span>
                </div>
                <div className="progress-bar-bg">
                  <div 
                    className="progress-bar-fill" 
                    style={{ width: `${Math.min(agent.cpu_usage, 100)}%` }}
                  ></div>
                </div>
              </div>

              {/* Memory */}
              <div className="stat-group">
                <div className="stat-row">
                  <span>Memory</span>
                  <span className="stat-value">{formatBytes(agent.mem_used)} / {formatBytes(agent.mem_total)}</span>
                </div>
                <div className="progress-bar-bg mem-bar">
                  <div 
                    className="progress-bar-fill" 
                    style={{ width: `${Math.min(memPercent, 100)}%` }}
                  ></div>
                </div>
              </div>

              {/* Disk */}
              <div className="stat-group">
                <div className="stat-row">
                  <span>Disk</span>
                  <span className="stat-value">{formatBytes(agent.disk_used)} / {formatBytes(agent.disk_total)}</span>
                </div>
                <div className="progress-bar-bg disk-bar">
                  <div 
                    className="progress-bar-fill" 
                    style={{ width: `${Math.min(diskPercent, 100)}%` }}
                  ></div>
                </div>
              </div>

              <div className="timestamp">
                Last updated: {new Date(agent.update_time).toLocaleString()}
              </div>
            </div>
          );
        })}
      </div>
      
      {!loading && agents.length === 0 && !error && (
        <p style={{ textAlign: 'center', color: '#666' }}>No agents connected yet.</p>
      )}
    </div>
  );
}

export default App;