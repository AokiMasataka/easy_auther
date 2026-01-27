export async function redirectLoginForm(
    redirect_uri: string,
    client_id: string
): Promise<void> {
    const { codeVerifier, codeChallenge } = await generatePkce();
    const state = generateState();

    const redirectUrl = new URL(redirect_uri);
    redirectUrl.searchParams.append("client_id", client_id);
    redirectUrl.searchParams.append("redirect_uri", `http://${globalThis.window.location.host}/callback`);
    redirectUrl.searchParams.append("code_challenge", codeChallenge);
    redirectUrl.searchParams.append("state", state);

    setCodeVerifier(codeVerifier);
    setOauthState(state);

    globalThis.window.location.href = redirectUrl.toString();
}


export async function callback(ea_api_base_url: string): Promise<string> {
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

    const jwt = await tokenApi(callbackCode, codeVerifier, ea_api_base_url);
    
    clearCodeVerifier();
    clearOauthState();
    return jwt;
}


async function tokenApi(
    authorization_code: string,
    code_verifier: string,
    ea_api_base_url: string
): Promise<string> {
    const url = new URL("/token", ea_api_base_url).toString();
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

async function generatePkce(): Promise<{
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


function generateState(length = 32): string {
  const bytes = crypto.getRandomValues(new Uint8Array(length));
  return base64UrlEncode(bytes);
}


function setCodeVerifier(verifier: string) {
    sessionStorage.setItem("pkce_verifier", verifier);
}

function getCodeVerifier(): string {
    const codeVerifier = sessionStorage.getItem("pkce_verifier");
    if (!codeVerifier) {
        throw new Error("code_verifier is missing");
    }
    return codeVerifier;
}

function clearCodeVerifier() {
    sessionStorage.removeItem("pkce_verifier");
}


function setOauthState(state: string) {
    sessionStorage.setItem("oauth_state", state);
}

function getOauthState(): string {
    const state = sessionStorage.getItem("oauth_state");
    if (!state) {
        throw new Error("oauth_state is missing");
    }
    return state;
}

function clearOauthState() {
    sessionStorage.removeItem("oauth_state");
}