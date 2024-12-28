export type CreateResponse = {
    id: string
};

export type LoginResponse = {
    id: string,
    token: string,
    refresh_token: string
}