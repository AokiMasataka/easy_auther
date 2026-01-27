import { BACKEND_BASE_URL } from "./const.ts";
import {
    setCodeVerifier,
    getCodeVerifier,
    clearCodeVerifier,
    setOauthState,
    getOauthState,
    clearOauthState
} from "./pkceStore.ts";


export async function redirectLoginForm(
    client_id: string,
): Promise<void> {
    const { codeVerifier, codeChallenge } = await generatePkce();
    const state = generateState();

    const redirectUrl = new URL("http://localhost:5173/");
    redirectUrl.searchParams.append("client_id", client_id);
    redirectUrl.searchParams.append("redirect_uri", `http://${globalThis.window.location.host}/callback`);
    redirectUrl.searchParams.append("code_challenge", codeChallenge);
    redirectUrl.searchParams.append("state", state);

    setCodeVerifier(codeVerifier);
    setOauthState(state);

    globalThis.window.location.href = redirectUrl.toString();
}


export async function callback(): Promise<string> {
    const params = new URLSearchParams(globalThis.window.location.search);

    const callbackCode = params.get("code");
    const callbackState = params.get("state") || "";

    if (!callbackCode) {
        throw new Error("authorization_code is missing");
    }

    const codeVerifier = getCodeVerifier();
    const state = getOauthState();

    if (!callbackState || callbackState !== state) {
        throw new Error("Invalid state");
    }

    const jwt = await tokenApi(callbackCode, codeVerifier);
    
    clearCodeVerifier();
    clearOauthState();
    return jwt;
}


async function tokenApi(
    authorization_code: string,
    code_verifier: string
): Promise<string> {
    const url = BACKEND_BASE_URL + "/token";
    const headers = {
        "Content-Type": "application/json",
    };
    const body = {
        authorization_code: authorization_code,
        code_verifier: code_verifier
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
    return json["jwt"];  
}


function base64UrlEncode(buffer: Uint8Array): string {
  return btoa(String.fromCharCode(...buffer))
    .replace(/\+/g, "-")
    .replace(/\//g, "_")
    .replace(/=+$/, "");
}


export async function generatePkce(): Promise<{
  codeVerifier: string;
  codeChallenge: string;
}> {
  // RFC7636: 43〜128文字
  const codeVerifier = base64UrlEncode(crypto.getRandomValues(new Uint8Array(32)));

  const encoder = new TextEncoder();
  const data = encoder.encode(codeVerifier);

  const digest = await crypto.subtle.digest("SHA-256", data);
  const codeChallenge = base64UrlEncode(new Uint8Array(digest));

  return {
    codeVerifier,
    codeChallenge,
  };
}


export function generateState(length = 32): string {
  const bytes = crypto.getRandomValues(new Uint8Array(length));
  return base64UrlEncode(bytes);
}