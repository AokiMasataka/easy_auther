import { getRefreshToken, getToken, setToken } from "../cookie.ts";


class EzAutherClient {
    readonly HOST: string;
    readonly PORT: number;

    constructor(host: string, port: number) {
        this.HOST = host;
        this.PORT = port;
    };

    prefix(): string {
        return "http://" + this.HOST + ":" + String(this.PORT) + "/";
    };

    async get(endpoint: string): Promise<Response> {
        const url = this.prefix() + endpoint;
        const token = getToken();
        const request = {
            method: "GET",
            headers: {
                'Content-Type': 'application/json',
                'X-Authorization': token
            },
        };
        
        try {
            const response = await fetch(url, request);
            
            if (response.status == 401) {
                const response = await fetch(url, request);
            }
            
            if (!response.ok) {
                throw new Error(`status: ${response.status}`);
            };

            return response;
        } catch (error) {
            console.log(error);
            throw error;
        };
    };

    async post(endpoint: string, body?: any): Promise<Response> {
        const url = this.prefix() + endpoint;
        const token = getToken();
        const request = {
            method: "POST",
            headers: {
                'Content-Type': 'application/json',
                'X-Authorization': token
            },
            body: JSON.stringify(body)
        };

        const response = await fetch(url, request);
        if (response.status == 401) {
            await this.refresh();
            const response = await fetch(url, request);
            if (!response.ok) {
                window.location.href = 'http://localhost:3000/login';
                throw new Error(`HTTP Error: ${response.status}`);
            };
        };

        return response;
    };

    async delete(endpoint: string): Promise<Response> {
        const url = this.prefix() + endpoint;
        const token = getToken();
        const request = {
            method: "DELETE",
            headers: {
                'Content-Type': 'application/json',
                'X-Authorization': token
            },
        };
        
        try {
            const response = await fetch(url, request);
            if (!response.ok) {
                throw new Error(`status: ${response.status}`);
            };

            return response;
        } catch (error) {
            throw error;
        };
    };

    async login(name: string, pass: string): Promise<Response> {
        const url = this.prefix() + "login";
        const request = {
            method: "POST",
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({name: name, pass: pass})
        };

        const response = await fetch(url, request);
        return response;
    }

    async refresh() {
        const url = this.prefix() + "refresh";
        const refreshToken = getRefreshToken();
        const request = {
            method: "POST",
            headers: {
                'Content-Type': 'application/json',
                'X-Authorization': refreshToken
            },
        };

        const response = await fetch(url, request);
        
        if (!response.ok){
            window.location.href = 'http://localhost:3000/login';
            throw new Error(`HTTP Error: ${response.status}`);
        };
        const jwt = (await response.json()).jwt;
        setToken(jwt);
    };
}


export const client = new EzAutherClient("localhost", 8080);
