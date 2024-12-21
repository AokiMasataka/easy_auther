import { ROOT } from "./const";
import { GroupInfo, UserInfo } from "./types";

//TODO https://zenn.dev/junki555/articles/4ab67fc78ce64c

export namespace LoginApi {
    export async function login(
        name: string,
        pass: string
    ): Promise<{
        id: string,
        token: string,
        refresh_tokrn: string
    }> {
        const response = await fetch(
            `${ROOT}/login`,
            {
                method: "POST",
                headers: {"content-type": "application/json"},
                body: JSON.stringify({name: name, pass: pass})
            }
        );
        return await response.json();
    }
}

export namespace groupApi {
    export async function createGroup(
        name: string,
        pass: string
    ): Promise<{
        id: string,
        pem: string
    }> {
        const response = await fetch(
            `${ROOT}/group`,
            {
                method: "POST",
                headers: {"content-type": "application/json"},
                body: JSON.stringify({name: name, pass: pass})
            }
        );
        return await response.json();
    };


    export async function getGroups(): Promise<GroupInfo[]> {
        const response = await fetch(
            `${ROOT}/groups`,
            {
                method: "GET",
                headers: {"content-type": "application/json"}
            }
        );
        return await response.json();
    };


    export async function deleteGroup(groupId: string) {
        console.log(`delete group id: ${groupId}`);
        await fetch(
            `${ROOT}/${groupId}`,
            {
                method: "DELETE",
                headers: {"content-type": "application/json"},
            }
        );
    };

    export async function getPublicPem(groupId: string): Promise<{pem: string}> {
        const response = await fetch(
            `${ROOT}/${groupId}/pem`,
            {
                method: "GET",
                headers: {"content-type": "application/json"}
            }
        );
        return await response.json();
    }
};


export namespace userApi {
    export async function createUser(
        groupId: string,
        name: string,
        pass: string
    ) {
        const _ = await fetch(
            `${ROOT}/${groupId}/user`,
            {
                method: "POST",
                headers: {"content-type": "application/json"},
                body: JSON.stringify({
                    group_id: groupId,
                    name: name,
                    pass: pass
                })
            }
        );  
    };
    
    export async function getUsers(groupId: string): Promise<UserInfo[]> {
        const response = await fetch(
            `${ROOT}/${groupId}/users`,
            {
                method: "GET",
                headers: {"content-type": "application/json"}
            }
        );
    
        return await response.json();
    }
    
    export async function deleteUser(
        groupId: string,
        userId: string
    ) {
        console.log(`delete group id: ${userId}`);
        await fetch(
            `${ROOT}/${groupId}/${userId}`,
            {
                method: "DELETE",
                headers: {"content-type": "application/json"},
            }
        );
    };
};