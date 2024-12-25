export type AuthInfo = {
    name: string,
    pass: string
};

export type ItemInfo = {
    name: string,
    id: string
};


export type LoginResponse = {
    id: string,
    token: string,
    refresh_token: string
}