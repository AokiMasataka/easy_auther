import { getCookie, setCookie } from 'https://esm.sh/typescript-cookie@1.0.6';

const PREFIX = "easyAuth";

export function setToken(token: string) {
    setCookie(`${PREFIX}-token`, token);
}

export function setRefreshToken(token: string) {
    setCookie(`${PREFIX}-refreshToken`, token);
}

export function getToken(): string {
    return _getToken('token');
}

export function getRefreshToken(): string {
    return _getToken('refreshToken');
}

function _getToken (name: string): string {
    const token = getCookie(`${PREFIX}-${name}`);
    if( token == undefined) {
        console.warn("cookie: token is undefined");
        return "";
    } else {
        return token;
    }
}