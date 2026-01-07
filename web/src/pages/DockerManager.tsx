import { useEffect, useState } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { 
    getContainers, getImages, refreshDocker, 
    startContainer, stopContainer, removeContainer, 
    removeImage, runContainer, pullImage 
} from '../api/api';
import type { ContainerInfo, ImageInfo } from '../types';
import '../App.css'; // Reuse existing styles for now, add specific ones later

const DockerManager = () => {
    const { uuid } = useParams<{ uuid: string }>();
    const navigate = useNavigate();
    const [containers, setContainers] = useState<ContainerInfo[]>([]);
    const [images, setImages] = useState<ImageInfo[]>([]);
    const [activeTab, setActiveTab] = useState<'containers' | 'images'>('containers');
    const [loading, setLoading] = useState(false);
    
    // Modal states
    const [showRunModal, setShowRunModal] = useState(false);
    const [runImage, setRunImage] = useState('');
    const [runCommand, setRunCommand] = useState('');
    const [runName, setRunName] = useState('');
    const [runPorts, setRunPorts] = useState(''); // "8080:80,9000:9000"

    const [showPullModal, setShowPullModal] = useState(false);
    const [pullImageName, setPullImageName] = useState('');

    useEffect(() => {
        if (uuid) {
            loadData();
        }
    }, [uuid]);

    const loadData = async () => {
        if (!uuid) return;
        setLoading(true);
        try {
            const [c, i] = await Promise.all([getContainers(uuid), getImages(uuid)]);
            setContainers(c);
            setImages(i);
        } catch (e) {
            console.error(e);
        } finally {
            setLoading(false);
        }
    };

    const handleRefresh = async () => {
        if (!uuid) return;
        setLoading(true);
        try {
            await refreshDocker(uuid);
            // Wait a bit for agent to push data? Or just rely on re-fetching.
            // Since it's async, we might not get it immediately.
            // Let's wait 1s then fetch.
            setTimeout(loadData, 2000);
        } catch (e) {
            console.error(e);
            setLoading(false);
        }
    };

    const handleStart = async (id: string) => {
        if (!uuid) return;
        await startContainer(uuid, id);
        handleRefresh();
    };

    const handleStop = async (id: string) => {
        if (!uuid) return;
        await stopContainer(uuid, id);
        handleRefresh();
    };

    const handleRemoveContainer = async (id: string) => {
        if (!uuid || !confirm('Are you sure?')) return;
        await removeContainer(uuid, id);
        handleRefresh();
    };

    const handleRemoveImage = async (id: string) => {
        if (!uuid || !confirm('Are you sure?')) return;
        await removeImage(uuid, id);
        handleRefresh();
    };

    const handleRunContainer = async () => {
        if (!uuid) return;
        try {
            const ports = runPorts.split(',').filter(s => s.trim()).map(s => {
                const [host, container] = s.split(':');
                return { host, container };
            });
            await runContainer(uuid, runImage, runCommand, runName, ports);
            setShowRunModal(false);
            setRunImage('');
            setRunCommand('');
            setRunName('');
            setRunPorts('');
            handleRefresh();
        } catch (e) {
            alert('Failed to run container');
        }
    };

    const handlePullImage = async () => {
        if (!uuid) return;
        try {
            await pullImage(uuid, pullImageName);
            alert('Pull started, check main dashboard logs or wait for refresh.');
            setShowPullModal(false);
            setPullImageName('');
            handleRefresh();
        } catch (e) {
            alert('Failed to pull image');
        }
    };

    return (
        <div className="dashboard-container">
            <header className="header">
                <button onClick={() => navigate('/')} className="action-btn" style={{marginRight: '1rem'}}>Back</button>
                <h1>Docker Manager - {uuid}</h1>
                <div style={{marginLeft: 'auto'}}>
                    <button className="action-btn" onClick={handleRefresh} disabled={loading}>
                        {loading ? 'Refreshing...' : 'Refresh'}
                    </button>
                </div>
            </header>

            <div className="tabs">
                <button 
                    className={`tab ${activeTab === 'containers' ? 'active' : ''}`}
                    onClick={() => setActiveTab('containers')}
                >
                    Containers
                </button>
                <button 
                    className={`tab ${activeTab === 'images' ? 'active' : ''}`}
                    onClick={() => setActiveTab('images')}
                >
                    Images
                </button>
            </div>

            <div className="tab-content">
                {activeTab === 'containers' && (
                    <div>
                        <div style={{marginBottom: '1rem'}}>
                            <button className="action-btn" onClick={() => setShowRunModal(true)}>Run New Container</button>
                        </div>
                        <table className="data-table">
                            <thead>
                                <tr>
                                    <th>Name</th>
                                    <th>Image</th>
                                    <th>State</th>
                                    <th>Status</th>
                                    <th>Ports</th>
                                    <th>Actions</th>
                                </tr>
                            </thead>
                            <tbody>
                                {containers.map(c => (
                                    <tr key={c.id}>
                                        <td>{c.names.join(', ')}</td>
                                        <td title={c.image_id}>{c.image}</td>
                                        <td>{c.state}</td>
                                        <td>{c.status}</td>
                                        <td>{c.ports.map(p => `${p.public_port || ''}->${p.private_port}/${p.type_}`).join(', ')}</td>
                                        <td>
                                            {c.state === 'running' ? (
                                                <button className="action-btn danger" onClick={() => handleStop(c.id)}>Stop</button>
                                            ) : (
                                                <button className="action-btn" onClick={() => handleStart(c.id)}>Start</button>
                                            )}
                                            <button className="action-btn danger" onClick={() => handleRemoveContainer(c.id)}>Delete</button>
                                        </td>
                                    </tr>
                                ))}
                            </tbody>
                        </table>
                    </div>
                )}

                {activeTab === 'images' && (
                    <div>
                         <div style={{marginBottom: '1rem'}}>
                            <button className="action-btn" onClick={() => setShowPullModal(true)}>Pull Image</button>
                        </div>
                        <table className="data-table">
                            <thead>
                                <tr>
                                    <th>ID</th>
                                    <th>Tags</th>
                                    <th>Size</th>
                                    <th>Created</th>
                                    <th>Actions</th>
                                </tr>
                            </thead>
                            <tbody>
                                {images.map(i => (
                                    <tr key={i.id}>
                                        <td title={i.id}>{i.id.substring(7, 19)}</td>
                                        <td>{i.repo_tags.join(', ')}</td>
                                        <td>{(i.size / 1024 / 1024).toFixed(2)} MB</td>
                                        <td>{new Date(i.created * 1000).toLocaleString()}</td>
                                        <td>
                                            <button className="action-btn danger" onClick={() => handleRemoveImage(i.id)}>Delete</button>
                                        </td>
                                    </tr>
                                ))}
                            </tbody>
                        </table>
                    </div>
                )}
            </div>

            {showRunModal && (
                <div className="modal-overlay">
                    <div className="modal">
                        <h3>Run Container</h3>
                        <div className="form-group">
                            <label>Image:</label>
                            <input type="text" className="modal-input" value={runImage} onChange={e => setRunImage(e.target.value)} />
                        </div>
                        <div className="form-group">
                            <label>Name (Optional):</label>
                            <input type="text" className="modal-input" value={runName} onChange={e => setRunName(e.target.value)} />
                        </div>
                        <div className="form-group">
                            <label>Command (Optional):</label>
                            <input type="text" className="modal-input" value={runCommand} onChange={e => setRunCommand(e.target.value)} />
                        </div>
                        <div className="form-group">
                            <label>Ports (Host:Container, comma sep):</label>
                            <input type="text" className="modal-input" value={runPorts} onChange={e => setRunPorts(e.target.value)} placeholder="8080:80, 9000:9000" />
                        </div>
                        <div className="modal-actions">
                            <button className="modal-btn confirm" onClick={handleRunContainer}>Run</button>
                            <button className="modal-btn cancel" onClick={() => setShowRunModal(false)}>Cancel</button>
                        </div>
                    </div>
                </div>
            )}

            {showPullModal && (
                <div className="modal-overlay">
                    <div className="modal">
                        <h3>Pull Image</h3>
                         <div className="form-group">
                            <label>Image Name:</label>
                            <input type="text" className="modal-input" value={pullImageName} onChange={e => setPullImageName(e.target.value)} />
                        </div>
                         <div className="modal-actions">
                            <button className="modal-btn confirm" onClick={handlePullImage}>Pull</button>
                            <button className="modal-btn cancel" onClick={() => setShowPullModal(false)}>Cancel</button>
                        </div>
                    </div>
                </div>
            )}
        </div>
    );
};

export default DockerManager;
