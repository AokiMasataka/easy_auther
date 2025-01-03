import { client } from "./client.ts";
import { CreateResponse, LoginResponse } from "./schema.ts";


export async function createGroup(
    name: string, pass: string
): Promise<CreateResponse>{
    const reponse = await client.post("group", {name: name, pass: pass});
    return await reponse.json();
}

export async function deleteGroup(
    groupId: string
): Promise<void> {
    await client.delete(`group/${groupId}`);
};

export async function login(
    name: string,
    pass: string
): Promise<LoginResponse> {
    const reponse = await client.login(name, pass);
    return await reponse.json();
}
