export type CreateResponse = {
    id: string
};

export type LoginResponse = {
    id: string,
    jwt: string,
    refresh_jwt: string
}