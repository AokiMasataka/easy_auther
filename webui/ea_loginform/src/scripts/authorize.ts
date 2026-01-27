import { BACKEND_BASE_URL } from "./const.ts";


export async function authorizeApi(
    email: string,
    password: string,
    clientId: string,
    redirectUri: string,
    codeChallenge: string,
): Promise<string> {
    const url = BACKEND_BASE_URL + "/authorize";
    const headers = {"Content-Type": "application/json"}
    const body = {
        email: email,
        password: password,
        client_id: clientId,
        redirect_uri: redirectUri,
        code_challenge: codeChallenge
    };
    const response = await fetch(
        url,
        {
            method: "POST",
            headers: headers,
            body: JSON.stringify(body),
        }
    );
    const json = await response.json();
    const authorization_code = json["authorization_code"];
    return authorization_code;
}


