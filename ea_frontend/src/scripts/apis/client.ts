import { setToken, getToken } from "../cookie.ts";
import { UserInfo } from "../types.ts";
import router from "../../router.ts";


export class ApiError extends Error {
  constructor(
    message: string,
    public status: number,
    public body?: unknown
  ) {
    super(message);
    this.name = "ApiError";
  }
}

export class NetworkError extends Error {
  constructor(message: string) {
    super(message);
    this.name = "NetworkError";
  }
}

class EzAuthClient {
    readonly baseUrl: string;

    constructor(baseUrl: string) {
        this.baseUrl = baseUrl;
    };

    async fetch(method: string, endpoint: string, headers: any, body?: any): Promise<Response> {
        const url = this.baseUrl + endpoint;
        const params: any = {
            method: method,
            headers: headers,
        };
        if (body) {
            params.body = JSON.stringify(body);
        }
        
        try {
            const response = await fetch(url, params);
            if (!response.ok) {
                throw new ApiError(
                    `API error: [${response.status}] ${response.statusText}`,
                    response.status,
                    await response.json()
                );
            }
            return response;
        } catch (error) {
            if (error instanceof ApiError) {
                console.log('API error occurred:', error);
                if (error.status === 401) {
                    router.push("/login");
                }
                throw error;
            } else {
                console.log('Network error occurred:', error);
                throw new NetworkError(`Network error: ${error}`);
            }
        }
    }

    async login(email: string, pass: string): Promise<void> {
        const endpoint = "/manage/login";
        const headers = {'Content-Type': 'application/json'};
        const body = {email: email, pass: pass};
        const response = await this.fetch("POST", endpoint, headers, body).then(
            (response) => response,
            (error) => { throw error;  }
        );
        
        const jwt = (await response.json()).jwt;
        setToken(jwt);
    }

    async register(name: string, email: string, pass: string): Promise<void> {
        const endpoint = "/manage/register";
        const headers = {'Content-Type': 'application/json'};
        const body = {name: name, email: email, pass: pass};


        const _ = await this.fetch("POST", endpoint, headers, body).then(
            (response) => response,
            (error) => { throw error; }
        );
    }

    async listManager(): Promise<UserInfo[]> {
        const endpoint = "/manage/managers";
        const headers = {
            'Content-Type': 'application/json',
            'Authorization': `Bearer ${getToken()}`
        };

        const response = await this.fetch("GET", endpoint, headers).then(
            (response) => response,
            (error) => { console.log(error.status); throw error;  }
        );

        const json = await response.json();
        return json["users"];
    };

    async getManager(id: string): Promise<UserInfo> {
        const endpoint = `/manage/managers/${id}`;
        const headers = {
            'Content-Type': 'application/json',
            'Authorization': `Bearer ${getToken()}`
        };

        const response = await this.fetch("GET", endpoint, headers).then(
            (response) => response,
            (error) => { throw error;  }
        );

        return await response.json();
    }

    async deleteManager(id: string): Promise<void> {
        const endpoint = `/manage/managers/${id}`;
        const headers = {
            'Content-Type': 'application/json',
            'Authorization': `Bearer ${getToken()}`
        };

        const _ = await this.fetch("DELETE", endpoint, headers).then(
            (response) => response,
            (error) => { throw error;  }
        );
    }

    async createUser(name: string, email: string, pass: string): Promise<void> {
        const endpoint = "/manage/users";
        const headers = {
            'Content-Type': 'application/json',
            'Authorization': `Bearer ${getToken()}`
        };
        const body = {name: name, email: email, pass: pass};
        const _ = await this.fetch("POST", endpoint, headers, body).then(
            (response) => response,
            (error) => { throw error; }
        );
    }

    async listUser(): Promise<UserInfo[]> {
        const endpoint = "/manage/users";
        const headers = {
            'Content-Type': 'application/json',
            'Authorization': `Bearer ${getToken()}`
        };

        const response = await this.fetch("GET", endpoint, headers).then(
            (response) => response,
            (error) => { console.log(error.status); throw error;  }
        );

        const json = await response.json();
        return json["users"];
    };


    async getUser(id: string): Promise<UserInfo> {
        const endpoint = `/manage/users/${id}`;
        const headers = {
            'Content-Type': 'application/json',
            'Authorization': `Bearer ${getToken()}`
        };

        const response = await this.fetch("GET", endpoint, headers).then(
            (response) => response,
            (error) => { throw error;  }
        );

        return await response.json();
    }

    async deleteUser(id: string): Promise<void> {
        const endpoint = `/manage/users/${id}`;
        const headers = {
            'Content-Type': 'application/json',
            'Authorization': `Bearer ${getToken()}`
        };

        const _ = await this.fetch("DELETE", endpoint, headers).then(
            (response) => response,
            (error) => { throw error;  }
        );
    }
}


export const ezAuthClient = new EzAuthClient("http://localhost:8080");