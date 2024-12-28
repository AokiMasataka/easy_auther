import { ItemInfo } from '../types.ts';
import { client } from "./client.ts";
import { CreateResponse, LoginResponse } from "./schema.ts";


export async function getUsers(
    groupId: string
): Promise<ItemInfo[]>{
    const response = await client.get(`${groupId}/users`);
    const json = await response.json();
    return json["users"];
}

export async function createUser(
    groupId: string,
    name: string,
    pass: string,
): Promise<CreateResponse> {
    const response = await client.post(`${groupId}/user`, {name: name, pass: pass})
    return await response.json();
}

export async function deleteUser(
    groupId: string,
    userId: string
) {
    await client.delete(`${groupId}/${userId}`);
}

export async function login(
    groupId: string,
    name: string,
    pass: string
): Promise<LoginResponse> {
    const reponse = await client.post(`${groupId}/login`, {name: name, pass: pass});
    return await reponse.json();
}
