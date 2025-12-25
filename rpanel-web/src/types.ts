export interface Agent {
  id: number;
  uuid: string;
  name: string;
  host_name: string | null;
  ip_address: string | null;
  os_info: string | null;
  version: string | null;
  status: number; // 0: offline, 1: online
  cpu_usage: number | null;
  mem_used: number | null;
  mem_total: number | null;
  disk_used: number | null;
  disk_total: number | null;
  last_update: string | null;
}

export interface Port {
    ip?: string;
    private_port: number;
    public_port?: number;
    type_: string;
}

export interface ContainerInfo {
    id: string;
    names: string[];
    image: string;
    image_id: string;
    command: string;
    created: number;
    state: string;
    status: string;
    ports: Port[];
}

export interface ImageInfo {
    id: string;
    repo_tags: string[];
    created: number;
    size: number;
}
