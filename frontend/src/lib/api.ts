const BASE_URL = 'http://127.0.0.1:3000';

export interface User {
  username: string;
  name: string;
  created: string;
}

export interface Blog {
  user: User;
  tweet: string;
  timestamp: string;
}

export interface CreateUserRequest {
  username: string;
  name: string;
}

export interface CreateBlogRequest {
  username: string;
  name: string;
  tweet: string;
}

async function request<T>(url: string, init?: RequestInit): Promise<T> {
  let res: Response;
  try {
    res = await fetch(url, init);
  } catch {
    throw new Error('Cannot connect to backend. Is the Rust server running on port 3000?');
  }
  if (!res.ok) {
    const text = await res.text().catch(() => '');
    throw new Error(`HTTP ${res.status}${text ? ' — ' + text : ''}`);
  }
  return res.json() as Promise<T>;
}

export const createUser = (data: CreateUserRequest): Promise<User> =>
  request<User>(`${BASE_URL}/user`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(data)
  });

export const getUser = (username: string): Promise<User> =>
  request<User>(`${BASE_URL}/user?username=${encodeURIComponent(username)}`);

export const createBlog = (data: CreateBlogRequest): Promise<Blog> =>
  request<Blog>(`${BASE_URL}/blog`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(data)
  });

// Returns all blog posts, newest first.
export const getBlogs = (): Promise<Blog[]> =>
  request<Blog[]>(`${BASE_URL}/blog`);
