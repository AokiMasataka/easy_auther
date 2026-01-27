export function setCodeVerifier(verifier: string) {
    sessionStorage.setItem("pkce_verifier", verifier);
}

export function getCodeVerifier(): string {
    const codeVerifier = sessionStorage.getItem("pkce_verifier");
    if (!codeVerifier) {
        throw new Error("code_verifier is missing");
    }
    return codeVerifier;
}

export function clearCodeVerifier() {
    sessionStorage.removeItem("pkce_verifier");
}


export function setOauthState(state: string) {
    sessionStorage.setItem("oauth_state", state);
}

export function getOauthState(): string {
    const state = sessionStorage.getItem("oauth_state");
    if (!state) {
        throw new Error("oauth_state is missing");
    }
    return state;
}

export function clearOauthState() {
    sessionStorage.removeItem("oauth_state");
}