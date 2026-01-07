import axios from 'axios';
import type { Agent, ContainerInfo, ImageInfo } from '../types';

const api = axios.create({
    baseURL: '/api/v1',
    timeout: 10000,
});

export const getAgentList = async (): Promise<Agent[]> => {
    const res = await api.get('/agent/list');
    return res.data;
};

export const pullImage = async (agent_id: string, image: string) => {
    return api.post('/image/pull', { agent_id, image });
};

export const getContainers = async (agent_id: string): Promise<ContainerInfo[]> => {
    const res = await api.get(`/agent/${agent_id}/docker/containers`);
    // The backend returns a JSON string in "content", or null.
    // Wait, the backend returns Json<Option<String>>.
    // If it's a string, we need to parse it? 
    // Wait, "content" is JSON string.
    // So res.data is String (the content field content) or null.
    // But axios might auto parse if Content-Type is application/json?
    // My backend returns `Json(info.map(|i| i.content)`.
    // If content is `[{"id":...}]`, then res.data will be that string?
    // Actually, `Json(...)` serializes the String as a JSON String?
    // If content is already a JSON string, and I wrap it in Json(String), it becomes a double-encoded string: `"[{"id":...}]"`.
    // I should probably have returned `Json(serde_json::from_str(&content))` in backend or just `content` as raw response with correct content-type.
    // However, SeaORM content is just string.
    // Let's assume double encoded for now and parse it.
    // OR, I can fix the backend to return `impl IntoResponse` using `axum::response::Json(serde_json::from_str::<Vec<ContainerInfo>>(&info.content).unwrap_or_default())`.
    
    // Let's fix backend later if needed. For now, try to parse.
    // If res.data is a string, parse it.
    let data = res.data;
    if (typeof data === 'string') {
        try {
            return JSON.parse(data);
        } catch (e) {
            console.error("Failed to parse containers", e);
            return [];
        }
    }
    return data || [];
};

export const getImages = async (agent_id: string): Promise<ImageInfo[]> => {
    const res = await api.get(`/agent/${agent_id}/docker/images`);
    let data = res.data;
    if (typeof data === 'string') {
        try {
            return JSON.parse(data);
        } catch (e) {
            console.error("Failed to parse images", e);
            return [];
        }
    }
    return data || [];
};

export const refreshDocker = async (agent_id: string) => {
    return api.post(`/agent/${agent_id}/docker/refresh`);
};

export const startContainer = async (agent_id: string, container_id: string) => {
    return api.post(`/agent/${agent_id}/docker/container/start`, { container_id });
};

export const stopContainer = async (agent_id: string, container_id: string) => {
    return api.post(`/agent/${agent_id}/docker/container/stop`, { container_id });
};

export const removeContainer = async (agent_id: string, container_id: string) => {
    return api.post(`/agent/${agent_id}/docker/container/remove`, { container_id, force: true });
};

export const removeImage = async (agent_id: string, image_id: string) => {
    return api.post(`/agent/${agent_id}/docker/image/remove`, { image_id, force: true });
};

export const runContainer = async (agent_id: string, image: string, command: string, name?: string, ports?: {host: string, container: string}[]) => {
    // ports needs to be converted to Vec<(String, String)>
    const portsVec = ports ? ports.map(p => [p.host, p.container]) : undefined;
    return api.post(`/agent/${agent_id}/docker/run`, {
        image,
        command: command || undefined,
        name: name || undefined,
        ports: portsVec
    });
};

export default api;
