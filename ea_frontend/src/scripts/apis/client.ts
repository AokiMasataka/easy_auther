import { getRefreshToken, getToken, setToken } from "../cookie.ts";


export class FetchError extends Error {
    code: number
    constructor(message: string, code: number) {
        super(message);
        this.code = code;
    }
}


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

    async refresh_token(): Promise<Response> {
        const refresh_token = getRefreshToken();

        const params = {
            method: "POST",
            headers: {
                'Content-Type': 'application/json',
                'Authorization': refresh_token
            },
        }
        const response = await fetch(this.prefix() + "refresh", params);

        const jwt = (await response.json()).jwt;
        setToken(jwt);

        if (!response.ok){
            throw new FetchError("", response.status);
        };

        return response;
    }

    async get(endpoint: string): Promise<Response> {
        const url = this.prefix() + endpoint;
        const token = getToken();
        const params = {
            method: "GET",
            headers: {
                'Content-Type': 'application/json',
                'Authorization': token
            },
        };

        const response = await fetch(url, params);
        
        if (!response.ok) {
            if (response.status == 401){
                const ref_response = await this.refresh_token();
                if (!ref_response.ok) {
                    throw new FetchError("UnAuth", 401)
                }
                return await this.get(endpoint);
            };
            throw new FetchError("", response.status);
        };

        return response;
    };

    async post(endpoint: string, body?: any): Promise<Response> {
        const url = this.prefix() + endpoint;
        const token = getToken();
        const params = {
            method: "POST",
            headers: {
                'Content-Type': 'application/json',
                'Authorization': token
            },
            body: JSON.stringify(body)
        };

        const response = await fetch(url, params);
        
        if (!response.ok) {
            if (response.status == 401){
                const ref_response = await this.refresh_token();
                if (!ref_response.ok) {
                    throw new FetchError("UnAuth", 401)
                }
                return await this.post(endpoint, body);
            };
            throw new FetchError("", response.status);
        };

        return response;
    };

    async delete(endpoint: string): Promise<Response> {
        const url = this.prefix() + endpoint;
        const token = getToken();
        const params = {
            method: "DELETE",
            headers: {
                'Content-Type': 'application/json',
                'Authorization': token
            },
        };
        
        const response = await fetch(url, params);
        
        if (!response.ok) {
            if (response.status == 401){
                const ref_response = await this.refresh_token();
                if (!ref_response.ok) {
                    throw new FetchError("UnAuth", 401)
                }
                return await this.delete(endpoint);
            };
            throw new FetchError("", response.status);
        };

        return response;
    };

    async put(endpoint: string, body?: any): Promise<Response> {
        const url = this.prefix() + endpoint;
        const token = getToken();
        const params = {
            method: "PUTT",
            headers: {
                'Content-Type': 'application/json',
                'Authorization': token
            },
            body: JSON.stringify(body)
        };

        const response = await fetch(url, params);
        
        if (!response.ok) {
            if (response.status == 401){
                const ref_response = await this.refresh_token();
                if (!ref_response.ok) {
                    throw new FetchError("UnAuth", 401)
                }
                return await this.put(endpoint, body);
            };
            throw new FetchError("", response.status);
        };

        return response;
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
}


export const client = new EzAutherClient("localhost", 8080);
