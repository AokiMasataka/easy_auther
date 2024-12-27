import { getToken } from "../cookie.ts";


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
                'Authorization': token
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

    async post(endpoint: string, body: any): Promise<Response> {
        const url = this.prefix() + endpoint;
        const token = getToken();
        const request = {
            method: "POST",
            headers: {
                'Content-Type': 'application/json',
                'Authorization': token
            },
            body: JSON.stringify(body)
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

    async delete(endpoint: string): Promise<Response> {
        const url = this.prefix() + endpoint;
        const token = getToken();
        const request = {
            method: "DELETE",
            headers: {
                'Content-Type': 'application/json',
                'Authorization': token
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
}


export const client = new EzAutherClient("localhost", 8080);