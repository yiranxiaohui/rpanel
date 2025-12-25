import { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { getAgentList } from '../api/api';
import type { Agent } from '../types';
import { translations, type Lang } from '../locales';
// import '../App.css'; // Imported in main.tsx or App.tsx

function formatBytes(bytes: number | null | undefined, decimals = 2) {
  if (bytes === null || bytes === undefined) return '-';
  if (bytes === 0) return '0 B';
  const k = 1024;
  const dm = decimals < 0 ? 0 : decimals;
  const sizes = ['KB', 'MB', 'GB', 'TB', 'PB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(dm)) + ' ' + sizes[i];
}

function formatPercent(value: number | null | undefined) {
  if (value === null || value === undefined) return '-';
  return `${value.toFixed(1)}%`;
}

function Dashboard() {
  const navigate = useNavigate();
  const [agents, setAgents] = useState<Agent[]>([]);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);
  const [lang, setLang] = useState<Lang>('zh');
  
  const t = translations[lang];

  const fetchAgents = async () => {
    try {
      const data = await getAgentList();
      data.sort((a: Agent, b: Agent) => {
        if (b.status !== a.status) return b.status - a.status;
        const timeA = a.last_update ? new Date(a.last_update).getTime() : 0;
        const timeB = b.last_update ? new Date(b.last_update).getTime() : 0;
        return timeB - timeA;
      });
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
    const interval = setInterval(fetchAgents, 5000);
    return () => clearInterval(interval);
  }, []);

  const toggleLang = () => {
    setLang(l => l === 'en' ? 'zh' : 'en');
  };

  return (
    <div className="dashboard-container">
      <header className="header">
        <h1>{t.title}</h1>
        <button className="lang-btn" onClick={toggleLang}>
          {lang === 'en' ? '中文' : 'English'}
        </button>
      </header>

      {loading && agents.length === 0 && <p>{t.loading}</p>}
      
      {error && <div className="error-message">{t.error}{error}</div>}

      <div className="agent-grid">
        {agents.map((agent) => {
          const memUsed = agent.mem_used || 0;
          const memTotal = agent.mem_total || 1; 
          const diskUsed = agent.disk_used || 0;
          const diskTotal = agent.disk_total || 1;
          const memPercent = (memUsed / memTotal) * 100;
          const diskPercent = (diskUsed / diskTotal) * 100;
          const isOnline = agent.status === 1;

          return (
            <div key={agent.id} className="agent-card" style={{ borderColor: isOnline ? '#333' : '#551111' }}>
              <div className="agent-header">
                <div style={{ display: 'flex', flexDirection: 'column' }}>
                    <span className="agent-name">{agent.name}</span>
                    <span className="agent-uuid">{agent.uuid}</span>
                </div>
                <span className={`status-badge`} style={{ 
                  backgroundColor: isOnline ? '#1a3c1a' : '#3c1a1a', 
                  color: isOnline ? '#4cc74c' : '#c74c4c',
                  borderColor: isOnline ? '#2d5a2d' : '#5a2d2d'
                }}>
                  {isOnline ? t.online : t.offline}
                </span>
              </div>
              
              <div className="agent-info">
                 <div className="info-row" title="IP Address">
                    <span className="label">{t.ip}:</span> {agent.ip_address || t.unknown}
                 </div>
                 <div className="info-row" title="OS Info">
                    <span className="label">{t.os}:</span> {agent.os_info || t.unknown}
                 </div>
              </div>

              <div className="stat-group">
                <div className="stat-row">
                  <span>{t.cpu}</span>
                  <span className="stat-value">{formatPercent(agent.cpu_usage)}</span>
                </div>
                <div className="progress-bar-bg">
                  <div className="progress-bar-fill" style={{ width: `${Math.min(agent.cpu_usage || 0, 100)}%` }}></div>
                </div>
              </div>

              <div className="stat-group">
                <div className="stat-row">
                  <span>{t.memory}</span>
                  <span className="stat-value">{formatBytes(agent.mem_used)} / {formatBytes(agent.mem_total)}</span>
                </div>
                <div className="progress-bar-bg mem-bar">
                  <div className="progress-bar-fill" style={{ width: `${Math.min(memPercent, 100)}%` }}></div>
                </div>
              </div>

              <div className="stat-group">
                <div className="stat-row">
                  <span>{t.disk}</span>
                  <span className="stat-value">{formatBytes(agent.disk_used)} / {formatBytes(agent.disk_total)}</span>
                </div>
                <div className="progress-bar-bg disk-bar">
                  <div className="progress-bar-fill" style={{ width: `${Math.min(diskPercent, 100)}%` }}></div>
                </div>
              </div>

              {isOnline && (
                  <div className="actions-row">
                      <button className="action-btn" onClick={() => navigate(`/docker/${agent.uuid}`)}>
                          Docker Management
                      </button>
                  </div>
              )}

              <div className="timestamp">
                {t.lastUpdated}: {agent.last_update ? new Date(agent.last_update).toLocaleString() : t.never}
              </div>
            </div>
          );
        })}
      </div>
      
      {!loading && agents.length === 0 && !error && (
        <p style={{ textAlign: 'center', color: '#666' }}>{t.noAgents}</p>
      )}

    </div>
  );
}

export default Dashboard;
