export const BACKEND_BASE_URL = import.meta.env.VITE_BACKEND_API_BASE_URL;

if (!BACKEND_BASE_URL) {
    throw new Error("VITE_BACKEND_API_BASE_URL is not defined");
}